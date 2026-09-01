use crate::*;

#[tokio::test]
async fn get_from_hash() -> anyhow::Result<()> {
    let ctx = TestContext::new().await?;

    ctx.modrinth
        .version_get_from_hash("619e250c133106bacc3e3b560839bd4b324dfda8")
        .await?;

    Ok(())
}
