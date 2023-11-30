use crate::database::database::Database;
use crate::{Error, Result};
use axum::{routing::post, Extension, Json, Router};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use tower_cookies::{Cookie, Cookies};
use tracing::debug;
use validator::Validate;

pub fn routes() -> Router {
    Router::new().route("/api/login", post(api_login))
}
//Extension(shared_data): Extension<SharedData>
//Extension(db):Extension<Database>)
async fn api_login(
    Extension(db): Extension<Database>,
    cookies: Cookies,
    payload: Json<LoginPayload>,
) -> Result<Json<Value>> {
    /* let is_valid = payload.validate();
    match is_valid {
        Ok(_)=> let pizza_name = payload.pizza_name.clone();


    } */

    debug!(" {:<12} - handler_hello ", "api_login");
    if payload.username != "demo1" || payload.pw != "welcome" {
        return Err(Error::LoginFail);
    }

    //TODO Impl real auth token
    cookies.add(Cookie::new("auth-token", "user-1.exp.sign"));
    let body = Json(json!({"result":{
        "success":true
    }

    }));
    let pizzas = db.get_all().await;
    match pizzas {
        Some(found_pizzas) => Ok(body),
        None => Err(Error::AuthFailCtxNotInRequestExt),
    }

    // Ok(body)
}

async fn add_pizza(Extension(db): Extension<Database>, new_pizza: Pizza) -> Option<Pizza> {
    debug!(" {:<12} - handler_hello ", "api_login");

    let body = Json(json!({"result":{
        "success":true
    }

    }));
    let created_pizza = db
        .client
        .create(("pizza", new_pizza.uuid.clone()))
        .content(new_pizza)
        .await;
    match created_pizza {
        Ok(created) => created,
        Err(_) => None,
    }
}

#[derive(Deserialize, Debug, Validate)]
struct LoginPayload {
    #[validate(length(min = 1, message = "pizza name required"))]
    username: String,
    pw: String,
}

#[derive(Validate, Deserialize, Serialize, Debug)]
pub struct Pizza {
    pub uuid: String,
    pub name: String,
}

impl Pizza {
    pub fn new(uuid: String, name: String) -> Pizza {
        Pizza { uuid, name }
    }
}
