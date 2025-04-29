use async_graphql::{Context, Guard, Result};
use sqlx::PgPool;

use crate::auth::User;

#[derive(Clone, Copy)]
pub enum RoleGuard {
    Purchasing,
    ProductEdit,
    SellerEdit,
    AcctEdit,
}

impl Guard for RoleGuard {
    async fn check(&self, ctx: &Context<'_>) -> Result<()> {
        if ctx.data_opt::<User>().is_some() {
            let user = ctx.data::<User>()?;
            let pool = ctx.data::<PgPool>()?;

            let results = sqlx::query!(
                "
                    SELECT
                        r.purchasing,
                        r.product_edit_any,
                        r.seller_edit_any,
                        r.acct_edit_any
                    FROM acct a
                    JOIN acct_role ar ON ar.acct_key = a.acct_key
                    JOIN role r ON r.role_key = ar.role_key
                    WHERE a.acct_key = $1
                ",
                user.key
            )
            .fetch_all(pool)
            .await?;

            let found = results.iter().find(|v| match self {
                RoleGuard::Purchasing => v.purchasing,
                RoleGuard::ProductEdit => v.product_edit_any,
                RoleGuard::SellerEdit => v.seller_edit_any,
                RoleGuard::AcctEdit => v.acct_edit_any,
            });

            found
                .ok_or_else(|| "You do not have permission to perform this operation.".into())
                .map(|_| ())
        } else {
            Err("You must be logged in to perform this operation.".into())
        }
    }
}
