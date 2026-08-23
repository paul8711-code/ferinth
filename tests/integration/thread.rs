use crate::*;
use ferinth::structures::thread;

#[tokio::test]
#[stubr::mock("thread/submit_report.json")]
async fn submit_report() -> anyhow::Result<()> {
    let ctx = TestContext::new(&stubr.uri()).await?;

    ctx.modrinth
        .submit_report(&thread::ReportSubmission {
            report_type: "copyright".to_string(),
            item_id: "EEFFGGHH".to_string(),
            item_type: thread::ReportItemType::Project,
            body: "This is a reupload of my mod, AABBCCDD!".to_string(),
        })
        .await?;

    Ok(())
}
