use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};
use surrealdb::engine::remote::ws::{Client, Ws, Wss};
use surrealdb::sql::Thing;
use surrealdb::Surreal;
use tracing::info;

const DB_DEV_SURREAL_URL: &str = "";
//const DB_DEV_SURREAL_APP_URL: &str = "surrealdbworld.fly.dev";
const DB_DEV_SURREAL_APP_URL: &str = "0.0.0.0:8000";

const SQL_RECREATE_DB_FILE_NAME: &str = "00-recreate-db.sql";
const DB_DIR: &str = "db/dev_initial";

pub async fn init_dev_db() -> Result<(), Box<dyn std::error::Error>> {
    info!("{:<12} - {:?}\n", "SocketAddr", "test");

    // -- Get the sql_dir
    // Note: This is because cargo test and cargo run won't give the same
    //       current_dir given the worspace layout.
    let current_dir = std::env::current_dir().unwrap();
    let v: Vec<_> = current_dir.components().collect();
    let path_comp = v.get(v.len().wrapping_sub(3));
    let base_dir = if Some(true) == path_comp.map(|c| c.as_os_str() == "crates") {
        v[..v.len() - 3].iter().collect::<PathBuf>()
    } else {
        current_dir.clone()
    };
    let sql_dir = base_dir.join(DB_DIR);

    // -- Get sql files.
    let mut paths: Vec<PathBuf> = fs::read_dir(sql_dir)?
        .filter_map(|entry| entry.ok().map(|e| e.path()))
        .collect();
    paths.sort();

    // -- SQL Execute each file.
    let app_db = new_db_pool(DB_DEV_SURREAL_APP_URL).await?;
    for path in paths {
        let path_str = path.to_string_lossy();

        if path_str.ends_with(".sql") && !path_str.ends_with(SQL_RECREATE_DB_FILE_NAME) {
            println!("{:?}", path);
            //pexec(&app_db, &path).await?;
        }
    }
    Ok(())
}

async fn pexec(db: &Surreal<Client>, file: &Path) -> Result<(), std::io::Error> {
    info!("{:<12} - pexec: {file:?}", "FOR-DEV-ONLY");
    // Select a specific namespace / database
    //db.use_ns("test").use_db("test").await?;
    // -- Read the file.
    let content = fs::read_to_string(file)?;

    // FIXME: Make the split more sql proof.
    let sqls: Vec<&str> = content.split(';').collect();

    for sql in sqls {
        println!("{}", sql)
        //sqlx::query(sql).execute(db).await?;
    }

    todo!()
}

async fn new_db_pool(db_con_url: &str) -> std::result::Result<Surreal<Client>, surrealdb::Error> {
    info!("{:<12} - {:?}\n", "Trying Init db", "test");
    let db = Surreal::new::<Ws>(db_con_url).await?;

    // Signin as a namespace, database, or root user
    /* db.signin(Root {
        username: "root",
        password: "M0skwa!",
    })
    .await?; */

    db.use_ns("ns").use_db("db").await;
    Ok(db)
}

#[derive(Debug, Serialize, Deserialize)]
struct Person {
    title: String,
    name: Name,
}

#[derive(Debug, Serialize, Deserialize)]
struct Name {
    first: String,
    last: String,
}

#[cfg(test)]
mod tests {

    use super::*;
    #[tokio::test]
    async fn tt() {
        let db = new_db_pool(DB_DEV_SURREAL_APP_URL).await.unwrap();
        let mut result = db
            .query("CREATE person SET title = 'name', name.first = 'nameww', name.last = 'nameww'")
            .bind(("table", "person"))
            .await
            .unwrap();

        // Get the first result from the first query statement

        let created: Option<Person> = result.take(0).unwrap();
        // Get all of the results from the second query statement
        //let people: Vec<Person> = result.take(1).unwrap();

        print!("{:?}", created)
    }
}
