use crate::*;

#[tokio::test]
async fn get() -> anyhow::Result<()> {
    let ctx = TestContext::new().await?;

    let project_id = "test_project";

    let body_json = json!({
        "id": "AABBCCDD",
        "team": "MMNNOOPP",
        "title": project_id,
        "description": "test project",
        "body": "A long Test project",
        "status": "archived",
        "project_type": "mod",
        "categories": [
            "technology",
            "adventure",
            "fabric"
        ],
        "additional_categories": [
            "technology",
            "adventure",
            "fabric"
        ],
        "environment": ["client_and_server"],
        "game_versions": ["1.19"],
        "loaders": ["neoforge", "fabric"],
        "versions": ["IIJJKKLL"],
        "license": {
            "id": "LGPL-3.0-or-later",
            "name": "GNU Lesser General Public License v3 or later",
            "url": None::<String>
        },
        "published": "2026-08-19T22:00:00.000Z",
        "updated": "2026-08-19T22:00:00.000Z",
        "downloads": 3,
        "followers": 1,
        "gallery": [],
        "thread_id": "",
        "monetization_status": "force-demonetized",
        "slug": String::new(),
        "organization": None::<String>,
        "requested_status": None::<String>,
        "approved": "2026-08-19T22:00:00.000Z",
        "queued": None::<String>,
        "icon_url": None::<String>,
        "raw_icon_url": None::<String>,
        "color": 8703084,
        "issues_url": None::<String>,
        "source_url": None::<String>,
        "wiki_url": None::<String>,
        "discord_url": None::<String>,
        "donation_urls": Vec::<String>::new(),
    });

    Mock::given(method("GET"))
        .and(path(format!("/project/{}", project_id)))
        .respond_with(ResponseTemplate::new(200).set_body_json(body_json))
        .expect(1)
        .mount(&ctx.mock_server)
        .await;

    ctx.modrinth.project_get(project_id).await?;

    Ok(())
}

#[tokio::test]
async fn delete() -> anyhow::Result<()> {
    let ctx = TestContext::new().await?;

    let project_id = "test_project";

    Mock::given(method("DELETE"))
        .and(path(format!("/project/{}", project_id)))
        .and(header("Authorization", "token"))
        .respond_with(ResponseTemplate::new(204))
        .expect(1)
        .mount(&ctx.mock_server)
        .await;

    ctx.modrinth.project_delete(project_id).await?;

    Ok(())
}

#[tokio::test]
async fn delete_icon() -> anyhow::Result<()> {
    let ctx = TestContext::new().await?;

    let project_id = "test_project";

    Mock::given(method("DELETE"))
        .and(path(format!("/project/{}/icon", project_id)))
        .and(header("Authorization", "token"))
        .respond_with(ResponseTemplate::new(204))
        .expect(1)
        .mount(&ctx.mock_server)
        .await;

    ctx.modrinth.project_delete_icon(project_id).await?;

    Ok(())
}

#[tokio::test]
async fn edit_icon() -> anyhow::Result<()> {
    let ctx = TestContext::new().await?;

    let project_id = "test_project";

    Mock::given(method("PATCH"))
        .and(path(format!("/project/{}/icon", project_id)))
        .and(header("Authorization", "token"))
        .and(query_param("ext", "png"))
        .respond_with(ResponseTemplate::new(204))
        .expect(1)
        .mount(&ctx.mock_server)
        .await;

    let image = std::fs::read("test_image.png").expect("Cannot read test image");
    ctx.modrinth
        .project_edit_icon(project_id, image, project::ImageFileExt::PNG)
        .await?;

    Ok(())
}

#[tokio::test]
async fn check_validity() -> anyhow::Result<()> {
    let ctx = TestContext::new().await?;

    let project_id = "AABBCCDD";

    Mock::given(method("GET"))
        .and(path(format!("/project/{}/check", project_id)))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "id": project_id,
        })))
        .expect(1)
        .mount(&ctx.mock_server)
        .await;

    ctx.modrinth.project_check_validity(project_id).await?;

    Ok(())
}

