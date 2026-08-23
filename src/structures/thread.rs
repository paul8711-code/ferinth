//! Models related to threads

use super::*;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Report {
    pub id: ID,
    pub report_type: String,
    /// The ID of the item being reported
    pub item_id: ID,
    /// The type of item being reported
    pub item_type: ReportItemType,
    /// An extended explanation of the report
    pub body: String,
    /// The ID of the user who submitted the report
    pub reporter: ID,
    pub created: UtcTime,
    pub closed: bool,
    // The ID of the moderation thread associated with this report
    pub thread_id: ID,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ReportSubmission {
    pub report_type: String,
    /// The ID of the item being reported
    pub item_id: ID,
    /// The type of item being reported
    pub item_type: ReportItemType,
    /// An extended explanation of the report
    pub body: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum ReportItemType {
    Project,
    User,
    Version,
    Unknown,
}
