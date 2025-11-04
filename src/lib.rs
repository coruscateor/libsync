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

mod limited_notifier;

pub use limited_notifier::*;

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