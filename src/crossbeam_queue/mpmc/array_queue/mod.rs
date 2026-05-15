//!
//! A channel implementation which uses a [Crossbeam Queue ArrayQueue](https://docs.rs/crossbeam-queue/latest/crossbeam_queue/struct.ArrayQueue.html) for message transferral.
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

mod weak_sender;

pub use weak_sender::*;

mod weak_receiver;

pub use weak_receiver::*;

mod broadcaster;

pub use broadcaster::*;
