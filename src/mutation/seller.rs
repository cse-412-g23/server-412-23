use async_graphql::{Context, Object, Result};
use sqlx::PgPool;

use crate::guards::role::RoleGuard;

#[derive(Default)]
pub struct SellerMutation;

#[Object]
impl SellerMutation {
    #[graphql(guard = "RoleGuard::SellerEdit")]
    /// Creates a new seller. You must have the `SellerEdit` permission to perform this action.
    pub async fn create_seller(
        &self,
        ctx: &Context<'_>,
        name: String,
        country: String,
    ) -> Result<i32> {
        let pool = ctx.data::<PgPool>()?;

        let key = sqlx::query!(
            "INSERT INTO seller (seller_country, seller_name) VALUES ($1, $2) RETURNING seller_key",
            country,
            name
        )
        .fetch_one(pool)
        .await?;

        Ok(key.seller_key)
    }
}
