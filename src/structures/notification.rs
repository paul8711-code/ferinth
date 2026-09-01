//! Models related to notifications

use super::*;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Notification {
    pub id: ID,
    /// The ID of the user who received the notification
    pub user_id: ID,
    pub title: String,
    pub text: String,
    /// A _relative_ link to the related project/version
    pub link: String,
    pub read: bool,
    pub created: UtcTime,
    /// A list of actions that can be performed
    pub actions: Vec<NotificationAction>,
    pub body: NotificationBody,
}

// Undocumented struct pulled from the labrinth source code
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct NotificationAction {
    pub title: String,
    /// The route to call when this notification action is called.
    /// Contains the HTTP method and route respectively.
    pub action_route: (String, String),
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum NotificationBody {
    ProjectUpdate {
        project_id: ID,
        version_id: ID,
    },
    TeamInvite {
        project_id: ID,
        team_id: ID,
        invited_by: ID,
        role: String,
    },
    OrganizationInvite {
        organization_id: ID,
        invited_by: ID,
        team_id: ID,
        role: String,
    },
    StatusChange {
        project_id: ID,
        old_status: project::ProjectStatus,
        new_status: project::ProjectStatus,
    },
    ModeratorMessage {
        thread_id: ID,
        message_id: ID,

        project_id: Option<ID>,
        report_id: Option<ID>,
    },
    LegacyMarkdown {
        notification_type: Option<String>,
        title: String,
        text: String,
        link: String,
        actions: Vec<NotificationAction>,
    },
    Unknown,
}
