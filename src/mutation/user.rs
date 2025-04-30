use std::env::var;

use argon2::{
    Argon2, PasswordHash, PasswordVerifier,
    password_hash::{PasswordHasher, SaltString, rand_core::OsRng},
};
use async_graphql::{Context, Object, Result};
use hmac::{Hmac, digest::KeyInit};
use jwt::SignWithKey;
use sha2::Sha256;
use sqlx::PgPool;

use crate::guards::role::RoleGuard;
use crate::{auth::User, query::user::Permission};

#[derive(Default)]
pub struct UserMutation;

#[Object]
impl UserMutation {
    /// Creates a new user account with the specified information.
    pub async fn create_user(
        &self,
        ctx: &Context<'_>,
        email: String,
        country: String,
        password: String,
    ) -> Result<bool> {
        let pool = ctx.data::<PgPool>()?;

        let email_check = sqlx::query!("SELECT FROM acct WHERE acct_email = $1", email)
            .fetch_one(pool)
            .await
            .ok();

        if email_check.is_some() {
            return Err("Email is already used.".into());
        }

        // Implement user creation logic here
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        let hash = argon2
            .hash_password(password.as_bytes(), &salt)
            .expect("failed to hash password") // apparently this doesn't implement error?
            .to_string();

        sqlx::query!(
            "WITH ROWS AS (INSERT INTO acct (acct_email, acct_country, acct_password)
                VALUES ($1, $2, $3)
                RETURNING acct_key)
            INSERT INTO acct_role (acct_key, role_key)
                SELECT acct_key, 1
                FROM rows;",
            email,
            country,
            hash
        )
        .execute(pool)
        .await?;

