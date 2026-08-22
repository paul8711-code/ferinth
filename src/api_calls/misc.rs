//! Miscellaneous API calls

use super::*;
use crate::structures::misc::*;

impl<T> Ferinth<T> {
    /**
    Get various statistics about this Modrinth instance

    ## Example
    ```no_run
    # use paul8711_ferinth as ferinth;
    # tokio_test::block_on(async {
    # let modrinth = ferinth::Ferinth::default();
    let statistics = modrinth.instance_statistics().await?;
    # Ok::<_, ferinth::Error>(()) }).unwrap()
    ```
    */
    pub async fn instance_statistics(&self) -> Result<Statistics> {
        self.client
            .get(self.url.join_all(vec!["statistics"]))
            .custom_send_json()
            .await
    }
}
