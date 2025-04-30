use std::collections::HashSet;

use async_graphql::{ComplexObject, Context, Enum, Object, Result, SimpleObject};
use sqlx::PgPool;

use crate::auth::User;
use crate::guards::login::LoginGuard;
use crate::guards::role::RoleGuard;
use crate::util::self_or_acct_any;

#[derive(SimpleObject)]
#[graphql(complex)]
/// A user account.
pub struct Account {
    /// The user's primary key.
    key: i32,
    /// The user's email.
    email: String,
    /// The country the user stated they reside in during registration.
    country: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Enum, Eq, Hash)]
/// An individual account permission.
pub enum Permission {
    /// Allows the user to order goods.
    Purchasing,
    /// Allows the user to edit any product.
    ProductEdit,
    /// Allows the user to edit any seller.
    SellerEdit,
    /// Allows the user to edit any account. This also allows the user to perform ancillary actions
    /// such as managing roles.
    AcctEdit,
}

#[ComplexObject]
impl Account {
    /// Returns the names of the roles of the account.
    async fn roles(&self, ctx: &Context<'_>) -> Result<Vec<String>> {
        self_or_acct_any(ctx, self.key).await?;

        let pool = ctx.data::<PgPool>()?;

        let roles = sqlx::query!(
            "SELECT r.role_name FROM role r JOIN acct_role ar ON r.role_key = ar.role_key WHERE ar.acct_key = $1;",
            self.key
        )
        .fetch_all(pool)
        .await?;

        Ok(roles.into_iter().map(|r| r.role_name).collect())
    }

    /// Returns the permissions of the account, as a list of `Permission`.
    async fn permissions(&self, ctx: &Context<'_>) -> Result<Vec<Permission>> {
        self_or_acct_any(ctx, self.key).await?;

        let pool = ctx.data::<PgPool>()?;

        let permissions = sqlx::query!(
            "SELECT r.purchasing, r.product_edit_any, r.seller_edit_any, r.acct_edit_any FROM role r JOIN acct_role ar ON r.role_key = ar.role_key WHERE ar.acct_key = $1;",
            self.key
        )
        .fetch_all(pool)
        .await?;

        let mut permission_set: HashSet<Permission> = HashSet::new();

        for permission in permissions {
            if permission.purchasing {
                permission_set.insert(Permission::Purchasing);
            }
            if permission.product_edit_any {
                permission_set.insert(Permission::ProductEdit);
            }
            if permission.seller_edit_any {
                permission_set.insert(Permission::SellerEdit);
            }
            if permission.acct_edit_any {
                permission_set.insert(Permission::AcctEdit);
            }
        }

        Ok(permission_set.into_iter().collect())
    }
}

#[derive(Default)]
pub struct UserQuery;

#[Object]
impl UserQuery {
    #[graphql(guard = "LoginGuard")]
    /// Retrieves the currently logged in user's information.
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

    #[graphql(guard = "RoleGuard::AcctEdit")]
    /// Retrieves a list of users. You must have the `AcctEdit` permission to perform this action.
    async fn user_list(&self, ctx: &Context<'_>) -> Result<Vec<Account>> {
        let pool = ctx.data::<PgPool>()?;

        let accts = sqlx::query!("SELECT acct_key, acct_email, acct_country FROM acct;")
            .fetch_all(pool)
            .await?;

        Ok(accts
            .into_iter()
            .map(|acct| Account {
                key: acct.acct_key,
                email: acct.acct_email,
                country: acct.acct_country,
            })
            .collect())
    }
}
