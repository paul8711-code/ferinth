//! API calls related to users
//!
//! [documentation](https://docs.modrinth.com/api/operations/tags/users/)

use super::*;
use crate::structures::{project::Project, user::*};

impl<T> Ferinth<T> {
    /// Get the user of `user_id`
    ///
    /// ## Example
    /// ```no_run
    /// # tokio_test::block_on(async {
    /// # let modrinth = paul8711_ferinth::Ferinth::default();
    /// let simibubi = modrinth.user_get("Ud4jzpdg").await?;
    /// assert_eq!(
    ///     simibubi.role,
    ///     paul8711_ferinth::structures::user::UserRole::Developer,
    /// );
    /// # Ok::<_, paul8711_ferinth::Error>(()) }).unwrap()
    /// ```
    pub async fn user_get(&self, user_id: &str) -> Result<User> {
        check_id_slug(&[user_id])?;
        self.client
            .get(self.url.join_all(vec!["user", user_id]))
            .custom_send_json()
            .await
    }

    /// Get the users of `user_ids`
    ///
    /// ## Example
    /// ```no_run
    /// # use paul8711_ferinth::structures::user::UserRole;
    /// # tokio_test::block_on(async {
    /// # let modrinth = paul8711_ferinth::Ferinth::default();
    /// let users = modrinth.user_get_multiple(&["Ud4jzpdg", "zvYJrcc6"]).await?;
    /// assert_eq!(users.len(), 2);
    /// # Ok::<_, paul8711_ferinth::Error>(()) }).unwrap()
    /// ```
    pub async fn user_get_multiple(&self, user_ids: &[&str]) -> Result<Vec<User>> {
        check_id_slug(user_ids)?;
        self.client
            .get(
                self.url
                    .join_all(vec!["users"])
                    .with_query_json("ids", user_ids)?,
            )
            .custom_send_json()
            .await
    }

    /// Get the projects of the user of `user_id`
    ///
    /// ## Example
    /// ```no_run
    /// # tokio_test::block_on(async {
    /// # let modrinth = paul8711_ferinth::Ferinth::default();
    /// let simibubi_projects = modrinth.user_list_projects("Ud4jzpdg").await?;
    /// assert_eq!(simibubi_projects.len(), 1);
    /// # Ok::<_, paul8711_ferinth::Error>(()) }).unwrap()
    /// ```
    pub async fn user_list_projects(&self, user_id: &str) -> Result<Vec<Project>> {
        check_id_slug(&[user_id])?;
        self.client
            .get(self.url.join_all(vec!["user", user_id, "projects"]))
            .custom_send_json()
            .await
    }
}

impl Ferinth<Authenticated> {
    /// Get the projects that the user of `user_id` has followed
    ///
    /// ## Example
    /// ```no_run
    /// # tokio_test::block_on(async {
    /// # let modrinth = paul8711_ferinth::Ferinth::<paul8711_ferinth::Authenticated>::new(
    /// #     env!("CARGO_CRATE_NAME"),
    /// #     Some(env!("CARGO_PKG_VERSION")),
    /// #     None,
    /// #     "token",
    /// # )?;
    /// # let user_id = modrinth.user_get_current().await?.id;
    /// let projects = modrinth.user_list_followed_projects(&user_id).await?;
    /// # Ok::<_, paul8711_ferinth::Error>(()) }).unwrap()
    /// ```
    pub async fn user_list_followed_projects(&self, user_id: &str) -> Result<Vec<Project>> {
        check_id_slug(&[user_id])?;
        self.client
            .get(self.url.join_all(vec!["user", user_id, "follows"]))
            .custom_send_json()
            .await
    }

    /// Get the user from the current authorisation header
    ///
    /// ## Example
    /// ```no_run
    /// # tokio_test::block_on(async {
    /// # let modrinth = paul8711_ferinth::Ferinth::<paul8711_ferinth::Authenticated>::new(
    /// #     env!("CARGO_CRATE_NAME"),
    /// #     Some(env!("CARGO_PKG_VERSION")),
    /// #     None,
    /// #     "token",
    /// # )?;
    /// let current_user = modrinth.user_get_current().await?;
    /// // The email should be visible as we are authorised
    /// assert!(current_user.email.is_some());
    /// # Ok::<_, paul8711_ferinth::Error>(()) }).unwrap()
    /// ```
    pub async fn user_get_current(&self) -> Result<User> {
        self.client
            .get(self.url.join_all(vec!["user"]))
            .custom_send_json()
            .await
    }
}
