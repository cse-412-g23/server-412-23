use async_graphql::{Context, Object, Result, SimpleObject};
use rust_decimal::prelude::ToPrimitive;
use sqlx::PgPool;

#[derive(SimpleObject)]
pub struct Product {
    pub key: i32,
    pub name: String,
    pub desc: String,
    pub price: f32,
    pub qty: i32,
    pub listed: bool,
}

#[derive(Default)]
pub struct ProductQuery;

#[Object]
impl ProductQuery {
    pub async fn product(&self, ctx: &Context<'_>, key: i32) -> Result<Product> {
        let pool = ctx.data::<PgPool>()?;

        let product = sqlx::query!(
            "SELECT
                product_key,
                product_name,
                product_desc,
                product_price,
                product_qty,
                product_listed
            FROM product WHERE product_key = $1",
            key
        )
        .fetch_one(pool)
        .await?;

        Ok(Product {
            key: product.product_key,
            name: product.product_name,
            desc: product.product_desc,
            price: product.product_price.to_decimal(2).to_f32().unwrap(),
            qty: product.product_qty,
            listed: product.product_listed,
        })
    }

    pub async fn products(&self, ctx: &Context<'_>, start: i64, amnt: i64) -> Result<Vec<Product>> {
        let pool = ctx.data::<PgPool>()?;

        let products = sqlx::query!(
            "SELECT
                product_key,
                product_name,
                product_desc,
                product_price,
                product_qty,
                product_listed
            FROM product
            WHERE product_listed = true
            ORDER BY product_key ASC
            LIMIT $1 OFFSET $2",
            amnt,
            start
        )
        .fetch_all(pool)
        .await?;

        Ok(products
            .into_iter()
            .map(|product| Product {
                key: product.product_key,
                name: product.product_name,
                desc: product.product_desc,
                price: product.product_price.to_decimal(2).to_f32().unwrap(),
                qty: product.product_qty,
                listed: product.product_listed,
            })
            .collect())
    }

    pub async fn products_search(
        &self,
        ctx: &Context<'_>,
        start: i64,
        amnt: i64,
        text_like: String,
    ) -> Result<Vec<Product>> {
        let pool = ctx.data::<PgPool>()?;

        let products = sqlx::query!(
            "SELECT
                product_key,
                product_name,
                product_desc,
                product_price,
                product_qty,
                product_listed
            FROM product
            WHERE product_listed = true AND product_name LIKE $3
            ORDER BY product_key ASC
            LIMIT $1 OFFSET $2",
            amnt,
            start,
            format!("%{}%", text_like)
        )
        .fetch_all(pool)
        .await?;

        Ok(products
            .into_iter()
            .map(|product| Product {
                key: product.product_key,
                name: product.product_name,
                desc: product.product_desc,
                price: product.product_price.to_decimal(2).to_f32().unwrap(),
                qty: product.product_qty,
                listed: product.product_listed,
            })
            .collect())
    }
}