#[tokio::test]
async fn add_gallery_image() -> anyhow::Result<()> {
    let ctx = TestContext::new().await?;

    let project_id = "test_project";

    Mock::given(method("POST"))
        .and(path(format!("/project/{}/gallery", project_id)))
        .and(header("Authorization", "token"))
        .and(query_param("ext", "png"))
        .and(query_param("featured", "true"))
        .and(query_param("title", "test_image"))
        .and(query_param("description", "test image"))
        .respond_with(ResponseTemplate::new(204))
        .expect(1)
        .mount(&ctx.mock_server)
        .await;

    let image_data = std::fs::read("test_image.png").expect("Failed to read test image");
    ctx.modrinth
        .project_add_gallery_image(
            project_id,
            image_data,
            &project::ImageFileExt::PNG,
            true,
            Some("test_image".to_string()),
            Some("test image".to_string()),
        )
        .await?;

    Ok(())
}

#[tokio::test]
async fn delete_gallery_image() -> anyhow::Result<()> {
    let ctx = TestContext::new().await?;

    let project_id = "test_project";

    Mock::given(method("DELETE"))
        .and(path(format!("/project/{}/gallery", project_id)))
        .and(header("Authorization", "token"))
        .and(query_param("url", "https://example.com/"))
        .respond_with(ResponseTemplate::new(204))
        .expect(1)
        .mount(&ctx.mock_server)
        .await;

    ctx.modrinth
        .project_delete_gallery_image(project_id, "https://example.com/")
        .await?;

    Ok(())
}

#[tokio::test]
async fn edit_gallery_image() -> anyhow::Result<()> {
    let ctx = TestContext::new().await?;

    let project_id = "test_project";

    Mock::given(method("PATCH"))
        .and(path(format!("/project/{}/gallery", project_id)))
        .and(header("Authorization", "token"))
        .and(query_param("url", "https://example.com/"))
        .and(query_param("featured", "false"))
        .and(query_param("title", "modified_test_image"))
        .respond_with(ResponseTemplate::new(204))
        .expect(1)
        .mount(&ctx.mock_server)
        .await;

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
async fn follow() -> anyhow::Result<()> {
    let ctx = TestContext::new().await?;

    let project_id = "test_project";

    Mock::given(method("POST"))
        .and(path(format!("/project/{}/follow", project_id)))
        .and(header("Authorization", "token"))
        .respond_with(ResponseTemplate::new(204))
        .expect(1)
        .mount(&ctx.mock_server)
        .await;

    ctx.modrinth.project_follow(project_id).await?;

    Ok(())
}

#[tokio::test]
async fn unfollow() -> anyhow::Result<()> {
    let ctx = TestContext::new().await?;

    let project_id = "test_project";

    Mock::given(method("DELETE"))
        .and(path(format!("/project/{}/follow", project_id)))
        .and(header("Authorization", "token"))
        .respond_with(ResponseTemplate::new(204))
        .expect(1)
        .mount(&ctx.mock_server)
        .await;

    ctx.modrinth.project_unfollow(project_id).await?;

    Ok(())
}

#[tokio::test]
async fn schedule() -> anyhow::Result<()> {
    let ctx = TestContext::new().await?;

    let project_id = "test_project";

    let time = "2023-02-05T19:39:55.551839Z";

    Mock::given(method("POST"))
        .and(path(format!("/project/{}/schedule", project_id)))
        .and(body_json(json!({
            "time": time,
            "requested_status": "approved",
        })))
        .respond_with(ResponseTemplate::new(204))
        .expect(1)
        .mount(&ctx.mock_server)
        .await;

    ctx.modrinth
        .project_schedule(
            project_id,
            &time.parse::<UtcTime>().expect("parsing should not fail"),
            &project::RequestedStatus::Approved,
        )
        .await?;

    Ok(())
}
