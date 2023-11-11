#![allow(unused)]

use anyhow::Result;
use serde_json::json;

#[tokio::test]
async fn quick_dev() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:9000")?;

    hc.do_get("/hello").await?.print().await?;
    //hc.do_get("/hello/test?name=Jens").await?.print().await?;
    //hc.do_get("/src/main.rs").await?.print().await?;
    hc.do_post("/api/login", json!({"username": "demo1", "pw": "welcome"}))
        .await?
        .print()
        .await?;

    let req_create_ticket = hc.do_post("/api/tickets", json!({"title": "Ticket AAA"}));
    req_create_ticket.await?.print().await?;

    hc.do_delete("/api/tickets/2").await?.print().await?;

    hc.do_get("/api/tickets").await?.print().await?;

    Ok(())
}
