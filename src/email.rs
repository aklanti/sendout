//! Email data structure

pub mod delivery;
pub mod message;

#[doc(inline)]
pub use delivery::EmailDelivery;
#[doc(inline)]
pub use message::{Attachment, Body, EmailMessage, Header, Recipients};
