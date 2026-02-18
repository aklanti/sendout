use googletest::matchers::eq;
use googletest::{expect_that, gtest};
use secrecy::ExposeSecret;
use sendout::EmailService;
use sendout::error::Error;
use serde_json::{Value, json};
use wiremock::matchers::{header, method, path};
use wiremock::{Mock, ResponseTemplate};

use crate::app::TestApp;

#[tokio::test]
#[gtest]
async fn send_mail_succeeds() {
    let app = TestApp::spawn().await;
    let response_body = email_delivery_receipt();

    Mock::given(method("POST"))
        .and(path("/email"))
        .and(header(
            "X-Postmark-Server-Token",
            app.config.server_token.expose_secret(),
        ))
        .respond_with(ResponseTemplate::new(200).set_body_json(response_body))
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

#[tokio::test]
#[gtest]
async fn send_email_hit_rate_limit() {
    let app = TestApp::spawn().await;
    Mock::given(method("POST"))
        .and(header(
            "X-Postmark-Server-Token",
            app.config.server_token.expose_secret(),
        ))
        .respond_with(ResponseTemplate::new(429))
        .expect(1)
        .mount(&app.email_server)
        .await;

    let message = TestApp::email_message();
    let email_client = app.postmark_client();
    let result = email_client.send_email(message).await;
    assert!(matches!(result, Err(Error::RateLimitExceeded)));
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
