use crate::*;

#[tokio::test]
async fn delete() -> anyhow::Result<()> {
    let ctx = TestContext::new().await?;

    ctx.modrinth.version_delete("IIJJKKLL").await?;

    Ok(())
}
