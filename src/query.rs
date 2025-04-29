use async_graphql::MergedObject;
use server::ServerQuery;

mod server;
mod user;

#[derive(MergedObject, Default)]
pub struct Query(ServerQuery);
