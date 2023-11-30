use serde::Serialize;
use surrealdb::sql::Number;

use crate::ctx::Ctx;
use crate::model::task::{Record, Task, Tasks};
use crate::model::ModelManager;
use crate::model::{Error, Result};

use super::country::Country_month;

pub trait DbBmc {
    const TABLE: &'static str;
}

pub async fn create<MC, E: Serialize>(
    _ctx: &Ctx,
    mm: &ModelManager,
    (resource): (String, String),
    data: E,
) -> Result<Country_month>
where
    MC: DbBmc,
{
    println!("{:?}", "created");

    let db = mm.db();
    let ee: i64 = 9997;
    /* let add_task: Task = Task {
        id: ee,
        title: "Testtitle1".to_owned(),
    }; */

    /* let created: Task = db.create("add_task").content(add_task).await?;
     */

    let created: Option<Country_month> = db.create(resource).content(data).await?;
    println!("{:?}", created);
    println!("{:?}", "created");
    created.ok_or(crate::model::Error::EntityNotFound { entity: "", id: 2 })
    //todo!()
}

pub async fn get<MC>(
    _ctx: &Ctx,
    mm: &ModelManager,
    (resource): (String, String),
    id: i64,
) -> Result<Task>
where
    MC: DbBmc,
    // E: for<'r> FromRow<'r, PgRow> + Unpin + Send,
{
    let db = mm.db();
    println!("{:?}", "select");

    let persons: Vec<Record> = db.select(("person2")).await.unwrap();

    println!("{:?}", persons);

    let person: Option<Tasks> = db
        .select(("person2", surrealdb::sql::Id::Number(663)))
        .await?;
    println!("{:?}", "<<<<<<select>>>>>>");
    println!("{:?}", person);
    let person: Option<Task> = db
        .select(("person2", surrealdb::sql::Id::Number(663)))
        .await?;

    /* let entity: E = sqlb::select()
            .table(MC::TABLE)
            .columns(E::field_names())
            .and_where("id", "=", id)
            .fetch_optional(db)
            .await?
            .ok_or(Error::EntityNotFound {
                entity: MC::TABLE,
                id,
            })?;
    */
    println!("{:?}", person);
    println!("{:?}", "wwwwwwwww");
    person.ok_or(Error::EntityNotFound {
        entity: "()",
        id: 2,
    })
    // todo!()
}

pub async fn list<MC, E>(_ctx: &Ctx, mm: &ModelManager) -> Result<Vec<E>>
where
    MC: DbBmc,
    //E: for<'r> FromRow<'r, PgRow> + Unpin + Send,
{
    /* let db = mm.db();

    let entities: Vec<E> = sqlb::select()
        .table(MC::TABLE)
        .columns(E::field_names())
        .order_by("id")
        .fetch_all(db)
        .await?;

    Ok(entities) */
    todo!()
}

pub async fn update<MC, E>(_ctx: &Ctx, mm: &ModelManager, id: i64, data: E) -> Result<()>
where
    MC: DbBmc,
{
    let db = mm.db();

    /*  let fields = data.not_none_fields();
    let count = sqlb::update()
        .table(MC::TABLE)
        .and_where("id", "=", id)
        .data(fields)
        .exec(db)
        .await?;

    if count == 0 {
        Err(Error::EntityNotFound {
            entity: MC::TABLE,
            id,
        })
    } else {
        Ok(())
    } */
    todo!()
}

pub async fn delete(
    _ctx: &Ctx,
    mm: &ModelManager,
    resource: (String, String),
) -> Result<Country_month> {
    let db = mm.db();

    let deleted: Option<Country_month> = db.delete(resource).await?;

    deleted.ok_or(Error::EntityNotFound {
        entity: "()",
        id: 2,
    })
    /*
    let count = sqlb::delete()
        .table(MC::TABLE)
        .and_where("id", "=", id)
        .exec(db)
        .await?;

    if count == 0 {
        Err(Error::EntityNotFound {
            entity: MC::TABLE,
            id,
        })
    } else {
        Ok(())
    } */
}
