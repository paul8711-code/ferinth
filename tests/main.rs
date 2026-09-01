use paul8711_ferinth as ferinth;

use ferinth::structures::UtcTime;
use ferinth::{Authenticated, Ferinth};
use url::Url;
mod integration {
    mod misc;
    mod project;
    mod thread;
    mod version;
}

struct TestContext {
    modrinth: Ferinth<Authenticated>,
}

impl TestContext {
    async fn new() -> anyhow::Result<Self> {
        let base_url = Url::parse("http://localhost:8080")?;

        let modrinth = Ferinth::new_with_base_url(
            env!("CARGO_CRATE_NAME"),
            Some(env!("CARGO_PKG_VERSION")),
            None,
            "token",
            base_url,
        )?;

        Ok(Self { modrinth })
    }
}
