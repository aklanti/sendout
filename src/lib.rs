//! # Sendout
//!
//! Provides an abstraction over sending emails using APIs
pub mod config;
pub mod error;

use async_trait::async_trait;

use self::error::EmailError;

/// Trait for sending emails
///
/// This trait defines the mechanism for sending emails.
#[async_trait]
pub trait EmailSender: Send + Sync {
    /// Send an email
    async fn send(&self) -> Result<(), EmailError>;
}
