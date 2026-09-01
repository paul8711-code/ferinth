use crate::*;

#[tokio::test]
async fn list_project_members() -> anyhow::Result<()> {
    let ctx = TestContext::new().await?;

    ctx.modrinth.team_list_project_members("create").await?;

    Ok(())
}
