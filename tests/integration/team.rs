use crate::*;

#[tokio::test]
async fn list_project_members() -> anyhow::Result<()> {
    let ctx = TestContext::new().await?;

    ctx.modrinth.team_list_project_members("create").await?;

    Ok(())
}

#[tokio::test]
async fn list_members() -> anyhow::Result<()> {
    let ctx = TestContext::new().await?;

    ctx.modrinth.team_list_members("zftNHDXi").await?;

    Ok(())
}

#[tokio::test]
async fn add_user() -> anyhow::Result<()> {
    let ctx = TestContext::new().await?;

    ctx.modrinth.team_add_user("MMNNOOPP", "EEFFGGHH").await?;

    Ok(())
}

#[tokio::test]
async fn multiple_list_members() -> anyhow::Result<()> {
    let ctx = TestContext::new().await?;

    ctx.modrinth
        .team_multiple_list_members(&["4reLOAKe", "zftNHDXi"])
        .await?;

    Ok(())
}

#[tokio::test]
async fn join() -> anyhow::Result<()> {
    let ctx = TestContext::new().await?;

    ctx.modrinth.team_join("MMNNOOPP").await?;

    Ok(())
}

#[tokio::test]
async fn remove_member() -> anyhow::Result<()> {
    let ctx = TestContext::new().await?;

    ctx.modrinth
        .team_remove_member("MMNNOOPP", "my_user")
        .await?;

    Ok(())
}

#[tokio::test]
async fn transfer_ownership() -> anyhow::Result<()> {
    let ctx = TestContext::new().await?;

    ctx.modrinth
        .team_transfer_ownership("MMNNOOPP", "EEFFGGHH")
        .await?;

    Ok(())
}