        Ok(true)
    }

    /// Creates a JWT token for a given user, if the email and password combination are valid.
    /// The token must be presented in the form of a Bearer Authorization header, only for
    /// authenticated requests.
    pub async fn login_user(
        &self,
        ctx: &Context<'_>,
        email: String,
        password: String,
    ) -> Result<String> {
        let pool = ctx.data::<PgPool>()?;

        let acct = match sqlx::query!(
            "SELECT acct_key, acct_password FROM acct WHERE acct_email = $1",
            email
        )
        .fetch_one(pool)
        .await
        {
            Ok(acct) => acct,
            Err(sqlx::Error::RowNotFound) => {
                return Err("Invalid email or password.".into());
            }
            Err(e) => return anyhow::Result::Err(e.into()),
        };

        let parsed = PasswordHash::new(&acct.acct_password).expect("Unable to parse password hash");
        Argon2::default()
            .verify_password(password.as_bytes(), &parsed)
            .map_err(|_| anyhow::anyhow!("Invalid email or password."))?;

        let auth_jwt_obj = User { key: acct.acct_key };
        let key: Hmac<Sha256> = Hmac::new_from_slice(&var("JWT_SECRET")?.into_bytes())?;
        let token = auth_jwt_obj.sign_with_key(&key)?;

        Ok(token)
    }

    #[graphql(guard = "RoleGuard::AcctEdit")]
    /// Adds a role to a given account. You must have the `AcctEdit` permission to perform this
    /// action.
    pub async fn add_role(&self, ctx: &Context<'_>, acct_key: i32, role_key: i32) -> Result<bool> {
        let pool = ctx.data::<PgPool>()?;

        let exists = sqlx::query!(
            "SELECT acct_key FROM acct_role WHERE acct_key = $1 AND role_key = $2",
            acct_key,
            role_key
        )
        .fetch_one(pool)
        .await
        .ok();

        if exists.is_some() {
            return Err("User already has this role.".into());
        }

        sqlx::query!(
            "INSERT INTO acct_role (acct_key, role_key) VALUES ($1, $2)",
            acct_key,
            role_key
        )
        .execute(pool)
        .await?;

        Ok(true)
    }

    /// Removes a role from a given account. You must have the `AcctEdit` permission to perform
    /// this action.
    #[graphql(guard = "RoleGuard::AcctEdit")]
    pub async fn remove_role(
        &self,
        ctx: &Context<'_>,
        acct_key: i32,
        role_key: i32,
    ) -> Result<bool> {
        let pool = ctx.data::<PgPool>()?;

        let exists = sqlx::query!(
            "SELECT acct_key FROM acct_role WHERE acct_key = $1 AND role_key = $2",
            acct_key,
            role_key
        )
        .fetch_one(pool)
        .await
        .ok();

        if exists.is_none() {
            return Err("User does not have this role.".into());
        }

        sqlx::query!(
            "DELETE FROM acct_role WHERE acct_key = $1 AND role_key = $2",
            acct_key,
            role_key
        )
        .execute(pool)
        .await?;

        Ok(true)
    }

    /// Create a new role with no permissions. You must have the `AcctEdit` permission to perform
    /// this action.
    #[graphql(guard = "RoleGuard::AcctEdit")]
    pub async fn create_role(&self, ctx: &Context<'_>, name: String) -> Result<i32> {
        let pool = ctx.data::<PgPool>()?;

        let exists = sqlx::query!("SELECT role_name FROM role WHERE role_name = $1", name)
            .fetch_one(pool)
            .await
            .ok();

        if exists.is_some() {
            return Err("Role already exists.".into());
        }

        let result = sqlx::query!(
            "INSERT INTO role (
                role_name,
                purchasing,
                product_edit_any,
                seller_edit_any,
                acct_edit_any)
                VALUES
                    ($1, false, false, false, false)
                RETURNING role_key
            ",
            name
        )
        .fetch_one(pool)
        .await?;

        Ok(result.role_key)
    }

    /// Delete a role. You must have the `AcctEdit` permission to perform this action.
    #[graphql(guard = "RoleGuard::AcctEdit")]
    pub async fn delete_role(&self, ctx: &Context<'_>, role_key: i32) -> Result<bool> {
        let pool = ctx.data::<PgPool>()?;

        let exists = sqlx::query!("SELECT role_key FROM role WHERE role_key = $1", role_key)
            .fetch_one(pool)
            .await
            .ok();

        if exists.is_none() {
            return Err("Role does not exist.".into());
        }

        sqlx::query!("DELETE FROM role WHERE role_key = $1", role_key)
            .execute(pool)
            .await?;

        Ok(true)
    }

    /// Grant/deny a role a permission. You must have the `AcctEdit` permission to perform this
    /// action.
    #[graphql(guard = "RoleGuard::AcctEdit")]
    pub async fn toggle_permission(
        &self,
        ctx: &Context<'_>,
        role_key: i32,
        permission: Permission,
    ) -> Result<bool> {
        let pool = ctx.data::<PgPool>()?;

        let exists = sqlx::query!("SELECT role_key FROM role WHERE role_key = $1", role_key)
            .fetch_one(pool)
            .await
            .ok();

        if exists.is_none() {
            return Err("Role does not exist.".into());
        }

        match permission {
            Permission::Purchasing => {
                sqlx::query!(
                    "UPDATE role SET purchasing = NOT purchasing WHERE role_key = $1",
                    role_key
                )
            }
            Permission::ProductEdit => {
                sqlx::query!(
                    "UPDATE role SET product_edit_any = NOT product_edit_any WHERE role_key = $1",
                    role_key
                )
            }
            Permission::SellerEdit => {
                sqlx::query!(
                    "UPDATE role SET seller_edit_any = NOT seller_edit_any WHERE role_key = $1",
                    role_key
                )
            }
            Permission::AcctEdit => {
                sqlx::query!(
                    "UPDATE role SET acct_edit_any = NOT acct_edit_any WHERE role_key = $1",
                    role_key
                )
            }
        }
        .execute(pool)
        .await?;

        Ok(true)
    }

    /// Connect a role to a seller, allowing them to update products owned by that seller. You must
    /// have the `AcctEdit` permission to perform this action.
    #[graphql(guard = "RoleGuard::AcctEdit")]
    pub async fn connect_role_to_seller(
        &self,
        ctx: &Context<'_>,
        role_key: i32,
        seller_key: i32,
    ) -> Result<bool> {
        let pool = ctx.data::<PgPool>()?;

        let exists = sqlx::query!("SELECT role_key FROM role WHERE role_key = $1", role_key)
            .fetch_one(pool)
            .await
            .ok();

        if exists.is_none() {
            return Err("Role does not exist.".into());
        }

        let seller_exists = sqlx::query!(
            "SELECT seller_key FROM seller WHERE seller_key = $1",
            seller_key
        )
        .fetch_one(pool)
        .await
        .ok();

        if seller_exists.is_none() {
            return Err("Seller does not exist.".into());
        }

        sqlx::query!(
            "INSERT INTO seller_admin_role (role_key, seller_key) VALUES ($1, $2)",
            role_key,
            seller_key
        )
        .execute(pool)
        .await?;

        Ok(true)
    }
}
