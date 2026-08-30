//! Models related to projects

use super::*;
use std::fmt;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Project {
    /// The project's slug, used for vanity URLs.
    /// This can change at any time, so use the [`Self::id`] for long term storage.
    pub slug: String,
    pub title: String,
    /// A short description of the project
    pub description: String,
    pub categories: Vec<String>,
    pub environment: Vec<EnvironmentType>,
    /// A long form description of the project
    pub body: String,
    pub status: ProjectStatus,
    /// The status requested. Only visible to those with appropriate permissions
    pub requested_status: Option<RequestedStatus>,
    /// A list of categories which are searchable but non-primary
    pub additional_categories: Vec<String>,
    /// A link to submit bugs or issues with the project
    #[serde(deserialize_with = "deserialise_optional_url")]
    pub issues_url: Option<Url>,
    /// A link to the project's source code
    #[serde(deserialize_with = "deserialise_optional_url")]
    pub source_url: Option<Url>,
    /// A link to the project's wiki page or other relevant information
    #[serde(deserialize_with = "deserialise_optional_url")]
    pub wiki_url: Option<Url>,
    /// The project's Discord server invite
    #[serde(deserialize_with = "deserialise_optional_url")]
    pub discord_url: Option<Url>,
    pub donation_urls: Vec<DonationLink>,
    pub project_type: ProjectType,
    pub downloads: Int,
    #[serde(deserialize_with = "deserialise_optional_url")]
    pub icon_url: Option<Url>,
    /// The RGB color of the project, automatically generated from the project icon
    pub color: Option<Int>,
    /// The ID of the moderation thread associated with this project
    pub thread_id: ID,
    pub monetization_status: MonetizationStatus,
    pub id: ID,
    /// The ID of the team that has ownership of this project
    pub team: ID,
    pub published: UtcTime,
    pub updated: UtcTime,
    /// The date the project's status was approved
    pub approved: Option<UtcTime>,
    pub queued: Option<UtcTime>,
    pub followers: Int,
    pub license: License,
    /// A list of the version IDs of the project.
    /// This will only ever be empty if the project is a draft.
    pub versions: Vec<ID>,
    /// A list of all of the game versions supported by the project
    pub game_versions: Vec<String>,
    /// A list of all of the loaders supported by the project
    pub loaders: Vec<String>,
    /// A list of images that have been uploaded to the project's gallery
    pub gallery: Vec<GalleryItem>,
    // The ID of the organisation that owns this project
    pub organization: Option<ID>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct License {
    /// The SPDX license ID of a project
    pub id: String,
    pub name: String,
    /// The URL to this license
    #[serde(deserialize_with = "deserialise_optional_url")]
    pub url: Option<Url>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct DonationLink {
    /// The donation platform's ID
    pub id: String,
    pub platform: String,
    /// A link to the donation platform and user
    pub url: Url,
}

/// An image that have been uploaded to a project's gallery
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GalleryItem {
    pub url: Url,
    pub raw_url: Url,
    pub featured: bool,
    pub title: Option<String>,
    pub description: Option<String>,
    pub created: UtcTime,
    /// The order of the gallery image.
    /// Gallery images are sorted by this field and then alphabetically by title.
    pub ordering: isize,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProjectDependencies {
    pub projects: Vec<Project>,
    pub versions: Vec<version::Version>,
}

/// Fields to edit on the projects specified
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EditMultipleProjectsBody {
    /// Set all of the categories to the categories specified here
    pub categories: Vec<String>,
    /// Add all of the categories specified here
    pub add_categories: Vec<String>,
    /// Remove all of the categories specified here
    pub remove_categories: Vec<String>,
    /// Set all of the additional categories to the categories specified here
    pub additional_categories: Vec<String>,
    /// Add all of the additional categories specified here
    pub add_additional_categories: Vec<String>,
    /// Remove all of the additional categories specified here
    pub remove_additional_categories: Vec<String>,
    /// Set all of the donation links to the donation links specified here
    pub donation_urls: Vec<DonationLink>,
    /// Add all of the donation links specified here
    pub add_donation_urls: Vec<DonationLink>,
    /// Remove all of the donation links specified here
    pub remove_donation_urls: Vec<DonationLink>,
    /// A link to where to submit bugs or issues with the projects
    pub issues_url: Option<String>,
    /// A link to the source code of the projects
    pub source_url: Option<String>,
    /// A link to the projects' wiki page or other relevant information
    pub wiki_url: Option<String>,
    /// An optional invite link to the projects' discord
    pub discord_url: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum ProjectStatus {
    Approved,
    Archived,
    /// A moderator's message should be available in the project struct
    Rejected,
    Draft,
    /// The project has been approved and is publicly accessible, but will not show up in search results
    Unlisted,
    /// The project has been submitted for approval and is being reviewed
    Processing,
    Withheld,
    /// The project's status has been scheduled to change.
    /// Check the project's `requested_status` for more information.
    Scheduled,
    Private,
    #[serde(other)]
    Unknown,
}

#[derive(Deserialize, Serialize, Debug, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum RequestedStatus {
    Approved,
    Archived,
    Unlisted,
    Private,
    Draft,
    #[serde(other)]
    Other,
}

#[derive(Deserialize, Serialize, Debug, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum MonetizationStatus {
    Monetized,
    Demonetized,
    ForceDemonetized,
    #[serde(other)]
    Other,
}

pub type ProjectSupportRange = EnvironmentType;
#[derive(Deserialize, Serialize, Debug, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum EnvironmentType {
    /// The mod is required on the client side
    ClientOnly,
    /// The mod is required on the server side
    ServerOnly,
    /// The mod only runs on a dedicated server, not Singleplayer
    DedicatedServerOnly,
    /// The mod is required on both the server and the client
    ClientAndServer,
    /// The mod must be on the server, can be on the client for enhanced functionality
    ServerOnlyClientOptional,
    /// The mod must be on the client, can be on the server for enhanced functionality
    ClientOnlyServerOptional,
    /// The mod can be installed on just the client or just the server, but functionality is
    /// enhanced when on both
    ClientOrServerPrefersBoth,
    /// The mod can be installed on just the client or just the server and functionality is the same
    ClientOrServer,
    /// The mod only works in Singleplayer, does not in Multiplayer
    SingleplayerOnly,
    /// It is unknown if the project will run on this side
    Unknown,
}

#[derive(Deserialize, Serialize, Debug, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ProjectType {
    Mod,
    Modpack,
    Resourcepack,
    Shader,
    Plugin,
    Datapack,
    MinecraftJavaServer,
    #[serde(other)]
    Other,
}

