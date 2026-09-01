use crate::*;

#[tokio::test]
async fn get_from_hash() -> anyhow::Result<()> {
    let ctx = TestContext::new().await?;

    ctx.modrinth
        .version_get_from_hash("619e250c133106bacc3e3b560839bd4b324dfda8")
        .await?;

    Ok(())
}

#[tokio::test]
async fn file_delete_from_hash() -> anyhow::Result<()> {
    let ctx = TestContext::new().await?;

    ctx.modrinth
        .version_file_delete_from_hash("619e250c133106bacc3e3b560839bd4b324dfda8", None)
        .await?;

    Ok(())
}

#[tokio::test]
async fn get_from_multiple_hashes() -> anyhow::Result<()> {
    let ctx = TestContext::new().await?;

    ctx.modrinth
        .version_get_from_multiple_hashes(&[
            "619e250c133106bacc3e3b560839bd4b324dfda8",
            "380e4a7aa7c746db8bd908991823c9f38b5569a4",
        ])
        .await?;

    Ok(())
}
