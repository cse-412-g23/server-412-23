use async_graphql::{Object, SimpleObject};

#[derive(SimpleObject)]
/// Basic information about the server, to be used to verify the connection.
struct ServerInfo {
    /// The name of the server software.
    name: &'static str,
    /// The server software's version number.
    version: &'static str,
}

#[derive(Default)]
pub struct ServerQuery;

#[Object]
impl ServerQuery {
    /// Retrieve the ServerInfo for the current server version.
    async fn info(&self) -> ServerInfo {
        ServerInfo {
            name: "server-412-23",
            version: "0.2.0",
        }
    }
}
