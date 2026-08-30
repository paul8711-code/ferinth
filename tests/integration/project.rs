use crate::*;
use ferinth::structures::project;

#[tokio::test]
#[stubr::mock("project/search.json")]
async fn search() -> anyhow::Result<()> {
    use project::{Facet, ProjectType};

    let ctx = TestContext::new(&stubr.uri()).await?;

    let facets = vec![
        vec![Facet::ProjectType(ProjectType::Mod)],
        vec![Facet::Categories("fabric".to_string())],
        vec![Facet::Versions("1.20.1".to_string())],
        vec![Facet::OpenSource(true), Facet::License("MIT".to_string())],
    ];

    ctx.modrinth
        .project_search("create fabric", &project::Sort::Downloads, facets)
        .await?;

    Ok(())
}

#[tokio::test]
#[stubr::mock("project/search_paged.json")]
async fn search_paged() -> anyhow::Result<()> {
    use project::{Facet, ProjectType};

    let ctx = TestContext::new(&stubr.uri()).await?;

    let facets = vec![
        vec![Facet::ProjectType(ProjectType::Mod)],
        vec![Facet::Categories("fabric".to_string())],
        vec![Facet::Versions("1.20.1".to_string())],
        vec![Facet::OpenSource(true), Facet::License("MIT".to_string())],
    ];

    ctx.modrinth
        .project_search_paged("create fabric", &project::Sort::Downloads, 1, 1, facets)
        .await?;

    Ok(())
}

#[tokio::test]
#[stubr::mock("project/get.json")]
async fn get() -> anyhow::Result<()> {
    let ctx = TestContext::new(&stubr.uri()).await?;

    ctx.modrinth.project_get("create").await?;

    Ok(())
}

#[tokio::test]
#[stubr::mock("project/delete.json")]
async fn delete() -> anyhow::Result<()> {
    let ctx = TestContext::new(&stubr.uri()).await?;

    ctx.modrinth.project_delete("test_project").await?;

    Ok(())
}

#[tokio::test]
#[stubr::mock("project/get_multiple.json")]
async fn get_multiple() -> anyhow::Result<()> {
    let ctx = TestContext::new(&stubr.uri()).await?;

    ctx.modrinth
        .project_get_multiple(&["create", "farmers-delight"])
        .await?;

    Ok(())
}

#[tokio::test]
#[stubr::mock("project/edit_multiple.json")]
async fn edit_multiple() -> anyhow::Result<()> {
    let ctx = TestContext::new(&stubr.uri()).await?;

    let body = project::EditMultipleProjectsBody {
        categories: vec!["fabric".to_string(), "decoration".to_string()],
        add_categories: vec![],
        remove_categories: vec![],
        additional_categories: vec![],
        add_additional_categories: vec!["forge".to_string(), "adventure".to_string()],
        remove_additional_categories: vec![],
        donation_urls: vec![],
        add_donation_urls: vec![],
        remove_donation_urls: vec![project::DonationLink {
            id: "patreon".to_string(),
            platform: "Patreon".to_string(),
            url: Url::parse("https://www.patreon.com/my_user").unwrap(),
        }],
        issues_url: None,
        source_url: None,
        wiki_url: Some("https://example.com".to_string()),
        discord_url: None,
    };

    ctx.modrinth
        .project_edit_multiple(&["AABBCCDD", "EEFFGGHH"], body)
        .await?;

    Ok(())
}

#[tokio::test]
#[stubr::mock("project/get_random.json")]
async fn get_random() -> anyhow::Result<()> {
    let ctx = TestContext::new(&stubr.uri()).await?;

    ctx.modrinth.project_get_random(2).await?;

    Ok(())
}

#[tokio::test]
#[stubr::mock("project/delete_icon.json")]
async fn delete_icon() -> anyhow::Result<()> {
    let ctx = TestContext::new(&stubr.uri()).await?;

    ctx.modrinth.project_delete_icon("test_project").await?;

    Ok(())
}

#[tokio::test]
#[stubr::mock("project/edit_icon.json")]
async fn edit_icon() -> anyhow::Result<()> {
    let ctx = TestContext::new(&stubr.uri()).await?;

    ctx.modrinth
        .project_edit_icon("test_project", "image data", project::ImageFileExt::PNG)
        .await?;

    Ok(())
}

#[tokio::test]
#[stubr::mock("project/check_validity.json")]
async fn check_validity() -> anyhow::Result<()> {
    let ctx = TestContext::new(&stubr.uri()).await?;

    ctx.modrinth.project_check_validity("AABBCCDD").await?;

    Ok(())
}

#[tokio::test]
#[stubr::mock("project/add_gallery_image.json")]
async fn add_gallery_image() -> anyhow::Result<()> {
    let ctx = TestContext::new(&stubr.uri()).await?;

    ctx.modrinth
        .project_add_gallery_image(
            "test_project",
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

    ctx.modrinth
        .project_delete_gallery_image("test_project", "https://example.com/")
        .await?;

    Ok(())
}

#[tokio::test]
#[stubr::mock("project/edit_gallery_image.json")]
async fn edit_gallery_image() -> anyhow::Result<()> {
    let ctx = TestContext::new(&stubr.uri()).await?;

    ctx.modrinth
        .project_edit_gallery_image(
            "test_project",
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
#[stubr::mock("project/get_dependencies.json")]
async fn get_dependencies() -> anyhow::Result<()> {
    let ctx = TestContext::new(&stubr.uri()).await?;

    ctx.modrinth.project_get_dependencies("create").await?;

    Ok(())
}

#[tokio::test]
#[stubr::mock("project/follow.json")]
async fn follow() -> anyhow::Result<()> {
    let ctx = TestContext::new(&stubr.uri()).await?;

    ctx.modrinth.project_follow("test_project").await?;

    Ok(())
}

#[tokio::test]
#[stubr::mock("project/unfollow.json")]
async fn unfollow() -> anyhow::Result<()> {
    let ctx = TestContext::new(&stubr.uri()).await?;

    ctx.modrinth.project_unfollow("test_project").await?;

    Ok(())
}

#[tokio::test]
#[stubr::mock("project/schedule.json")]
async fn schedule() -> anyhow::Result<()> {
    let ctx = TestContext::new(&stubr.uri()).await?;

    let time = "2023-02-05T19:39:55.551839Z";

    ctx.modrinth
        .project_schedule(
            "test_project",
            &time.parse::<UtcTime>().expect("parsing should not fail"),
            &project::RequestedStatus::Approved,
        )
        .await?;

    Ok(())
}
