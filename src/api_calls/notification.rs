//! API calls related to notifications
//!
//! [documentation](https://docs.modrinth.com/api/operations/tags/notifications/)

use super::*;
use crate::structures::notification::*;

impl Ferinth<Authenticated> {
    /**
    Get the notifications of the user of `user_id`

    ## Example
    ```no_run
    # use paul8711_ferinth as ferinth;
    # tokio_test::block_on(async {
    # let modrinth = ferinth::Ferinth::<ferinth::Authenticated>::new(
    #     env!("CARGO_CRATE_NAME"),
    #     Some(env!("CARGO_PKG_VERSION")),
    #     None,
    #     "token",
    # )?;
    # let user_id = modrinth.user_get_current().await?.id;
    let notifications = modrinth.notification_list(&user_id).await?;
    # Ok::<_, ferinth::Error>(()) }).unwrap()
    ```
    */
    pub async fn notification_list(&self, user_id: &str) -> Result<Vec<Notification>> {
        check_id_slug(&[user_id])?;
        self.client
            .get(self.url.join_all(vec!["user", user_id, "notifications"]))
            .custom_send_json()
            .await
    }
}
