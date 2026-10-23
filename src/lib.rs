#![doc = include_str!("../README.md")] 

#![cfg_attr(docsrs, feature(doc_cfg))]

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

//Disabled

/*

mod single_waker_multi_permit;

pub use single_waker_multi_permit::*;

#[cfg(test)]
mod single_waker_multi_permit_tests;

*/

//Re-enabled in v0.4.0?

#[cfg(feature="crossbeam_queue")]
pub mod crossbeam_queue;

//Disabled

/*

mod limited_single_waker_multi_permit;

pub use limited_single_waker_multi_permit::*;

#[cfg(test)]
mod limited_single_waker_multi_permit_tests;

*/

//Re-enabled in v0.4.0?

///
/// The preferred type of mutex to be used for synchronisation.
/// 
#[cfg(feature="use_std_sync")]
pub type PreferredMutexType<T> = ::std::sync::Mutex<T>;

///
/// The preferred type of mutex to be used for synchronisation.
/// 
#[cfg(feature="use_parking_lot_sync")]
pub type PreferredMutexType<T> = parking_lot::Mutex<T>;

//MutexGuard Aliases

#[cfg(feature="use_std_sync")]
pub type MutexGuard<'a, T> = ::std::sync::MutexGuard<'a, T>;

#[cfg(feature="use_parking_lot_sync")]
pub type MutexGuard<'a, T> = parking_lot::MutexGuard<'a, T>;

#[cfg(feature="use_parking_lot_fair_sync")]
pub type MutexGuard<'a, T> = parking_lot::FairMutexGuard<'a, T>;

//

///
/// The preferred type of mutex to be used for synchronisation.
/// 
#[cfg(feature="use_parking_lot_fair_sync")]
pub type PreferredMutexType<T> = parking_lot::FairMutex<T>;

///
/// The preferred type of RwLock to be used for synchronisation.
/// 
#[cfg(not(feature="use_faux_rwlock"))]
#[cfg(feature="use_std_sync")]
pub type PreferredRwLockType<T> = ::std::sync::RwLock<T>;

///
/// The preferred type of RwLock to be used for synchronisation.
/// 
#[cfg(not(feature="use_faux_rwlock"))]
#[cfg(any(feature="use_parking_lot_sync", feature="use_parking_lot_fair_sync"))]
pub type PreferredRwLockType<T> = parking_lot::RwLock<T>;

#[cfg(feature="use_faux_rwlock")]
pub type PreferredRwLockType<T> = crate::sync::FauxRwlock<T>;

//RwLockReadGuard and RwLockWriteGuard Aliases

cfg_select!
{

    feature="use_faux_rwlock" =>
    {

        pub type RwLockReadGuard<'a, T> = crate::sync::FauxRwLockReadGuard<'a, T>;

        pub type RwLockWriteGuard<'a, T> = crate::sync::FauxRwLockWriteGuard<'a, T>;

    }
    feature="use_std_sync" =>
    {

        pub type RwLockReadGuard<'a, T> = ::std::sync::RwLockReadGuard<'a, T>;

        pub type RwLockWriteGuard<'a, T> = ::std::sync::RwLockWriteGuard<'a, T>;

    }
    any(feature="use_parking_lot_sync", feature="use_parking_lot_fair_sync") =>
    {

        pub type RwLockReadGuard<'a, T> = parking_lot::RwLockReadGuard<'a, T>;

        pub type RwLockWriteGuard<'a, T> = parking_lot::RwLockWriteGuard<'a, T>;

    }

}

//

pub mod shared_reading_and_writing;

mod waker_queue_with_updated_item;

pub use waker_queue_with_updated_item::*;

mod item_updater;

pub use  item_updater::*;

mod item_updaters;

pub use item_updaters::*;

//Disabled

//mod drop_box;

//pub use drop_box::*;

//Disabled

//mod single_item_drop_box;

//pub use single_item_drop_box::*;

pub mod multi_shot;

mod enums;

pub use enums::*;

static EXPECTED_VALUE_NOT_FOUND_MESSAGE: &str = "Error: Expected value not found.";

mod channel_shared_details_with_waker_queues;

pub use channel_shared_details_with_waker_queues::*;

//Disabled

//mod auto_waker;

//pub use auto_waker::*;

pub mod no_swp;

pub mod atomic;

mod spin_count;

pub use spin_count::*;

pub mod sync;
