//! API request trait and utilities
//!
//! This module provides the [`ApiRequest`] trait, which describes how a request
//! type should be sent to an HTTP API. Each request type that implements this
//! trait declares its own HTTP method and endpoint path.

use http::Method;
use serde::Serialize;

/// A trait for types that represent API requests
///
/// Implementors define the HTTP method and endpoint path for the request,
/// allowing generic clients to route requests without hardcoding API details.
pub trait ApiRequest: Serialize {
    /// The HTTP method for this request
    const METHOD: Method;

    /// The API endpoint path for this request
    ///
    /// This should be the path portion of the URL, starting with a `/`.
    /// The base URL is typically provided by the client configuration.
    ///
    /// # Examples
    ///
    /// - `"/email"` - Send an email
    const ENDPOINT: &'static str;
}
