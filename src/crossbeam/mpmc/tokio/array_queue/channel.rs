use std::sync::{atomic::AtomicUsize, Arc};

use crossbeam::queue::ArrayQueue;

use crate::BoundedSharedDetails;

use tokio::sync::Notify;

use super::{Sender, Receiver};


pub fn channel<T>(size: usize) -> (Sender<T>, Receiver<T>)
{

    let shared_details = Arc::new(BoundedSharedDetails::new(ArrayQueue::<T>::new(size), Notify::new(), Notify::new()));
    
    let sender_count = Arc::new(());

    (Sender::new(&shared_details, &sender_count), Receiver::new(&shared_details, &sender_count))

}

