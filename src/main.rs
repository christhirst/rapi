#![allow(unused)]

use std::net::SocketAddr;

use axum::response::Html;
use axum::routing::get;
use axum::Router;
use rapi::run;
use serde::{Deserialize, Serialize};
use surrealdb::engine::remote::ws::{Ws, Wss};
use surrealdb::opt::auth::Root;
use surrealdb::sql::Thing;
use surrealdb::Surreal;

// While exploring, remove for prod.

#[tokio::main]
async fn main() -> surrealdb::Result<()> {
    let routers_hello = Router::new().route(
        "/hello",
        get(|| async { Html("Hello <strong>World</strong>") }),
    );
    let addr = SocketAddr::from(([127, 0, 0, 1], 9000));
    axum::Server::bind(&addr)
        .serve(routers_hello.into_make_service())
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
