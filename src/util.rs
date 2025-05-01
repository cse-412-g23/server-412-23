use async_graphql::{Context, Guard, Result};
use sqlx::PgPool;

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

pub async fn seller_or_edit_any(ctx: &Context<'_>, seller_id: i32) -> Result<()> {
    let user = ctx.data::<User>()?;
    let pool = ctx.data::<PgPool>()?;

    let has_role = sqlx::query!(
        "
        SELECT sr.seller_key FROM acct_role ar
            JOIN seller_admin_role sr ON sr.role_key = ar.role_key
            WHERE ar.acct_key = $1;
        ",
        user.key
    )
    .fetch_one(pool)
    .await
    .ok();

    if has_role.and_then(|v| v.seller_key) == Some(seller_id) {
        Ok(())
    } else {
        RoleGuard::SellerEdit.check(ctx).await
    }
}
