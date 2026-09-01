//! Models related to users

use super::*;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct User {
    pub username: String,
    /// The user's display name
    pub name: Option<String>,
    /// The user's email, only visible to the user itself when authenticated
    pub email: Option<String>,
    pub email_verified: Option<bool>,
    pub has_password: Option<bool>,
    pub has_totp: Option<bool>,
    pub auth_providers: Option<Vec<AuthProvider>>,
    /// A description of the user
    pub bio: Option<String>,
    /// Various data relating to the user's payouts status,
    /// only visible to the user itself when authenticated
    pub payout_data: Option<PayoutData>,
    pub id: ID,
    pub avatar_url: Option<Url>,
    pub created: UtcTime,
    pub role: UserRole,
    /// Bitflags of badges applicable to this user
    ///
    /// [code](https://github.com/modrinth/code/blob/6c16688ca93fc1ab878d9e915246d78fa723dca8/apps/labrinth/src/models/v3/users.rs#L9-L23)
    pub badges: Int,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct PayoutData {
    pub balance: f64,
    pub payout_wallet: Option<Wallet>,
    pub payout_wallet_type: Option<WalletType>,
    pub payout_address: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum Wallet {
    PayPal,
    Venmo,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum WalletType {
    Email,
    Phone,
    UserHandle,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct TeamMember {
    /// The ID of the member's team
    pub team_id: ID,
    pub user: User,
    pub role: String,
    /// The user's permissions in bitflag format
    /// (requires authorisation to view)
    ///
    /// [code](https://github.com/modrinth/code/blob/6c16688ca93fc1ab878d9e915246d78fa723dca8/apps/labrinth/src/models/v3/teams.rs#L24-L38)
    pub permissions: Option<Int>,
    /// Whether the user has accepted membership of the team
    /// (requires authorisation to view)
    pub accepted: bool,
    /// The split of payouts going to this user.
    /// The proportion of payouts they get is their split divided by the sum of the splits of all members.
    pub payouts_split: Option<f64>,
    pub ordering: Int,
}

#[derive(Deserialize, Serialize, Debug, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
#[non_exhaustive]
pub enum UserRole {
    Developer,
    Moderator,
    Admin,
    #[serde(other)]
    Other,
}

#[derive(Deserialize, Serialize, Debug, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum AuthProvider {
    GitHub,
    Discord,
    Microsoft,
    GitLab,
    Google,
    Steam,
    PayPal,
    #[serde(other)]
    Other,
}
