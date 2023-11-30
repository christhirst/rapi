// region:    --- Modules

mod error;

pub use self::error::{Error, Result};
use surrealdb::opt::auth::Root;
use surrealdb::{
    engine::remote::ws::{Client, Ws},
    Surreal,
};
use tracing::info;

use crate::config;

// endregion: --- Modules

pub type Db = Surreal<Client>;

pub async fn new_db_pool() -> Result<Db> {
    const DB_DEV_SURREAL_APP_URL: &str = "0.0.0.0:8000";
    // * signin
    // Signin as a namespace, database, or root user

    /* PgPoolOptions::new()
    .max_connections(max_connections)
    .connect(&config().DB_URL)
    .await
    .map_err(|ex| Error::FailToCreatePool(ex.to_string())) */

    info!("{:<12} - {:?}\n", "Trying Init db", "test");
    let db = Surreal::new::<Ws>(DB_DEV_SURREAL_APP_URL).await?;

    db.use_ns("ns").use_db("db").await?;
    Ok(db)
    //todo!()
}

// NOTE 1) This is not an ideal situation; however, with sqlx 0.7.1, when executing `cargo test`, some tests that use sqlx fail at a
//         rather low level (in the tokio scheduler). It appears to be a low-level thread/async issue, as removing/adding
//         tests causes different tests to fail. The cause remains uncertain, but setting max_connections to 1 resolves the issue.
//         The good news is that max_connections still function normally for a regular run.
//         This issue is likely due to the unique requirements unit tests impose on their execution, and therefore,
//         while not ideal, it should serve as an acceptable temporary solution.
//         It's a very challenging issue to investigate and narrow down. The alternative would have been to stick with sqlx 0.6.x, which
//         is potentially less ideal and might lead to confusion as to why we are maintaining the older version in this blueprint.
