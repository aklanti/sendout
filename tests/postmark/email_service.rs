use googletest::matchers::eq;
use googletest::{expect_that, gtest};
use secrecy::ExposeSecret;
use sendout::service::EmailService;
use serde_json::{Value, json};
use wiremock::matchers::{header, method, path};
use wiremock::{Mock, ResponseTemplate};

use crate::app::TestApp;

#[tokio::test]
#[gtest]
async fn send_mail_succeeds() {
    let app = TestApp::spawn().await;
    let body = email_delivery_receipt();

    Mock::given(method("POST"))
        .and(path("/email"))
        .and(header(
            "X-POSTMARK-SERVER",
            app.config.server_token.expose_secret(),
        ))
        .respond_with(ResponseTemplate::new(200).set_body_json(body))
        .expect(1)
        .mount(&app.email_server)
        .await;

    let message = TestApp::email_message();
    let email_client = app.postmark_client();
    let delivery = email_client
        .send_email(message)
        .await
        .expect("email to be sent");
    expect_that!(delivery.message_id, eq("msg-abc-123"));
    expect_that!(delivery.error_code, eq(0));
}

fn email_delivery_receipt() -> Value {
    json!({
        "To": "kwame.nkrumah@example.africa",
        "SubmittedAt": "2026-02-15T10:00:00Z",
        "MessageID": "msg-abc-123",
        "ErrorCode": 0,
        "Message": "Ok"
    })
}
