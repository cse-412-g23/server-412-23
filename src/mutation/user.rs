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

use crate::auth::User;
use crate::guards::role::RoleGuard;

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
}
