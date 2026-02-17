//! A trait that specifies how requests describe themselves to the HTTP layer
//!
//! The [`ApiRequest`] trait lets each request type declare its own HTTP
//! method and endpoint path, so the client knows how to send it.

use http::Method;
use serde::Serialize;

/// Describe how to turn a type into an HTTP request
pub trait ApiRequest: Serialize {
    /// Which HTTP method to use when sending this request
    const METHOD: Method;

    /// The path portion of the URL to hit and must start with a `/`
    ///
    /// # Examples
    ///
    /// - `"/email"` - Send an email
    const ENDPOINT: &'static str;
}
