use async_graphql::SimpleObject;

#[derive(SimpleObject)]
pub struct Product {
    pub key: i32,
    pub name: String,
    pub desc: String,
    pub price: f32,
    pub qty: i32,
    pub listed: bool,
}
