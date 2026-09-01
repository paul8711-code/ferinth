use crate::*;
use ferinth::structures::version;

#[tokio::test]
async fn list() -> anyhow::Result<()> {
    let ctx = TestContext::new().await?;

    ctx.modrinth.version_list("create").await?;

    Ok(())
}

#[tokio::test]
async fn list_filtered() -> anyhow::Result<()> {
    let ctx = TestContext::new().await?;

    ctx.modrinth
        .version_list_filtered("create", Some(&["neoforge"]), Some(&["1.20.1"]), None)
        .await?;

    Ok(())
}

#[tokio::test]
async fn get() -> anyhow::Result<()> {
    let ctx = TestContext::new().await?;

    ctx.modrinth.version_get("UjX6dr61").await?;

    Ok(())
}

#[tokio::test]
async fn delete() -> anyhow::Result<()> {
    let ctx = TestContext::new().await?;

    ctx.modrinth.version_delete("IIJJKKLL").await?;

    Ok(())
}

#[tokio::test]
async fn get_from_number() -> anyhow::Result<()> {
    let ctx = TestContext::new().await?;

    ctx.modrinth
        .version_get_from_number("create", "6.0.10+mc1.21.1")
        .await?;

    Ok(())
}

#[tokio::test]
async fn schedule() -> anyhow::Result<()> {
    let ctx = TestContext::new().await?;

    let time = "2023-02-05T19:39:55.551839Z";

    ctx.modrinth
        .version_schedule(
            "UjX6dr61",
            &time.parse::<UtcTime>().expect("parsing should not fail"),
            &version::RequestedStatus::Archived,
        )
        .await?;

    Ok(())
}

#[tokio::test]
async fn get_multiple() -> anyhow::Result<()> {
    let ctx = TestContext::new().await?;

    ctx.modrinth
        .version_get_multiple(&["UjX6dr61", "XTVZDOol"])
        .await?;

    Ok(())
}
