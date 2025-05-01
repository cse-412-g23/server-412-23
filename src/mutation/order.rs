use async_graphql::{Context, Object, Result};

#[derive(Default)]
pub struct OrderMutation;

#[Object]
impl OrderMutation {
    pub async fn add_to_cart(&self, ctx: &Context<'_>, product: i32) -> Result<i32> {
        todo!()
    }

    pub async fn remove_from_cart(&self, ctx: &Context<'_>, cart_item: i32) -> Result<i32> {
        todo!()
    }

    pub async fn place_order(&self, ctx: &Context<'_>, address: String) -> Result<i32> {
        todo!()
    }
}
