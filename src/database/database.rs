use crate::web::routes_login::Pizza;

use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::opt::auth::Root;
use surrealdb::{Error, Surreal};

#[derive(Clone)]
pub struct Database {
    pub client: Surreal<Client>,
    pub name_space: String,
    pub db_name: String,
}

impl Database {
    pub async fn intit() -> Result<Self, Error> {
        let client = Surreal::new::<Ws>("127.0.0.1:9000").await?;
        client
            .signin(Root {
                username: "root",
                password: "root",
            })
            .await?;

        client.use_ns("surreal").use_db("pizzas").await.unwrap();
        Ok(Database {
            client,
            name_space: String::from("surreal"),
            db_name: String::from("pizzas"),
        })
    }

    pub async fn get_all(&self) -> Option<Vec<Pizza>> {
        let result = self.client.select("pizza").await;
        match result {
            Ok(all_pizzas) => Some(all_pizzas),
            Err(_) => None,
        }
    }
}
