use axum::{routing::post, Json, Router};
use serde::Deserialize;
use serde_json::{json, Value};
use tower_cookies::{Cookie, Cookies};
use tracing::debug;

use crate::{Error, Result};

pub fn routes() -> Router {
    Router::new().route("/api/login", post(api_login))
}

async fn api_login(cookies: Cookies, payload: Json<LoginPayload>) -> Result<Json<Value>> {
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

    Ok(body)
}

#[derive(Deserialize, Debug)]
struct LoginPayload {
    username: String,
    pw: String,
}
