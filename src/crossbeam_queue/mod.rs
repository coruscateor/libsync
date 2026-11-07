//!
//! Objects that use [Crossbeam Queues](https://crates.io/crates/crossbeam-queue).
//! 

#[cfg(feature="crossbeam_queue")]
pub mod mpmc;

//Disabled

//pub mod spsc;

#[cfg(feature="use_crossbeam_queue_no_swp")]
pub mod no_swp;
