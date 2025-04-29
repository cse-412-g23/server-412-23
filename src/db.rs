use std::env::var;

use sqlx::{PgPool, postgres::PgConnectOptions};

pub async fn create_pool() -> PgPool {
    let options = PgConnectOptions::new()
        .host(&var("POSTGRES_HOST").expect("no host"))
        .username(&var("POSTGRES_USER").expect("no user"))
        .password(&var("POSTGRES_PASS").expect("no password"))
        .port(
            var("POSTGRES_PORT")
                .map(|v| v.parse().ok())
                .ok()
                .flatten()
                .unwrap_or(5432),
        )
        .database(&var("POSTGRES_DB").expect("no database"));

    let pool = PgPool::connect_with(options)
        .await
        .expect("failed to connect to database");

    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("failed to run migrations");

    pool
}
