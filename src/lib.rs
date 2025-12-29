#![doc = include_str!("../README.md")] 

#![cfg_attr(docsrs, feature(doc_auto_cfg))]

//#[cfg(all(feature="crossbeam", feature="tokio"))]
#[cfg(feature="crossbeam")]
pub mod crossbeam;

#[cfg(feature="std")]
pub mod std;

mod results;

pub use results::*;

mod shared_details;

pub use shared_details::*;

mod bounded_shared_details;

pub use bounded_shared_details::*;

/*
#[cfg(feature="count_waiting_senders_and_receivers")]
mod scoped_incrementer;

#[cfg(feature="count_waiting_senders_and_receivers")]
pub use scoped_incrementer::*;
*/

#[cfg(feature="tokio")]
pub mod tokio_helpers;

mod waker_permit_queue;

pub use waker_permit_queue::*;

#[cfg(test)]
mod waker_permit_queue_tests;

//#[cfg(test)]
//pub use waker_permit_queue_tests::*;

mod waker_queue;

pub use waker_queue::*;

mod drop_waker;

pub use drop_waker::*;

#[cfg(feature="scc")]
pub mod scc;

mod channel_shared_details;

pub use channel_shared_details::*;

mod single_waker;

pub use single_waker::*;

/*
pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
*/