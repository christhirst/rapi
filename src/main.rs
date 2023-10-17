#![allow(unused)]

use rapi::run;

// While exploring, remove for prod.

#[tokio::main]
async fn main() -> surrealdb::Result<()> {
    let db = Surreal::new::<Ws>("https://surrealdbworld.fly.dev:8080").await?;

    // Signin as a namespace, database, or root user
    db.signin(Root {
        username: "root",
        password: "root",
    })
    .await?;

    // Select a specific namespace / database
    db.use_ns("test").use_db("test").await;

    //run("").await;
}
