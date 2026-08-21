use paul8711_ferinth as ferinth;

use ferinth::structures::{project, UtcTime};
use ferinth::{Authenticated, Ferinth};
use serde_json::json;
use url::Url;
use wiremock::matchers::{body_json, header, method, path, query_param};
use wiremock::{Mock, MockServer, ResponseTemplate};
mod integration {
    mod project;
}

struct TestContext {
    mock_server: MockServer,
    modrinth: Ferinth<Authenticated>,
}

impl TestContext {
    async fn new() -> anyhow::Result<Self> {
        let mock_server = MockServer::start().await;
        let base_url = Url::parse(&mock_server.uri())?;

        let modrinth = Ferinth::new_with_base_url(
            env!("CARGO_CRATE_NAME"),
            Some(env!("CARGO_PKG_VERSION")),
            None,
            "token",
            base_url,
        )?;

        Ok(Self {
            mock_server,
            modrinth,
        })
    }
}
