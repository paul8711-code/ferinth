use crate::*;

#[tokio::test]
async fn get() -> anyhow::Result<()> {
    let ctx = TestContext::new().await?;

    ctx.modrinth.user_get("modrinth").await?;

    Ok(())
}
