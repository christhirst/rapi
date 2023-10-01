#![allow(unused)]

use anyhow::Result;

#[tokio::test]
async fn quick_dev() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:3100")?;
    hc.do_get("/ss").await?.print().await?;
    Ok(())
}