impl fmt::Display for ProjectType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            ProjectType::Mod => "mod",
            ProjectType::Modpack => "modpack",
            ProjectType::Resourcepack => "resourcepack",
            ProjectType::Shader => "shader",
            ProjectType::Plugin => "plugin",
            ProjectType::Datapack => "datapack",
            ProjectType::MinecraftJavaServer => "minecraft_java_server",
            ProjectType::Other => "other",
        };
        write!(f, "{s}")
    }
}

/// File extensions for images
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ImageFileExt {
    /// [Portable Network Graphics](https://en.wikipedia.org/wiki/PNG)
    PNG,
    /// [Joint Photographic Experts Group](https://en.wikipedia.org/wiki/JPEG)
    JPG,
    /// [Joint Photographic Experts Group](https://en.wikipedia.org/wiki/JPEG)
    JPEG,
    /// [Bitmap](https://en.wikipedia.org/wiki/BMP_file_format)
    BMP,
    /// [Graphics Interchange Format](https://en.wikipedia.org/wiki/GIF)
    GIF,
    /// [Web Picture](https://en.wikipedia.org/wiki/WebP)
    WebP,
    /// [Scalable Vector Graphics](https://en.wikipedia.org/wiki/SVG)
    SVG,
    /// [Scalable Vector Graphics](https://en.wikipedia.org/wiki/SVG#Compression) (gZip compressed)
    SVGZ,
    /// [Silicon Graphics Image](https://en.wikipedia.org/wiki/Silicon_Graphics_Image)
    RGB,
}

impl std::fmt::Display for ImageFileExt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", format!("{:?}", self).to_lowercase())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Sort {
    Relevance,
    /// Sorts matches by the number of downloads
    Downloads,
    /// Sorts matches by the number of followers
    Follows,
    /// Sorts by the time of initial creation
    Newest,
    /// Sorts by the time of the latest update
    Updated,
}

impl std::fmt::Display for Sort {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", format!("{self:?}").to_lowercase())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Facet {
    ProjectType(project::ProjectType),
    /// Mod loader or category to filter
    Categories(String),
    /// Game versions to filter
    Versions(String),
    OpenSource(bool),
    /// License ID to filter
    License(String),
    /// A custom facet
    ///
    /// [documentation](https://docs.modrinth.com/api-spec#tag/projects/operation/searchProjects)
    Custom {
        /// The type of metadata to filter
        _type: String,
        /// The comparison to use
        ///
        /// Can be `=`/`:`, `!=`, `>`, `>=`, `<`, `<=`
        operation: String,
        /// The value to compare against
        value: String,
    },
}

impl Serialize for Facet {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let output = match self {
            Facet::ProjectType(project_type) => {
                format!("project_type:{project_type}")
            }
            Facet::Categories(category) => format!("categories:{category}"),
            Facet::Versions(version) => format!("versions:{version}"),
            Facet::OpenSource(bool) => format!("open_source:{bool}"),
            Facet::License(license_id) => format!("license:{license_id}"),
            Facet::Custom {
                _type,
                operation,
                value,
            } => format!("{_type} {operation} {value}"),
        };
        serializer.collect_str(&output)
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Response {
    pub hits: Vec<SearchHit>,
    /// The number of results that were skipped by the query
    pub offset: Int,
    /// The number of results that were returned by the query
    pub limit: Int,
    /// The total number of results that match the query
    pub total_hits: Int,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct SearchHit {
    /// The project's slug, used for vanity URLs.
    /// This can change at any time, so use the [`Self::project_id`] for long term storage.
    pub slug: Option<String>,
    pub title: String,
    pub description: String,
    pub categories: Vec<String>,
    pub environment: Vec<project::EnvironmentType>,
    pub project_type: project::ProjectType,
    pub downloads: Int,
    #[serde(deserialize_with = "deserialise_optional_url")]
    pub icon_url: Option<Url>,
    /// The RGB color of the project, automatically generated from the project icon
    pub color: Option<Int>,
    /// The ID of the moderation thread associated with this project
    pub thread_id: Option<ID>,
    pub monetization_status: Option<project::MonetizationStatus>,
    pub project_id: ID,
    /// Username of the project's authour
    pub author: String,
    /// A list of the project's primary/featured categories
    pub display_categories: Vec<String>,
    #[serde(rename = "versions")]
    /// A list of all of the game versions supported by the project
    pub game_versions: Vec<String>,
    pub follows: Int,
    pub date_created: UtcTime,
    pub date_modified: UtcTime,
    /// The latest game version that this project supports
    pub latest_version: String,
    /// The SPDX license ID of a project
    pub license: String,
    pub gallery: Vec<Url>,
    pub featured_gallery: Option<Url>,
}
