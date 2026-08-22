use paul8711_ferinth as ferinth;

use ferinth::structures::{project, UtcTime};
use ferinth::{Authenticated, Ferinth};
use url::Url;
mod integration {
    mod project;
}

struct TestContext {
    modrinth: Ferinth<Authenticated>,
}

impl TestContext {
    async fn new(base_url: &str) -> anyhow::Result<Self> {
        let base_url = Url::parse(base_url)?;

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
