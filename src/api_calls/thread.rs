//! API calls related to threads
//!
//! [documentation](https://docs.modrinth.com/api/operations/tags/threads/)

use super::*;
use crate::structures::thread::*;

impl Ferinth<Authenticated> {
    /// Submit a report to the moderators
    ///
    /// Valid report types can be found using [`Ferinth::tag_list_report_types`]
    ///
    /// ## Example
    ///```no_run
    /// # tokio_test::block_on(async {
    /// # let modrinth = paul8711_ferinth::Ferinth::<paul8711_ferinth::Authenticated>::new(
    /// #     env!("CARGO_CRATE_NAME"),
    /// #     Some(env!("CARGO_PKG_VERSION")),
    /// #     None,
    /// #     "token",
    /// # )?;
    /// let report = modrinth.submit_report(&paul8711_ferinth::structures::thread::ReportSubmission {
    ///     report_type: "other".to_string(),
    ///     item_id: "XXXXXXXX".to_string(),
    ///     item_type: paul8711_ferinth::structures::thread::ReportItemType::User,
    ///     body: "This is an example report".to_string(),
    /// }).await?;
    /// # Ok::<_, paul8711_ferinth::Error>(()) }).unwrap()
    /// ```
    pub async fn submit_report(&self, report: &ReportSubmission) -> Result<Report> {
        check_id_slug(&[&report.item_id])?;
        self.client
            .post(self.url.join_all(vec!["report"]))
            .json(report)
            .custom_send_json()
            .await
    }
}
