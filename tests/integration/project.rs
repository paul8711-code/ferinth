use crate::*;

#[tokio::test]
#[stubr::mock("project/get.json")]
async fn get() -> anyhow::Result<()> {
    let ctx = TestContext::new(&stubr.uri()).await?;

    let project_id = "create";

    ctx.modrinth.project_get(project_id).await?;

    Ok(())
}

#[tokio::test]
#[stubr::mock("project/delete.json")]
async fn delete() -> anyhow::Result<()> {
    let ctx = TestContext::new(&stubr.uri()).await?;

    let project_id = "test_project";

    ctx.modrinth.project_delete(project_id).await?;

    Ok(())
}

#[tokio::test]
#[stubr::mock("project/delete_icon.json")]
async fn delete_icon() -> anyhow::Result<()> {
    let ctx = TestContext::new(&stubr.uri()).await?;

    let project_id = "test_project";

    ctx.modrinth.project_delete_icon(project_id).await?;

    Ok(())
}

#[tokio::test]
#[stubr::mock("project/edit_icon.json")]
async fn edit_icon() -> anyhow::Result<()> {
    let ctx = TestContext::new(&stubr.uri()).await?;

    let project_id = "test_project";

    ctx.modrinth
        .project_edit_icon(project_id, "image data", project::ImageFileExt::PNG)
        .await?;

    Ok(())
}

#[tokio::test]
#[stubr::mock("project/check_validity.json")]
async fn check_validity() -> anyhow::Result<()> {
    let ctx = TestContext::new(&stubr.uri()).await?;

    let project_id = "AABBCCDD";

    ctx.modrinth.project_check_validity(project_id).await?;

    Ok(())
}

#[tokio::test]
#[stubr::mock("project/add_gallery_image.json")]
async fn add_gallery_image() -> anyhow::Result<()> {
    let ctx = TestContext::new(&stubr.uri()).await?;

    let project_id = "test_project";

    ctx.modrinth
        .project_add_gallery_image(
            project_id,
            "image data",
            &project::ImageFileExt::PNG,
            true,
            Some("test_image".to_string()),
            Some("test image".to_string()),
        )
        .await?;

    Ok(())
}

#[tokio::test]
#[stubr::mock("project/delete_gallery_image.json")]
async fn delete_gallery_image() -> anyhow::Result<()> {
    let ctx = TestContext::new(&stubr.uri()).await?;

    let project_id = "test_project";

    ctx.modrinth
        .project_delete_gallery_image(project_id, "https://example.com/")
        .await?;

    Ok(())
}

#[tokio::test]
#[stubr::mock("project/edit_gallery_image.json")]
async fn edit_gallery_image() -> anyhow::Result<()> {
    let ctx = TestContext::new(&stubr.uri()).await?;

    let project_id = "test_project";

    ctx.modrinth
        .project_edit_gallery_image(
            project_id,
            "https://example.com/",
            Some(false),
            Some("modified_test_image"),
            None,
            None,
        )
        .await?;

    Ok(())
}

#[tokio::test]
#[stubr::mock("project/follow.json")]
async fn follow() -> anyhow::Result<()> {
    let ctx = TestContext::new(&stubr.uri()).await?;

    let project_id = "test_project";

    ctx.modrinth.project_follow(project_id).await?;

    Ok(())
}

#[tokio::test]
#[stubr::mock("project/unfollow.json")]
async fn unfollow() -> anyhow::Result<()> {
    let ctx = TestContext::new(&stubr.uri()).await?;

    let project_id = "test_project";

    ctx.modrinth.project_unfollow(project_id).await?;

    Ok(())
}

#[tokio::test]
#[stubr::mock("project/schedule.json")]
async fn schedule() -> anyhow::Result<()> {
    let ctx = TestContext::new(&stubr.uri()).await?;

    let project_id = "test_project";

    let time = "2023-02-05T19:39:55.551839Z";

    ctx.modrinth
        .project_schedule(
            project_id,
            &time.parse::<UtcTime>().expect("parsing should not fail"),
            &project::RequestedStatus::Approved,
        )
        .await?;

    Ok(())
}
