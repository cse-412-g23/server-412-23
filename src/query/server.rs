use async_graphql::{Object, SimpleObject};

#[derive(SimpleObject)]
struct ServerInfo {
    name: &'static str,
    version: &'static str,
}

#[derive(Default)]
pub struct ServerQuery;

#[Object]
impl ServerQuery {
    async fn info(&self) -> ServerInfo {
        ServerInfo {
            name: "server-412-23",
            version: "0.1.0",
        }
    }
}
