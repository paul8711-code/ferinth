use crate::*;

#[tokio::test]
async fn get() -> anyhow::Result<()> {
    let ctx = TestContext::new().await?;

    ctx.modrinth.user_get("modrinth").await?;

    Ok(())
}

#[tokio::test]
async fn get_current() -> anyhow::Result<()> {
    let ctx = TestContext::new().await?;

    ctx.modrinth.user_get_current().await?;

    Ok(())
}

#[tokio::test]
async fn get_multiple() -> anyhow::Result<()> {
    let ctx = TestContext::new().await?;

    ctx.modrinth
        .user_get_multiple(&["modrinth", "simibubi"])
        .await?;

    Ok(())
}

#[tokio::test]
async fn remove_avatar() -> anyhow::Result<()> {
    let ctx = TestContext::new().await?;

    ctx.modrinth.user_remove_avatar("my_user").await?;

    Ok(())
}

#[tokio::test]
async fn list_projects() -> anyhow::Result<()> {
    let ctx = TestContext::new().await?;

    ctx.modrinth.user_list_projects("simibubi").await?;

    Ok(())
}

#[tokio::test]
async fn list_followed_projects() -> anyhow::Result<()> {
    let ctx = TestContext::new().await?;

    ctx.modrinth.user_list_followed_projects("my_user").await?;

    Ok(())
}
