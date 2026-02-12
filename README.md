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

## Overview

`sendout` is a crate for sending emails via API-based providers, with a focus on strong type safety, validation, and extensibility.
It supports providers like Postmark and is designed for easy integration and robust error handling.

## Design Philosophy

- **Provider Abstraction:** Unified API for multiple email providers (e.g., Postmark).
- **Strong Typing:** Compile-time validation for email fields, recipients, attachments, and headers.
- **Extensible:** Add new providers or customize existing ones with minimal effort.
- **Comprehensive Validation:** Uses [garde](https://github.com/jprochazk/garde) for field validation.
- **Test Utilities:** Includes mock senders and a rich test suite.

## Usage

### Add to Your `Cargo.toml`

```toml
[dependencies]
sendout = "0.1"
# Optional: enable Postmark support
sendout = { version = "0.1", features = ["postmark"] }
```

## Optional Features

You can enable optional features to customize the crate for your needs:

- `bon` enables builder pattern
- `garde` enables validation using the `garde` crate
- `postmark` enables Postmark provider support
- `reqwest` uses `reqwest` as the HTTP backend for sending requests
- `tracing` enables tracing instrumentation
- `test-util` enables test utilities and mock sender for integration

## Supported Rust Versions
TODO

## License

This project is licensed under the [MPL-2.0 license](LICENSE).

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in `sendout` by you, shall be licensed as MPL-2.0, without any additional
terms or conditions.
