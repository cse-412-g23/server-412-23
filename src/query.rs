use async_graphql::MergedObject;
use server::ServerQuery;
use user::UserQuery;

mod server;
mod user;

#[derive(MergedObject, Default)]
pub struct Query(ServerQuery, UserQuery);
