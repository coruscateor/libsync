//!
//! A channel implementation which uses a [Crossbeam Queue SegQueue](https://docs.rs/crossbeam-queue/latest/crossbeam_queue/struct.SegQueue.html) for message transferral.
//! 

mod sender;

pub use sender::*;

mod receiver;

pub use receiver::*;

mod channel;

pub use channel::*;

#[cfg(test)]
mod channel_tests;

pub mod io_channels;
