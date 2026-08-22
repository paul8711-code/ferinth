use crate::*;

#[tokio::test]
#[stubr::mock("misc/instance_statistics.json")]
async fn instance_statistics() -> anyhow::Result<()> {
    let ctx = TestContext::new(&stubr.uri()).await?;

    ctx.modrinth.instance_statistics().await?;

    Ok(())
}
