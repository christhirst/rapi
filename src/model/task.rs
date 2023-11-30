use crate::ctx::Ctx;
use crate::model::base::{self, DbBmc};
use crate::model::country::month;
use crate::model::country::Country_month;
use crate::model::ModelManager;
use crate::model::Result;

use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;
// region:    --- Task Types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: i64,
    pub title: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tasks {
    //#[allow(dead_code)]
    pub title: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Record {
    #[allow(dead_code)]
    pub id: Thing,
    pub title: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TaskForCreate {
    pub title: String,
}

#[derive(Deserialize)]
pub struct TaskForUpdate {
    pub title: Option<String>,
}
// endregion: --- Task Types

// region:    --- TaskBmc
pub struct TaskBmc;

impl DbBmc for TaskBmc {
    const TABLE: &'static str = "task";
}

impl TaskBmc {
    pub async fn create<D: std::fmt::Debug + serde::Serialize>(
        ctx: &Ctx,
        mm: &ModelManager,
        resource: (String, String),
        task_c: D,
    ) -> Result<Country_month> {
        println!("{:?}", task_c);
        base::create::<Self, _>(ctx, mm, resource, task_c).await
    }

    pub async fn get(ctx: &Ctx, mm: &ModelManager, (resource): (String, String)) -> Result<Task> {
        //let pp: Result<Task> = base::get::<Self>(ctx, mm, resource, id).await;
        //pp

        Ok(Task {
            id: 2,
            title: "".to_owned(),
        })
    }

    pub async fn list(ctx: &Ctx, mm: &ModelManager) -> Result<Vec<Task>> {
        base::list::<Self, _>(ctx, mm).await
    }

    pub async fn update(
        ctx: &Ctx,
        mm: &ModelManager,
        id: i64,
        task_u: TaskForUpdate,
    ) -> Result<()> {
        base::update::<Self, _>(ctx, mm, id, task_u).await
    }

    pub async fn delete(
        ctx: &Ctx,
        mm: &ModelManager,
        resource: (String, String),
    ) -> Result<Country_month> {
        base::delete(ctx, mm, resource).await
    }
}
// endregion: --- TaskBmc

// region:    --- Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::_dev_utils;
    use crate::model::Error;
    use anyhow::Result;
    use serial_test::serial;
    use tower_cookies::cookie::time::Month;

    #[serial]
    #[tokio::test]
    async fn test_delete_err_not_found() -> Result<()> {
        // -- Setup & Fixtures
        let mm = _dev_utils::init_test().await;
        let ctx = Ctx::root_ctx();
        let resource = (String::from("value"), String::from("value1"));
        // -- Exec

        let res = TaskBmc::delete(&ctx, &mm, resource).await;
        println!("{:?}", res);

        // -- Check
        assert!(
            matches!(
                res,
                Err(Error::EntityNotFound {
                    entity: "task",
                    id: 100
                })
            ),
            "EntityNotFound not matching"
        );

        Ok(())
    }

    #[serial]
    #[tokio::test]
    async fn test_create_ok() -> Result<()> {
        // -- Setup & Fixtures
        let mm = _dev_utils::init_test().await;
        let ctx = Ctx::root_ctx();
        let fx_title = "XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXx";

        // -- Exec
        /* let task_c = TaskForCreate {
            title: fx_title.to_string(),
        }; */
        let oo: Country_month = Country_month {
            //title: String::from("test33"),
            population_by_month: 2,
            month: month::April,
        };
        println!("{:?}", fx_title);
        let resource = (String::from("value"), String::from("value12"));
        let resp: Country_month = TaskBmc::create(&ctx, &mm, resource.clone(), oo).await?;
        /* //let oo:
        println!("{:?}", fx_title);

        let i = resp.id.id;
        println!("{:?}", i);
        // -- Check
        println!("{:?}", i);

        let task: Task = TaskBmc::get(&ctx, &mm, resource.clone(), 2).await?;
        println!("{:?}", fx_title);
        assert_eq!(task.title, fx_title);
        let id = String::from(""); */
        // -- Clean
        TaskBmc::delete(&ctx, &mm, resource).await?;

        Ok(())
    }

    #[serial]
    #[tokio::test]
    async fn test_get_err_not_found() -> Result<()> {
        // -- Setup & Fixtures
        let mm = _dev_utils::init_test().await;
        let ctx = Ctx::root_ctx();
        let fx_id = 100;
        let resource = (String::from("value"), String::from("value1"));
        // -- Exec
        let res = TaskBmc::get(&ctx, &mm, resource).await;

        // -- Check
        assert!(
            matches!(
                res,
                Err(Error::EntityNotFound {
                    entity: "task",
                    id: 100
                })
            ),
            "EntityNotFound not matching"
        );

        Ok(())
    }

    #[serial]
    #[tokio::test]
    async fn test_list_ok() -> Result<()> {
        // -- Setup & Fixtures
        let mm = _dev_utils::init_test().await;
        let ctx = Ctx::root_ctx();
        let fx_titles = &["test_list_ok-task 01", "test_list_ok-task 02"];
        _dev_utils::seed_tasks(&ctx, &mm, fx_titles).await?;

        // -- Exec
        let tasks = TaskBmc::list(&ctx, &mm).await?;

        // -- Check
        let tasks: Vec<Task> = tasks
            .into_iter()
            .filter(|t| t.title.starts_with("test_list_ok-task"))
            .collect();
        assert_eq!(tasks.len(), 2, "number of seeded tasks.");

        // -- Clean
        let id = String::from("value");
        for task in tasks.iter() {
            //TaskBmc::delete(&ctx, &mm, id.clone()).await?;
        }

        Ok(())
    }

    #[serial]
    #[tokio::test]
    async fn test_update_ok() -> Result<()> {
        // -- Setup & Fixtures
        let mm = _dev_utils::init_test().await;
        let ctx = Ctx::root_ctx();
        let fx_title = "test_update_ok - task 01";
        let fx_title_new = "test_update_ok - task 01 - new";
        let fx_task = _dev_utils::seed_tasks(&ctx, &mm, &[fx_title])
            .await?
            .remove(0);

        // -- Exec
        TaskBmc::update(
            &ctx,
            &mm,
            fx_task.id,
            TaskForUpdate {
                title: Some(fx_title_new.to_string()),
            },
        )
        .await?;
        let resource = (String::from("value"), String::from("value1"));
        // -- Check
        let task = TaskBmc::get(&ctx, &mm, resource).await?;
        assert_eq!(task.title, fx_title_new);

        Ok(())
    }
}
// endregion: --- Tests
