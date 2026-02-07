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

//Disabled

//mod waker_queue;

//pub use waker_queue::*;

mod drop_waker;

pub use drop_waker::*;

#[cfg(feature="scc")]
pub mod scc;

mod channel_shared_details;

pub use channel_shared_details::*;

//Disabled

/*
mod single_waker;

pub use single_waker::*;

#[cfg(test)]
mod single_waker_tests;

*/

mod queued_waker;

pub use queued_waker::*;

mod limited_waker_permit_queue;

pub use limited_waker_permit_queue::*;

#[cfg(test)]
mod limited_waker_permit_queue_tests;

//#[cfg(test)]
//mod limited_waker_permit_queue_tests;

mod single_waker_multi_permit;

pub use single_waker_multi_permit::*;

#[cfg(test)]
mod single_waker_multi_permit_tests;

#[cfg(feature="crossbeam-queue")]
pub mod crossbeam_queue;

mod limited_single_waker_multi_permit;

pub use limited_single_waker_multi_permit::*;

#[cfg(feature="use_std_sync")]
pub type PreferredMutexType<T> = ::std::sync::Mutex<T>;

#[cfg(feature="use_parking_lot_sync")]
pub type PreferredMutexType<T> = parking_lot::Mutex<T>;

#[cfg(feature="use_parking_lot_fair_sync")]
pub type PreferredMutexType<T> = parking_lot::FairMutex<T>;

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