#![allow(unused)]

mod config;
mod ctx;
mod database;
mod error;
mod log;
mod model;
mod models;
pub mod web;

//#[cfg(test)] //Commented during early development
pub mod _dev_utils;

use crate::log::log_request;
use crate::models::ModelController;
pub use config::config;

pub use self::error::{Error, Result};
use std::net::SocketAddr;
use std::vec;

use axum::extract::{Path, Query};
use axum::http::{Method, Uri};
use axum::response::{Html, IntoResponse, Response};
use axum::routing::{get, get_service};
use axum::{middleware, Extension, Json, Router};
use ctx::Ctx;
use rapi::run;
use serde::{Deserialize, Serialize};
use serde_json::json;
use tower_cookies::CookieManagerLayer;
use tower_http::services::ServeDir;
use tracing::{debug, info};
use tracing_subscriber::EnvFilter;
use uuid::Uuid;
/* use surrealdb::engine::remote::ws::{Ws, Wss};
use surrealdb::opt::auth::Root;
use surrealdb::sql::Thing;
use surrealdb::Surreal; */

// While exploring, remove for prod.
#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .without_time()
        .with_target(false)
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    _dev_utils::init_dev().await;

    let mc = ModelController::new().await?;

    let routes_apis = web::routes_tickets::routes(mc.clone())
        .route_layer(middleware::from_fn(web::mw_auth::mw_require_auth));

    let db = database::database::Database::intit()
        .await
        .expect("error connecting to database");

    let routes_all = Router::new()
        .merge(routes_hello())
        .merge(web::routes_login::routes())
        .nest("/api", routes_apis)
        .layer(middleware::map_response(main_response_mapper))
        .layer(middleware::from_fn_with_state(
            mc.clone(),
            web::mw_auth::mw_ctx_resolver,
        ))
        .layer(CookieManagerLayer::new())
        .layer(Extension(db))
        .fallback_service(routes_static());

    let port: u16 = 9000;
    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    info!("{:<12} - {:?}\n", "SocketAddr", port);
    axum::Server::bind(&addr)
        .serve(routes_all.into_make_service())
        .await
        .unwrap();

    /* let db = Surreal::new::<Wss>("surrealdbworld.fly.dev").await?;
    db.signin(Root {
        username: "dbadmin",
        password: "M0skwa!",
    })
    .await?; */

    Ok(())
    //run("").await;
}

async fn main_response_mapper(
    ctx: Option<Ctx>,
    uri: Uri,
    req_method: Method,
    res: Response,
) -> Response {
    debug!(" {:<12} - handler_hello ", "main_response_mapper");

    let uuid = Uuid::new_v4();
    let service_error = res.extensions().get::<Error>();
    let client_status_error = service_error.map(|se| se.client_status_and_error());

    // -- If client error, build the new reponse.
    let error_response = client_status_error
        .as_ref()
        .map(|(status_code, client_error)| {
            let client_error_body = json!({
                "error": {
                    "type": client_error.as_ref(),
                    "req_uuid": uuid.to_string(),
                }
            });

            println!("    ->> client_error_body: {client_error_body}");

            // Build the new response from the client_error_body
            (*status_code, Json(client_error_body)).into_response()
        });

    // Build and log the server log line.
    let client_error = client_status_error.unzip().1;
    // TODO: Need to hander if log_request fail (but should not fail request)
    let _ = log_request(uuid, req_method, uri, ctx, service_error, client_error).await;

    println!();
    error_response.unwrap_or(res)
}

fn routes_static() -> Router {
    debug!(" {:<12} - handler_hello ", "FALLBACK");
    Router::new().nest_service(
        "/",
        get_service(ServeDir::new(&config::config().WEB_FOLDER)),
    )
}

fn routes_hello() -> Router {
    Router::new()
        .route("/hello", get(handler_hello))
        .route("/hello2/:name", get(handler_hello2))
}

#[derive(Serialize, Deserialize, Debug)]
struct HelloParams {
    name: Option<String>,
}

async fn handler_hello(params: Query<HelloParams>) -> impl IntoResponse {
    debug!(" {:<12} - handler_hello - {params:?}", "HANDLER");
    let name = params.name.as_deref().unwrap_or("World!");
    Html("Hello <strong>{name}</strong>")
}

async fn handler_hello2(Path(name): Path<String>) -> impl IntoResponse {
    debug!(" {:<12} - handler_hello - {name:?}", "HANDLER");
    Html("Hello <strong>{name}</strong>")
}
