use crate::*;

#[tokio::test]
async fn get() -> anyhow::Result<()> {
    let ctx = TestContext::new().await?;

    ctx.modrinth.user_get("modrinth").await?;

    Ok(())
}

#[tokio::test]
async fn list_projects() -> anyhow::Result<()> {
    let ctx = TestContext::new().await?;

    ctx.modrinth.user_list_projects("simibubi").await?;

    Ok(())
}
