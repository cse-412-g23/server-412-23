use std::env::var;

use actix_cors::Cors;
use actix_web::{
    App, HttpRequest, HttpResponse, HttpServer, guard,
    http::header::HeaderMap,
    web::{self, resource},
};
use async_graphql::{EmptySubscription, Schema, http::GraphiQLSource};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};
use auth::User;
use db::create_pool;
use hmac::{Hmac, digest::KeyInit};
use jwt::VerifyWithKey;
use mutation::Mutation;
use query::Query;
use sha2::Sha256;

mod auth;
mod db;
mod guards;
mod mutation;
mod query;
mod util;

pub type ThisSchema = Schema<Query, Mutation, EmptySubscription>;

async fn index_graphiql() -> actix_web::Result<HttpResponse> {
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(GraphiQLSource::build().endpoint("/").finish()))
}

fn get_user_from_headers(headers: &HeaderMap) -> Option<User> {
    headers.get("Authorization").and_then(|v| {
        let header = v.to_str().ok()?;

        if header.starts_with("Bearer ") {
            let token = &header[7..header.len()];

            let key: Hmac<Sha256> =
                Hmac::new_from_slice(&var("JWT_SECRET").ok()?.into_bytes()).ok()?;
            token.verify_with_key(&key).ok()?
        } else {
            None
        }
    })
}

async fn index(
    schema: web::Data<ThisSchema>,
    req: HttpRequest,
    gql_request: GraphQLRequest,
) -> GraphQLResponse {
    let mut request = gql_request.into_inner();
    if let Some(user) = get_user_from_headers(req.headers()) {
        request = request.data(user);
    }
    schema.execute(request).await.into()
}

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    dotenvy::dotenv().expect("couldn't load env file");

    let pool = create_pool().await;

    HttpServer::new(move || {
        let schema = Schema::build(Query::default(), Mutation::default(), EmptySubscription)
            .data(pool.clone())
            .finish();

        let cors = Cors::permissive();

        App::new()
            .wrap(cors)
            .app_data(web::Data::new(schema.clone()))
            .service(resource("/").guard(guard::Post()).to(index))
            .service(resource("/").guard(guard::Get()).to(index_graphiql))
    })
    .bind("127.0.0.1:7123")?
    .run()
    .await
}
