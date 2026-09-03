//! Miscellaneous API calls

use super::*;
use crate::structures::misc::*;

impl<T> Ferinth<T> {
    /// Get various statistics about this Modrinth instance
    ///
    /// ## Example
    /// ```no_run
    /// # tokio_test::block_on(async {
    /// # let modrinth = paul8711_ferinth::Ferinth::default();
    /// let statistics = modrinth.instance_statistics().await?;
    /// # Ok::<_, paul8711_ferinth::Error>(()) }).unwrap()
    /// ```
    pub async fn instance_statistics(&self) -> Result<Statistics> {
        self.client
            .get(self.url.join_all(vec!["statistics"]))
            .custom_send_json()
            .await
    }
}
