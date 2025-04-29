use async_graphql::{Context, Object, Result, SimpleObject};
use sqlx::PgPool;

use crate::auth::User;
use crate::guards::login::LoginGuard;

#[derive(SimpleObject)]
pub struct Account {
    key: i32,
    email: String,
    country: String,
}

#[derive(Default)]
pub struct UserQuery;

#[Object]
impl UserQuery {
    #[graphql(guard = "LoginGuard")]
    async fn me(&self, ctx: &Context<'_>) -> Result<Account> {
        let user = ctx.data::<User>()?;
        let pool = ctx.data::<PgPool>()?;

        let acct = sqlx::query!(
            "SELECT acct_email, acct_country FROM acct WHERE acct_key = $1;",
            user.key
        )
        .fetch_one(pool)
        .await?;

        Ok(Account {
            key: user.key,
            email: acct.acct_email,
            country: acct.acct_country,
        })
    }
}
