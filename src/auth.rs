use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct User {
    /// The users's account primary key.
    pub key: i32,
}
