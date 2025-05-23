//!
//! For creating re-usable locations where values can be "returned" from one thread to another.
//! 

mod base_return_store;

pub use base_return_store::*;

#[cfg(feature="tokio")]
pub mod tokio;

mod notifying_return_store;

pub use notifying_return_store::*;

mod polled_return_store;

pub use polled_return_store::*;

