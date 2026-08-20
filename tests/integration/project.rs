use paul8711_ferinth as ferinth;

use ferinth::structures::{project, UtcTime};
use ferinth::Ferinth;
use serde_json::json;
use url::Url;
use wiremock::matchers::{body_json, header, method, path, query_param};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn delete_project() -> anyhow::Result<()> {
    let mock_server = MockServer::start().await;
    let base_url = Url::parse(&mock_server.uri())?;

    let project_id = "test_project";

    Mock::given(method("DELETE"))
        .and(path(format!("/project/{}", project_id)))
        .and(header("Authorization", "token"))
        .respond_with(ResponseTemplate::new(204))
        .expect(1)
        .mount(&mock_server)
        .await;

    let modrinth = Ferinth::new_with_base_url(
        env!("CARGO_CRATE_NAME"),
        Some(env!("CARGO_PKG_VERSION")),
        None,
        "token",
        base_url,
    )?;

    modrinth.project_delete(project_id).await?;

    Ok(())
}

#[tokio::test]
async fn delete_project_icon() -> anyhow::Result<()> {
    let mock_server = MockServer::start().await;
    let base_url = Url::parse(&mock_server.uri())?;

    let project_id = "test_project";

    Mock::given(method("DELETE"))
        .and(path(format!("/project/{}/icon", project_id)))
        .and(header("Authorization", "token"))
        .respond_with(ResponseTemplate::new(204))
        .expect(1)
        .mount(&mock_server)
        .await;

    let modrinth = Ferinth::new_with_base_url(
        env!("CARGO_CRATE_NAME"),
        Some(env!("CARGO_PKG_VERSION")),
        None,
        "token",
        base_url,
    )?;

    modrinth.project_delete_icon(project_id).await?;

    Ok(())
}

#[tokio::test]
async fn change_project_icon() -> anyhow::Result<()> {
    let mock_server = MockServer::start().await;
    let base_url = Url::parse(&mock_server.uri())?;

    let project_id = "test_project";

    Mock::given(method("PATCH"))
        .and(path(format!("/project/{}/icon", project_id)))
        .and(header("Authorization", "token"))
        .and(query_param("ext", "png"))
        .respond_with(ResponseTemplate::new(204))
        .expect(1)
        .mount(&mock_server)
        .await;

    let modrinth = Ferinth::new_with_base_url(
        env!("CARGO_CRATE_NAME"),
        Some(env!("CARGO_PKG_VERSION")),
        None,
        "token",
        base_url,
    )?;

    let image = std::fs::read("test_image.png").expect("Cannot read test image");
    modrinth
        .project_edit_icon(project_id, image, project::ImageFileExt::PNG)
        .await?;

    Ok(())
}

#[tokio::test]
async fn check_validity() -> anyhow::Result<()> {
    let mock_server = MockServer::start().await;
    let base_url = Url::parse(&mock_server.uri())?;

    let project_id = "AABBCCDD";

    Mock::given(method("GET"))
        .and(path(format!("/project/{}/check", project_id)))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "id": project_id,
        })))
        .expect(1)
        .mount(&mock_server)
        .await;

    let modrinth = Ferinth::new_with_base_url(
        env!("CARGO_CRATE_NAME"),
        Some(env!("CARGO_PKG_VERSION")),
        None,
        "token",
        base_url,
    )?;

    modrinth.project_check_validity(project_id).await?;

    Ok(())
}

#[tokio::test]
async fn add_gallery_image() -> anyhow::Result<()> {
    let mock_server = MockServer::start().await;
    let base_url = Url::parse(&mock_server.uri())?;

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
        .mount(&mock_server)
        .await;

    let modrinth = Ferinth::new_with_base_url(
        env!("CARGO_CRATE_NAME"),
        Some(env!("CARGO_PKG_VERSION")),
        None,
        "token",
        base_url,
    )?;

    let image_data = std::fs::read("test_image.png").expect("Failed to read test image");
    modrinth
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
    let mock_server = MockServer::start().await;
    let base_url = Url::parse(&mock_server.uri())?;

    let project_id = "test_project";

    Mock::given(method("DELETE"))
        .and(path(format!("/project/{}/gallery", project_id)))
        .and(header("Authorization", "token"))
        .and(query_param("url", "https://example.com/"))
        .respond_with(ResponseTemplate::new(204))
        .expect(1)
        .mount(&mock_server)
        .await;

    let modrinth = Ferinth::new_with_base_url(
        env!("CARGO_CRATE_NAME"),
        Some(env!("CARGO_PKG_VERSION")),
        None,
        "token",
        base_url,
    )?;

    modrinth
        .project_delete_gallery_image(project_id, "https://example.com/")
        .await?;

    Ok(())
}

#[tokio::test]
async fn modify_gallery_image() -> anyhow::Result<()> {
    let mock_server = MockServer::start().await;
    let base_url = Url::parse(&mock_server.uri())?;

    let project_id = "test_project";

    Mock::given(method("PATCH"))
        .and(path(format!("/project/{}/gallery", project_id)))
        .and(header("Authorization", "token"))
        .and(query_param("url", "https://example.com/"))
        .and(query_param("featured", "false"))
        .and(query_param("title", "modified_test_image"))
        .respond_with(ResponseTemplate::new(204))
        .expect(1)
        .mount(&mock_server)
        .await;

    let modrinth = Ferinth::new_with_base_url(
        env!("CARGO_CRATE_NAME"),
        Some(env!("CARGO_PKG_VERSION")),
        None,
        "token",
        base_url,
    )?;

    modrinth
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
    let mock_server = MockServer::start().await;
    let base_url = Url::parse(&mock_server.uri())?;

    let project_id = "test_project";

    Mock::given(method("POST"))
        .and(path(format!("/project/{}/follow", project_id)))
        .and(header("Authorization", "token"))
        .respond_with(ResponseTemplate::new(204))
        .expect(1)
        .mount(&mock_server)
        .await;

    let modrinth = Ferinth::new_with_base_url(
        env!("CARGO_CRATE_NAME"),
        Some(env!("CARGO_PKG_VERSION")),
        None,
        "token",
        base_url,
    )?;

    modrinth.project_follow(project_id).await?;

    Ok(())
}

#[tokio::test]
async fn unfollow() -> anyhow::Result<()> {
    let mock_server = MockServer::start().await;
    let base_url = Url::parse(&mock_server.uri())?;

    let project_id = "test_project";

    Mock::given(method("DELETE"))
        .and(path(format!("/project/{}/follow", project_id)))
        .and(header("Authorization", "token"))
        .respond_with(ResponseTemplate::new(204))
        .expect(1)
        .mount(&mock_server)
        .await;

    let modrinth = Ferinth::new_with_base_url(
        env!("CARGO_CRATE_NAME"),
        Some(env!("CARGO_PKG_VERSION")),
        None,
        "token",
        base_url,
    )?;

    modrinth.project_unfollow(project_id).await?;

    Ok(())
}

#[tokio::test]
async fn schedule() -> anyhow::Result<()> {
    let mock_server = MockServer::start().await;
    let base_url = Url::parse(&mock_server.uri())?;

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
        .mount(&mock_server)
        .await;

    let modrinth = Ferinth::new_with_base_url(
        env!("CARGO_CRATE_NAME"),
        Some(env!("CARGO_PKG_VERSION")),
        None,
        "token",
        base_url,
    )?;

    modrinth
        .project_schedule(
            project_id,
            &time.parse::<UtcTime>().expect("parsing should not fail"),
            &project::RequestedStatus::Approved,
        )
        .await?;

    Ok(())
}
