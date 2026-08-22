use crate::*;

#[tokio::test]
#[stubr::mock("version/delete.json")]
async fn delete() -> anyhow::Result<()> {
    let ctx = TestContext::new(&stubr.uri()).await?;

    ctx.modrinth.version_delete("IIJJKKLL").await?;

    Ok(())

}
