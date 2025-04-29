use async_graphql::{Context, Guard, Result};

use crate::auth::User;

pub struct LoginGuard;

impl Guard for LoginGuard {
    async fn check(&self, ctx: &Context<'_>) -> Result<()> {
        if ctx.data_opt::<User>().is_some() {
            Ok(())
        } else {
            Err("You must be logged in to perform this operation.".into())
        }
    }
}
