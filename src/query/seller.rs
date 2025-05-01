use async_graphql::{Context, Object, Result, SimpleObject};
use sqlx::PgPool;

#[derive(SimpleObject)]
pub struct Seller {
    pub key: i32,
    pub name: String,
    pub country: String,
}

#[derive(Default)]
pub struct SellerQuery;

#[Object]
impl SellerQuery {
    /// Retrieve a seller by key.
    pub async fn seller(&self, ctx: &Context<'_>, key: i32) -> Result<Seller> {
        let pool = ctx.data::<PgPool>()?;

        let data = sqlx::query!(
            "SELECT seller_key, seller_name, seller_country FROM seller WHERE seller_key = $1",
            key
        )
        .fetch_one(pool)
        .await?;

        Ok(Seller {
            key: data.seller_key,
            name: data.seller_name,
            country: data.seller_country,
        })
    }

    /// Retrieve all sellers.
    pub async fn sellers(&self, ctx: &Context<'_>) -> Result<Vec<Seller>> {
        let pool = ctx.data::<PgPool>()?;

        let data = sqlx::query!("SELECT seller_key, seller_name, seller_country FROM seller")
            .fetch_all(pool)
            .await?;

        Ok(data
            .into_iter()
            .map(|row| Seller {
                key: row.seller_key,
                name: row.seller_name,
                country: row.seller_country,
            })
            .collect())
    }
}
