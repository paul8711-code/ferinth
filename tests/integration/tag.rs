use crate::*;

#[tokio::test]
async fn list_categories() -> anyhow::Result<()> {
    let ctx = TestContext::new().await?;

    ctx.modrinth.tag_list_categories().await?;

    Ok(())
}

#[tokio::test]
async fn list_loaders() -> anyhow::Result<()> {
    let ctx = TestContext::new().await?;

    ctx.modrinth.tag_list_loaders().await?;

    Ok(())
}

#[tokio::test]
async fn list_game_versions() -> anyhow::Result<()> {
    let ctx = TestContext::new().await?;

    ctx.modrinth.tag_list_game_versions().await?;

    Ok(())
}

#[tokio::test]
async fn license_text_and_title() -> anyhow::Result<()> {
    let ctx = TestContext::new().await?;

    ctx.modrinth
        .tag_license_text_and_title("LGPL-3.0-or-later")
        .await?;

    Ok(())
}

#[tokio::test]
async fn list_donation_platforms() -> anyhow::Result<()> {
    let ctx = TestContext::new().await?;

    ctx.modrinth.tag_list_donation_platforms().await?;

    Ok(())
}

#[tokio::test]
async fn list_report_types() -> anyhow::Result<()> {
    let ctx = TestContext::new().await?;

    ctx.modrinth.tag_list_report_types().await?;

    Ok(())
}

#[tokio::test]
async fn list_project_types() -> anyhow::Result<()> {
    let ctx = TestContext::new().await?;

    ctx.modrinth.tag_list_project_types().await?;

    Ok(())
}

#[tokio::test]
async fn list_side_types() -> anyhow::Result<()> {
    let ctx = TestContext::new().await?;

    ctx.modrinth.tag_list_side_types().await?;

    Ok(())
}
