use surrealdb::engine::remote::ws::{Ws, Wss};
use surrealdb::opt::auth::Root;
use surrealdb::sql::Thing;
use surrealdb::Surreal;
use tracing::info;

const DB_DEV_SURREAL_URL: &str = "";
const DB_DEV_SURREAL_APP_URL: &str = "";

const DB_RECREATEDB: &str = "";
const DB_DIR: &str = "";

pub async fn init_dev_db() -> Result<(), Box<dyn std::error::Error>> {
    info!("{:<12} - {:?}\n", "SocketAddr", "test");
    todo!()

    // get sql files
}

async fn pexec() -> () {
    todo!()
}

async fn new_db_pool() -> surrealdb::Result<()> {
    let db = Surreal::new::<Ws>("127.0.0.1:8000").await?;
    Ok(())
}
