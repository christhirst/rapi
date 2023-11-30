mod dev_db;
mod dev_db_test;
use crate::ctx::Ctx;
use crate::model::task::{Task, TaskBmc, TaskForCreate};
use crate::model::{self, ModelManager};
use std::iter::Once;

use serde_json::Number;
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

/// Initialize test environment.
pub async fn init_test() -> ModelManager {
    static INIT: OnceCell<ModelManager> = OnceCell::const_new();

    let mm = INIT
        .get_or_init(|| async {
            init_dev().await;
            ModelManager::new().await.unwrap()
        })
        .await;

    mm.clone()
}

pub async fn seed_tasks(ctx: &Ctx, mm: &ModelManager, titles: &[&str]) -> model::Result<Vec<Task>> {
    let mut tasks = Vec::new();

    for title in titles {
        let resource = (String::from("value"), String::from("value"));
        let id = TaskBmc::create(
            ctx,
            mm,
            resource,
            TaskForCreate {
                title: title.to_string(),
            },
        )
        .await?;

        let mut num: u64 = id.population_by_month;
        //.id.id.to_raw().parse::<i64>().ok().unwrap();

        /* if let Some(i) = id.get(0) {
            let ee = i;
            let oo: &surrealdb::sql::Id = (&ee.id.id);
            let ii = oo.clone();
            if let surrealdb::sql::Id::Number(num) = ii {
                println!("value: {}", num);
            }
        }; */
        let resource = (String::from("value"), String::from("value1"));
        let task = TaskBmc::get(ctx, mm, resource).await?;

        tasks.push(task);
    }

    Ok(tasks)
}
