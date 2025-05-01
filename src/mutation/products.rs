use async_graphql::{Context, Object, Result};
use rust_decimal::{Decimal, prelude::FromPrimitive};
use sqlx::{PgPool, postgres::types::PgMoney};

use crate::{guards::role::RoleGuard, util::seller_or_edit_any};

#[derive(Default)]
pub struct ProductMutation;

#[Object]
impl ProductMutation {
    /// Creates a new product with the given information. You must either be a member of a role
    /// associated with this product or have the `SellerEdit` permission.
    pub async fn create_product(
        &self,
        ctx: &Context<'_>,
        name: String,
        description: String,
        price: f32,
        qty: i32,
        sold_by: i32,
    ) -> Result<i32> {
        seller_or_edit_any(ctx, sold_by, RoleGuard::SellerEdit).await?;

        let pool = ctx.data::<PgPool>()?;

        let result = sqlx::query!(
            "INSERT INTO product (
                product_name,
                product_desc,
                product_price,
                product_qty,
                product_listed,
                seller_key)
                VALUES (
                    $1,
                    $2,
                    $3,
                    $4,
                    false,
                    $5)
                RETURNING product_key
            ",
            name,
            description,
            PgMoney::from_decimal(
                Decimal::from_f32(price).expect("couldn't convert price to decimal"),
                2
            ),
            qty,
            sold_by
        )
        .fetch_one(pool)
        .await?;

        Ok(result.product_key)
    }

    /// (Un)lists a product. You must either be a member of a role associated with this product
    /// or have the `ProductEdit` permission.
    pub async fn list_product(&self, ctx: &Context<'_>, key: i32) -> Result<bool> {
        let pool = ctx.data::<PgPool>()?;

        let product_seller =
            sqlx::query!("SELECT seller_key FROM product WHERE product_key = $1", key)
                .fetch_one(pool)
                .await?;

        seller_or_edit_any(
            ctx,
            product_seller.seller_key.expect("product has no seller"),
            RoleGuard::ProductEdit,
        )
        .await?;

        sqlx::query!(
            "UPDATE product SET product_listed = NOT product_listed WHERE product_key = $1",
            key
        )
        .execute(pool)
        .await?;

        Ok(true)
    }

    /// Adds quantity to a product. You must either be a member of a role associated with this
    /// product or have the `ProductEdit` permission.
    pub async fn add_qty(&self, ctx: &Context<'_>, id: i32, qty: i32) -> Result<bool> {
        let pool = ctx.data::<PgPool>()?;

        let product_seller =
            sqlx::query!("SELECT seller_key FROM product WHERE product_key = $1", id)
                .fetch_one(pool)
                .await?;

        seller_or_edit_any(
            ctx,
            product_seller.seller_key.expect("product has no seller"),
            RoleGuard::ProductEdit,
        )
        .await?;

        sqlx::query!(
            "UPDATE product SET product_qty = product_qty + $1 WHERE product_key = $2",
            qty,
            id
        )
        .execute(pool)
        .await?;

        Ok(true)
    }
}
