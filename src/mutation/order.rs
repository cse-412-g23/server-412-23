use async_graphql::{Context, Object, Result};
use chrono::Utc;
use sqlx::PgPool;

use crate::{auth::User, guards::role::RoleGuard};

#[derive(Default)]
pub struct OrderMutation;

#[Object]
impl OrderMutation {
    #[graphql(guard = "RoleGuard::Purchasing")]
    /// Adds an item to the cart. You must have the `Purchasing` permission to perform this action.
    pub async fn add_to_cart(&self, ctx: &Context<'_>, product: i32) -> Result<i32> {
        let pool = ctx.data::<PgPool>().unwrap();
        let user = ctx.data::<User>().unwrap();

        let result = sqlx::query!(
            "INSERT INTO cart_item (acct_key, product_key)
                VALUES ($1, $2)
                RETURNING cart_item_key;",
            user.key,
            product
        )
        .fetch_one(pool)
        .await?;

        Ok(result.cart_item_key)
    }

    #[graphql(guard = "RoleGuard::Purchasing")]
    /// Removes an item from the cart. You must have the `Purchasing` permission to perform this
    /// action.
    pub async fn remove_from_cart(&self, ctx: &Context<'_>, cart_item: i32) -> Result<bool> {
        let pool = ctx.data::<PgPool>().unwrap();
        let user = ctx.data::<User>().unwrap();

        sqlx::query!(
            "DELETE FROM cart_item WHERE cart_item_key = $1 AND acct_key = $2;",
            cart_item,
            user.key
        )
        .execute(pool)
        .await?;

        Ok(true)
    }

    #[graphql(guard = "RoleGuard::Purchasing")]
    /// Places an order, removing all items from the cart, and reducing the quantity of purchased
    /// items. You must have the `Purchasing` permission to perform this action.
    pub async fn place_order(&self, ctx: &Context<'_>, address: String) -> Result<i32> {
        let pool = ctx.data::<PgPool>().unwrap();
        let user = ctx.data::<User>().unwrap();

        let purchase = sqlx::query!(
            "INSERT INTO purchase (acct_key, purchase_address, purchase_date)
                VALUES ($1, $2, $3)
                RETURNING purchase_key;",
            user.key,
            address,
            Utc::now().naive_utc()
        )
        .fetch_one(pool)
        .await?;

        let cart_items = sqlx::query!(
            "SELECT p.product_key, p.product_price FROM cart_item ci
                JOIN product p ON ci.product_key = p.product_key
                WHERE acct_key = $1;",
            user.key
        )
        .fetch_all(pool)
        .await?;

        let mut transaction = pool.begin().await?;

        for item in cart_items {
            sqlx::query!(
                "INSERT INTO purchase_item (purchase_key, product_key, purchased_price)
                    VALUES ($1, $2, $3);",
                purchase.purchase_key,
                item.product_key,
                item.product_price
            )
            .execute(&mut *transaction)
            .await?;

            sqlx::query!(
                "UPDATE product SET product_qty = product_qty - 1 WHERE product_key = $1;",
                item.product_key
            )
            .execute(&mut *transaction)
            .await?;
        }

        sqlx::query!("DELETE FROM cart_item WHERE acct_key = $1;", user.key)
            .execute(&mut *transaction)
            .await?;

        transaction.commit().await?;

        Ok(purchase.purchase_key)
    }
}
