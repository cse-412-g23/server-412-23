use async_graphql::{Context, Guard, Result};

use crate::{auth::User, guards::role::RoleGuard};

/// This function checks if the user is the same as the provided user ID, or that the user has
/// the "acct_edit_any" permission.
///
/// This is used to ensure that only administrators and the user themselves can retrieve their
/// roles and permissions.
pub async fn self_or_acct_any(ctx: &Context<'_>, user_id: i32) -> Result<()> {
    let user = ctx.data::<User>()?;

    if user.key == user_id {
        Ok(())
    } else {
        RoleGuard::AcctEdit.check(ctx).await
    }
}
