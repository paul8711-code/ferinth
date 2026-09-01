//! [documentation](https://docs.modrinth.com/api-spec)

pub mod misc;
pub mod notification;
pub mod project;
pub mod tag;
pub mod team;
pub mod thread;
pub mod user;
pub mod version;
pub mod version_file;

use crate::{
    request::RequestBuilderCustomSend,
    structures,
    url_ext::{UrlJoinAll, UrlWithQuery},
    Authenticated, Error, Ferinth, Result,
};

/// Verify that the `inputs` are Modrinth ID or slug compliant
pub fn check_id_slug<S: AsRef<str>>(inputs: &[S]) -> Result<()> {
    for input in inputs {
        // Regex from the [Modrinth documentation](https://docs.modrinth.com/api-spec/#tag/project_model)
        if !lazy_regex::regex_is_match!(r#"^[\w!@$()`.+,"\-']{3,64}$"#, input.as_ref()) {
            return Err(Error::InvalidIDorSlug);
        }
    }
    Ok(())
}

/// Verify that the given `inputs` are SHA1 compliant
pub fn check_sha1_hash<S: AsRef<str>>(inputs: &[S]) -> Result<()> {
    for input in inputs {
        if !lazy_regex::regex_is_match!("^[a-f0-9]{40}$", input.as_ref()) {
            return Err(Error::InvalidSHA1);
        }
    }
    Ok(())
}

/// Verify that the given `inputs` are SHA512 compliant
pub fn check_sha512_hash<S: AsRef<str>>(inputs: &[S]) -> Result<()> {
    for input in inputs {
        if !lazy_regex::regex_is_match!("^[a-f0-9]{128}$", input.as_ref()) {
            return Err(Error::InvalidSHA512);
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_id_slug() {
        // some definitely valid id/slug
        let slug = "MMNNOOPP";

        assert!(check_id_slug(&[slug]).is_ok());
    }

    #[test]
    fn test_invalid_id_slug() {
        // some definitely invalid id/slug
        let slug = "M$%&\"(/§";

        assert!(check_id_slug(&[slug]).is_err());
    }

    #[test]
    fn test_valid_sha1_hash() {
        // valid sha1 hash
        let hash = "b789f294a2018edd24500b57829abf8a76164c6b";

        assert!(check_sha1_hash(&[hash]).is_ok());
    }

    #[test]
    fn test_invalid_sha1_hash() {
        // invalid sha1 hash, "g" is an invalid char here
        let hash = "e519a9f66fdfdb78d0818f3e3247f791bd9185fg";

        assert!(check_sha1_hash(&[hash]).is_err());
    }

    #[test]
    fn test_valid_sha512_hash() {
        // valid sha512 hash
        let hash = "5ddbb1657ccdb0422fd397fa6f355a0f59114c90cfffb80102e4219b93f863c4937e89b6c17180db256caca40329122539ca73b5eee35f5e07018a7e62f51c67";

        assert!(check_sha512_hash(&[hash]).is_ok());
    }

    #[test]
    fn test_invalid_sha512_hash() {
        // invalid sha512 hash, has too little chars
        let hash = "10c291b7af66feb76141f6566829cffd3bc9ff55d7e2d8c1a7d0dba4e0c6c3c9b7b061a8542bd2ab14ece7a5a85e7bef3d9eacebf11d966213259de0f2db5d7";

        assert!(check_sha512_hash(&[hash]).is_err());
    }
}
