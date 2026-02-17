//! Core emails types: messages, recipients, attachments, and delivery receipt
pub mod delivery;
pub mod message;

#[doc(inline)]
pub use delivery::EmailDelivery;
#[doc(inline)]
pub use message::{Attachment, Body, EmailMessage, Header, Recipients};
