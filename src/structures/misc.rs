//! Models related to miscellaneous API calls

use super::*;

#[derive(Deserialize, Serialize, Debug, Clone, Copy)]
pub struct Statistics {
    /// The number of project on Modrinth
    pub projects: Int,
    /// The number of versions on Modrinth
    pub versions: Int,
    /// The number of version files on Modrinth
    pub files: Int,
    /// The number of authors (users with projects) on Modrinth
    pub authors: Int,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Welcome {
    pub about: String,
    pub documentation: Url,
    pub name: String,
    pub version: String,
}
