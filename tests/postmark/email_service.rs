use crate::app::TestApp;

#[tokio::test]
async fn send_mail_succeeds() {
    let app = TestApp::with_postmark()
        .await
        .expect("to spawn test app successfully");
}
