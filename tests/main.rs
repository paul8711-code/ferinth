use vektrace_ferinth as ferinth;

use ferinth::structures::UtcTime;
use ferinth::{Authenticated, Ferinth};
use url::Url;
mod integration {
    mod misc;
    mod notification;
    mod project;
    mod tag;
    mod team;
    mod thread;
    mod user;
    mod version;
    mod version_file;
}

/*
When running cargo test this Docker container must run in the directory of the project:
```bash
docker run -d --name wiremock-recorder \
    --user $(id -u):$(id -g) \
    -p 8080:8080 \
    -v ./tests/fixtures:/home/wiremock \
    wiremock/wiremock:latest \
    --proxy-all="https://api.modrinth.com/v2" \
    --record-mappings
```

When adding manually implemented fixtures the container must be restarted:
`docker restart wiremock-recorder`
Afterwards stop and remove it:
`docker stop wiremock-recorder && docker rm wiremock-recorder`
*/

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
