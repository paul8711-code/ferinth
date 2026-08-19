use ferinth::structures::project::ProjectType;
use ferinth::structures::search::{Facet, Sort};
use ferinth::Ferinth;

#[tokio::test]
async fn search_with_project_type_filter() -> Result<(), ferinth::Error> {
    let modrinth = Ferinth::<()>::new(
        env!("CARGO_PKG_NAME"),
        Some(env!("CARGO_PKG_VERSION")),
        Some(env!("CARGO_PKG_AUTHORS")),
    );

    let response = modrinth
        .search(
            "sodium",
            &Sort::Relevance,
            vec![vec![Facet::ProjectType(ProjectType::Mod)]],
        )
        .await?;

    assert!(
        !response.hits.is_empty(),
        "Expected search results for 'sodium' but hits vector was empty."
    );
    Ok(())
}
