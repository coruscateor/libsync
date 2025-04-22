//!
//! Sender, Receiver, channel, IOClient, IOServer and io_channels implementations using [Crossbeam ArrayQueues, SegQueues](https://docs.rs/crossbeam/0.8.4/crossbeam/queue/index.html) and [Tokio Semaphores](https://docs.rs/tokio/1.44.2/tokio/sync/struct.Semaphore.html).
//! 
//! Note that no channel user can directly close the channel they are using. 
//! 

pub mod array_queue;

pub mod seg_queue;

/*
mod channel_semaphore;

pub use channel_semaphore::*;
*/



