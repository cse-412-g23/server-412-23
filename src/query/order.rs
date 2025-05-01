use async_graphql::{ComplexObject, Context, Object, Result, SimpleObject};
use chrono::NaiveDateTime;
use rust_decimal::prelude::ToPrimitive;
use sqlx::PgPool;

use crate::auth::User;
use crate::guards::login::LoginGuard;

use super::products::Product;

#[derive(SimpleObject)]
#[graphql(complex)]
pub struct Purchase {
    pub key: i32,
    pub address: String,
    pub date: NaiveDateTime,
}

#[ComplexObject]
impl Purchase {
    pub async fn items(&self, ctx: &Context<'_>) -> Result<Vec<PurchaseItem>> {
        let pool = ctx.data::<PgPool>()?;

        let items = sqlx::query!(
            "SELECT purchase_item_key, purchased_price FROM purchase_item
                WHERE purchase_key = $1;",
            self.key
        )
        .fetch_all(pool)
        .await?;

        Ok(items
            .into_iter()
            .map(|item| PurchaseItem {
                key: item.purchase_item_key,
                purchased_price: item.purchased_price.to_decimal(2).to_f32().unwrap(),
            })
            .collect())
    }
}

#[derive(SimpleObject)]
#[graphql(complex)]
pub struct PurchaseItem {
    pub key: i32,
    pub purchased_price: f32,
}

#[ComplexObject]
impl PurchaseItem {
    pub async fn product(&self, ctx: &Context<'_>) -> Result<Product> {
        let pool = ctx.data::<PgPool>()?;

        let product = sqlx::query! {
            "SELECT p.product_key,
                    p.product_name,
                    p.product_desc,
                    p.product_qty,
                    p.product_price,
                    p.product_listed
                FROM product p JOIN purchase_item pi ON pi.product_key = p.product_key
                WHERE pi.purchase_item_key = $1;",
            self.key
        }
        .fetch_one(pool)
        .await?;

        Ok(Product {
            key: product.product_key,
            name: product.product_name,
            desc: product.product_desc,
            qty: product.product_qty,
            price: product.product_price.to_decimal(2).to_f32().unwrap(),
            listed: product.product_listed,
        })
    }
}

#[derive(SimpleObject)]
#[graphql(complex)]
pub struct CartItem {
    pub key: i32,
}

#[ComplexObject]
impl CartItem {
    pub async fn product(&self, ctx: &Context<'_>) -> Result<Product> {
        let pool = ctx.data::<PgPool>()?;

        let product = sqlx::query! {
            "SELECT p.product_key,
                    p.product_name,
                    p.product_desc,
                    p.product_qty,
                    p.product_price,
                    p.product_listed
                FROM product p
                JOIN cart_item ci ON ci.product_key = p.product_key
                WHERE ci.cart_item_key = $1",
            self.key
        }
        .fetch_one(pool)
        .await?;

        Ok(Product {
            key: product.product_key,
            name: product.product_name,
            desc: product.product_desc,
            qty: product.product_qty,
            price: product.product_price.to_decimal(2).to_f32().unwrap(),
            listed: product.product_listed,
        })
    }
}

#[derive(Default)]
pub struct OrderQuery {}

#[Object]
impl OrderQuery {
    #[graphql(guard = "LoginGuard")]
    /// Retrieve a purchase (made by the logged in user) by key.
    pub async fn purchase(&self, ctx: &Context<'_>, key: i32) -> Result<Purchase> {
        let pool = ctx.data::<PgPool>()?;
        let user = ctx.data::<User>()?;

        let purchase = sqlx::query! {
            "SELECT purchase_key, purchase_address, purchase_date FROM purchase
                WHERE purchase_key = $1 AND acct_key = $2",
            key, user.key
        }
        .fetch_one(pool)
        .await?;

        Ok(Purchase {
            key: purchase.purchase_key,
            address: purchase.purchase_address,
            date: purchase.purchase_date,
        })
    }

    #[graphql(guard = "LoginGuard")]
    /// Retrieve all purchases (made by the logged in user).
    pub async fn purchases(&self, ctx: &Context<'_>) -> Result<Vec<Purchase>> {
        let pool = ctx.data::<PgPool>()?;
        let user = ctx.data::<User>()?;

        let purchases = sqlx::query! {
            "SELECT purchase_key, purchase_address, purchase_date FROM purchase
                WHERE acct_key = $1",
            user.key
        }
        .fetch_all(pool)
        .await?;

        Ok(purchases
            .into_iter()
            .map(|purchase| Purchase {
                key: purchase.purchase_key,
                address: purchase.purchase_address,
                date: purchase.purchase_date,
            })
            .collect())
    }

    #[graphql(guard = "LoginGuard")]
    /// Retrieve the logged in user's cart items.
    pub async fn cart_items(&self, ctx: &Context<'_>) -> Result<Vec<CartItem>> {
        let pool = ctx.data::<PgPool>()?;
        let user = ctx.data::<User>()?;

        let cart_items = sqlx::query! {
            "SELECT cart_item_key FROM cart_item
                WHERE acct_key = $1",
            user.key
        }
        .fetch_all(pool)
        .await?;

        Ok(cart_items
            .into_iter()
            .map(|cart_item| CartItem {
                key: cart_item.cart_item_key,
            })
            .collect())
    }
}
