use async_graphql::MergedObject;
use order::OrderQuery;
use seller::SellerQuery;
use server::ServerQuery;
use user::UserQuery;

mod order;
mod products;
mod seller;
mod server;
pub mod user;

#[derive(MergedObject, Default)]
pub struct Query(ServerQuery, UserQuery, SellerQuery, OrderQuery);
