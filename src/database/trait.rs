// data trait for pizza

use crate::db::Database;
use crate::models::Pizza;
use actix_web::web::Data;
use async_trait::async_trait;
use surrealdb::Error;

#[async_trait]
pub trait PizzaDataTrait {
    async fn get_all_pizzas(db: &Data<Database>) -> Option<Vec<Pizza>>;
    async fn add_pizza(db: &Data<Database>, new_pizza: Pizza) -> Option<Pizza>;
    async fn update_pizza(db: &Data<Database>, uuid: String) -> Option<Pizza>;
}
#[async_trait]
impl PizzaDataTrait for Database {}
