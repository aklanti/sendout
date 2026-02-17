//! # Sendout
//!
//! A flexible crate for sending emails across multiple providers.
//!
//! Provides the building blocks - types, traits, and provider integrations for sending emails
//! from your app.
#![cfg_attr(docsrs, feature(doc_cfg))]
pub mod config;
pub mod error;
#[macro_use]
mod macros;
pub mod api;
pub mod email;
pub mod execute;
#[cfg(feature = "postmark")]
pub mod postmark;
pub mod service;

#[doc(inline)]
pub use self::api::ApiRequest;
#[doc(inline)]
pub use self::execute::Execute;
#[doc(inline)]
pub use config::ServiceConfig;
#[doc(inline)]
pub use service::EmailService;
