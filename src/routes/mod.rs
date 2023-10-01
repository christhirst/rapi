mod hello_world;
mod m_string;
mod mirror_json;

use axum::{
    body::Body,
    http::Method,
    middleware::{self},
    routing::{get, post},
    Extension, Router,
};
use dotenvy::dotenv;
use std::env;

use tower_http::cors::{Any, CorsLayer};
use surrealdb::engine::local::Db;
use surrealdb::Surreal;
use hello_world::root;
use m_string::m_body_string;
use mirror_json::*;

#[derive(Clone)]
pub struct SharedData {
    pub message: String,
}

pub fn create_routes(database: Surreal<Db>) -> Router<(), Body> {
    dotenv().expect(".env file not found");
    let env = env::var("Z").unwrap().to_owned();
    println!("{}", env);

    let shared_data = SharedData {
        message: "Message from shared data".to_owned(),
    };
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_origin(Any);

    Router::new()
        .route("/ss", get(read_middleware))
        .route_layer(middleware::from_fn(set_middleware))
        .route("/", get(root))
        .route("/test", post(m_body_string))
        .route("/test2", post(mirror_json_body))
        .route("/test1/:id", get(path_var_body))
        .route("/params", get(path_params))
        .route("/agent", get(m_user_agent))
        .route("/tt", get(middleware_message))
        .route("/qq", get(return_201))
        .layer(cors)
        .layer(Extension(shared_data))
        .route("/ae", get(always_errors))
        .route("/mod_header", get(m_header))        
        .route("/get_json", get(return_json))    
        //.route("/post_json", post(validate_json))
        .route("/create_task", post(create_task))
        .layer(Extension(database))
}
