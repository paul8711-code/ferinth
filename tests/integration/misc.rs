use crate::*;

#[tokio::test]
async fn instance_statistics() -> anyhow::Result<()> {
    let ctx = TestContext::new().await?;

    ctx.modrinth.instance_statistics().await?;

    Ok(())
}
