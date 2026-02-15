[![Build Status][actions-badge]][actions-url]
[![Crates.io][crates-badge]][crates-url]
[![Documentation][docs-badge]][docs-url]
[![MPL-2.0 license][mpl-2.0-badge]][mpl-2.0-license]

[crates-badge]: https://img.shields.io/crates/v/sendout
[crates-url]: https://crates.io/crates/sendout
[docs-badge]: https://img.shields.io/docsrs/sendout/latest
[docs-url]: https://docs.rs/sendout/latest/sendout/
[mpl-2.0-badge]: https://img.shields.io/badge/License-MPL_2.0-blue.svg
[mpl-2.0-license]: LICENSE
[actions-badge]: https://github.com/aklanti/sendout/workflows/CI/badge.svg
[actions-url]: https://github.com/aklanti/sendout/actions/workflows/main.yaml

## Sendout

`sendout` is a crate for sending emails via API-based providers, with a focus on strong type safety, validation, and extensibility.
It supports providers like Postmark and is designed for easy integration and robust error handling. However, it will never implement privacy invasive features like open tracking for example. 

## Usage

### Add to Your `Cargo.toml`

```toml
[dependencies]
sendout = "0.1"
# Optional: enable Postmark and reqwest support
sendout = { version = "0.1", features = ["postmark", "reqwest"] }
```

```rust
use reqwest::Client;
use sendout::email::EmailMessage;
use sendout::EmailService;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  let email_message = EmailMessage {
      from: "wangari.maathai@example.africa".to_owned(),
      to: vec!["kwame.nkrumah@example.africa"].into(),
      subject: "Green Belt Movement Monthly Update".to_owned(),
      body: Body::Text("We planted 10,000 trees across Kenya this month.".to_owned()),
  };

  let config = ServiceConfig {
    base_url: "https://example.africa".into(),
    server_token: String::from("<SERVER_TOKEN>").into(),
    account_token: Some("<ACCOUNT_TOKEN>").into(),
    from_email: "test-user".into()
  };

  let reqwest_client = Client::new();
  let postmark_client = PostmarkClient::new(client, config);
  postmark_client.send_email(email_message).await?;
}
```

## Optional Features

You can enable optional features to customize the crate for your needs:

- `bon` enables builder pattern
- `garde` enables validation using the `garde` crate
- `postmark` enables Postmark provider support
- `reqwest` uses `reqwest` as the HTTP backend for sending requests
- `tracing` enables tracing instrumentation
- `test-util` enables test utilities and mock sender for integration

## Design notes

- **Provider Abstraction:** Unified API for multiple email providers (e.g., Postmark).
- **Strong Typing:** Compile-time validation for email fields, recipients, attachments, and headers.
- **Extensible:** Add new providers or customize existing ones with minimal effort.
- **Comprehensive Validation:** Uses [garde](https://github.com/jprochazk/garde) for field validation.
- **Test Utilities:** Includes mock senders and a rich test suite.

## Supported Rust Versions

`sendout` MSRV is `1.93.0`

## License

This project is licensed under the [MPL-2.0 license](LICENSE).

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in `sendout` by you, shall be licensed as MPL-2.0, without any additional
terms or conditions.
