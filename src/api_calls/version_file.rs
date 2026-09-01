//! API calls related to version files
//!
//! [documentation](https://docs.modrinth.com/api/operations/tags/version-files/)

use super::*;
use crate::structures::version::*;
use std::collections::HashMap;

impl Ferinth<Authenticated> {
    /**
    Delete the version file with the `hash`.
    Optionally specify the version ID to delete the version file from, if multiple files of the same hash exist.

    ```no_run
    # use paul8711_ferinth as ferinth;
    # tokio_test::block_on(async {
    # let modrinth = ferinth::Ferinth::<ferinth::Authenticated>::new(
    #     env!("CARGO_CRATE_NAME"),
    #     Some(env!("CARGO_PKG_VERSION")),
    #     None,
    #     "token",
    # )?;
    modrinth.version_file_delete_from_hash("795d4c12bffdb1b21eed5ff87c07ce5ca3c0dcbf", ferinth::structures::version::ValidHashAlgorithm::SHA1, None).await?;
    # Ok::<_, ferinth::Error>(()) }).unwrap()
    ```
    */
    pub async fn version_file_delete_from_hash(
        &self,
        hash: &str,
        hash_algorithm: ValidHashAlgorithm,
        version_id: Option<&str>,
    ) -> Result<()> {
        match hash_algorithm {
            ValidHashAlgorithm::SHA1 => check_sha1_hash(&[hash])?,
            ValidHashAlgorithm::SHA512 => check_sha512_hash(&[hash])?,
        }
        let mut url = self
            .url
            .join_all(vec!["version_file", hash])
            .with_query("algorithm", hash_algorithm);
        if let Some(version_id) = version_id {
            check_id_slug(&[version_id])?;
            url = url.with_query("version_id", version_id);
        }
        self.client.delete(url).custom_send().await?;
        Ok(())
    }
}

impl<T> Ferinth<T> {
    /**
    Get the version of the version file with `hash`.

    ## Example
    ```no_run
    # use paul8711_ferinth as ferinth;
    # tokio_test::block_on(async {
    # let modrinth = ferinth::Ferinth::default();
    // If a mod file has the hash `795d4c12bffdb1b21eed5ff87c07ce5ca3c0dcbf`, we can get the version it belongs to
    let sodium_version = modrinth.version_get_from_hash("795d4c12bffdb1b21eed5ff87c07ce5ca3c0dcbf", ferinth::structures::version::ValidHashAlgorithm::SHA1).await?;
    assert_eq!(sodium_version.project_id, "AANobbMI");
    # Ok::<_, ferinth::Error>(()) }).unwrap()
    ```
    */
    pub async fn version_get_from_hash(
        &self,
        hash: &str,
        hash_algorithm: ValidHashAlgorithm,
    ) -> Result<Version> {
        match hash_algorithm {
            ValidHashAlgorithm::SHA1 => check_sha1_hash(&[hash])?,
            ValidHashAlgorithm::SHA512 => check_sha512_hash(&[hash])?,
        }
        self.client
            .get(
                self.url
                    .join_all(vec!["version_file", hash])
                    .with_query("algorithm", hash_algorithm),
            )
            .custom_send_json()
            .await
    }

    /**
    Get the versions of the version files with `hashes`

    Returns a map where the keys are the hashes given.

    ## Example
    ```no_run
    # use paul8711_ferinth as ferinth;
    # tokio_test::block_on(async {
    # let modrinth = ferinth::Ferinth::default();
    let sodium_hash = "795d4c12bffdb1b21eed5ff87c07ce5ca3c0dcbf";
    let snwylvspls_hash = "994ee99d172a5950a51ec2d08c158d270722d871";
    let versions = modrinth.version_get_from_multiple_hashes(&[
        sodium_hash,
        snwylvspls_hash,
    ],
    ferinth::structures::version::ValidHashAlgorithm::SHA1).await?;
    assert_eq!(versions[sodium_hash].project_id, "AANobbMI");
    assert_eq!(versions[snwylvspls_hash].project_id, "of7wIinq");
    # Ok::<_, ferinth::Error>(()) }).unwrap()
    ```
    */
    pub async fn version_get_from_multiple_hashes(
        &self,
        hashes: &[&str],
        hash_algorithm: ValidHashAlgorithm,
    ) -> Result<HashMap<String, Version>> {
        match hash_algorithm {
            ValidHashAlgorithm::SHA1 => check_sha1_hash(hashes)?,
            ValidHashAlgorithm::SHA512 => check_sha512_hash(hashes)?,
        }
        self.client
            .post(
                self.url
                    .join_all(vec!["version_files"])
                    .with_query("algorithm", hash_algorithm),
            )
            .json(&HashesBody {
                hashes: hashes.iter().map(|h| h.to_string()).collect(),
                algorithm: HashAlgorithm::SHA1,
            })
            .custom_send_json()
            .await
    }

    /// Get the latest version for the project of the version file with `hash` based on some `filters`.
    pub async fn version_get_latest_from_hash(
        &self,
        hash: &str,
        hash_algorithm: ValidHashAlgorithm,
        filters: &LatestVersionBody,
    ) -> Result<Version> {
        match hash_algorithm {
            ValidHashAlgorithm::SHA1 => check_sha1_hash(&[hash])?,
            ValidHashAlgorithm::SHA512 => check_sha512_hash(&[hash])?,
        }
        self.client
            .post(
                self.url
                    .join_all(vec!["version_file", hash, "update"])
                    .with_query("algorithm", hash_algorithm),
            )
            .json(filters)
            .custom_send_json()
            .await
    }

    /// Get the latest versions of the projects of the version files with hashes based on some `filters`.
    pub async fn version_get_latest_from_multiple_hashes(
        &self,
        hashes: &[&str],
        hash_algorithm: ValidHashAlgorithm,
        filters: &LatestVersionBody,
    ) -> Result<HashMap<String, Version>> {
        match hash_algorithm {
            ValidHashAlgorithm::SHA1 => check_sha1_hash(hashes)?,
            ValidHashAlgorithm::SHA512 => check_sha512_hash(hashes)?,
        }
        self.client
            .post(
                self.url
                    .join_all(vec!["version_files", "update"])
                    .with_query("algorithm", hash_algorithm),
            )
            .json(&LatestVersionsBody {
                hashes: hashes.iter().map(|h| h.to_string()).collect(),
                algorithm: HashAlgorithm::SHA1,
                loaders: filters.loaders.clone(),
                game_versions: filters.game_versions.clone(),
            })
            .custom_send_json()
            .await
    }
}
