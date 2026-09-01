use crate::*;
use ferinth::structures::version;

#[tokio::test]
async fn get_from_hash() -> anyhow::Result<()> {
    let ctx = TestContext::new().await?;

    ctx.modrinth
        .version_get_from_hash(
            "ea0f38408102e4d2efd53c2cc11b88b711996b48d8922f76ea6abf731219c5bd1efe39ddf9cce77c54d49a62ff10fb685c00d2e4c524ab99d20f6296677ab2c4",
            version::ValidHashAlgorithm::SHA512,
        )
        .await?;

    Ok(())
}

#[tokio::test]
async fn file_delete_from_hash() -> anyhow::Result<()> {
    let ctx = TestContext::new().await?;

    ctx.modrinth
        .version_file_delete_from_hash(
            "ea0f38408102e4d2efd53c2cc11b88b711996b48d8922f76ea6abf731219c5bd1efe39ddf9cce77c54d49a62ff10fb685c00d2e4c524ab99d20f6296677ab2c4",
            version::ValidHashAlgorithm::SHA512,
            None,
        )
        .await?;

    Ok(())
}

#[tokio::test]
async fn get_latest_from_hash() -> anyhow::Result<()> {
    let ctx = TestContext::new().await?;

    let filters = version::LatestVersionBody {
        loaders: vec!["fabric".into()],
        game_versions: vec!["1.18".into(), "1.18.1".into()],
    };

    ctx.modrinth
        .version_get_latest_from_hash(
            "380e4a7aa7c746db8bd908991823c9f38b5569a4",
            version::ValidHashAlgorithm::SHA1,
            &filters,
        )
        .await?;

    Ok(())
}

#[tokio::test]
async fn get_from_multiple_hashes() -> anyhow::Result<()> {
    let ctx = TestContext::new().await?;

    ctx.modrinth
        .version_get_from_multiple_hashes(
            &[
                "619e250c133106bacc3e3b560839bd4b324dfda8",
                "380e4a7aa7c746db8bd908991823c9f38b5569a4",
            ],
            version::ValidHashAlgorithm::SHA1,
        )
        .await?;

    Ok(())
}

#[tokio::test]
async fn get_latest_from_multiple_hashes() -> anyhow::Result<()> {
    let ctx = TestContext::new().await?;

    let filters = version::LatestVersionBody {
        loaders: vec!["fabric".into()],
        game_versions: vec!["1.18".into(), "1.18.1".into()],
    };

    ctx.modrinth
        .version_get_latest_from_multiple_hashes(
            &[
                "619e250c133106bacc3e3b560839bd4b324dfda8",
                "380e4a7aa7c746db8bd908991823c9f38b5569a4",
            ],
            version::ValidHashAlgorithm::SHA1,
            &filters,
        )
        .await?;

    Ok(())
}
