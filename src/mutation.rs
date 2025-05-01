use async_graphql::MergedObject;
use order::OrderMutation;
use products::ProductMutation;
use seller::SellerMutation;
use user::UserMutation;

mod order;
mod products;
mod seller;
mod user;

#[derive(MergedObject, Default)]
pub struct Mutation(UserMutation, OrderMutation, ProductMutation, SellerMutation);
