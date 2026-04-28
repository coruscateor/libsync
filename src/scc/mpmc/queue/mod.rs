//!
//! A channel implementation which uses a [SCC Queue](https://docs.rs/scc/latest/scc/struct.Queue.html) for message transferral.
//! 

mod sender;

pub use sender::*;

mod receiver;

pub use receiver::*;

mod channel;

pub use channel::*;

pub mod io_channels;

mod weak_sender;

pub use weak_sender::*;

mod weak_receiver;

pub use weak_receiver::*;
