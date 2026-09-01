use crate::*;

#[tokio::test]
async fn list() -> anyhow::Result<()> {
    let ctx = TestContext::new().await?;

    ctx.modrinth.notification_list("my_user").await?;

    Ok(())
}
