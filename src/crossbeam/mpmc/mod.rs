//!
//! Multi-producer multi-consumer channels based on [Crossbeam queues](https://docs.rs/crossbeam/0.8.4/crossbeam/queue/index.html).
//! 

//pub mod array_queue;

//pub mod seg_queue;

pub mod base;

#[cfg(feature="std")]
pub mod std;

#[cfg(feature="tokio")]
pub mod tokio;

