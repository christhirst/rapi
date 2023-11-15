mod dev_db;
mod dev_db_test;

use std::iter::Once;

use tokio::sync::OnceCell;
use tracing::info;

// init for development 
pub async fn init_dev() {
    static INIT: OnceCell<()> = OnceCell::const_new();

    INIT.get_or_init(|| async {
        info!("{:<12} - {:?}\n", "SocketAddr", "test");
        dev_db::init_dev_db().await.unwrap();
    })
    .await;
}
