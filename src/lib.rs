//! # Sendout
//!
//! A modular email delivery library.
//!
//! Provides core types and service traits for sending emails, batch operations, and provider integrations.
pub mod config;
pub mod error;
#[macro_use]
pub mod macros;
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
