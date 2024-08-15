use std::sync::{atomic::AtomicUsize, Arc};

use crossbeam::queue::ArrayQueue;

use super::{Sender, Receiver};

use crate::BoundedSharedDetails;

pub fn channel<T>(size: usize) -> (Sender<T>, Receiver<T>)
{
    
    let shared_details = Arc::new(BoundedSharedDetails::new(ArrayQueue::<T>::new(size), (), ()));

    let sender_count = Arc::new(());

    let weak_sender_count = Arc::downgrade(&sender_count);

    let receiver_count = Arc::new(());

    let sender = Sender::new(&shared_details, sender_count, &receiver_count);

    let receiver = Receiver::new(shared_details, weak_sender_count, receiver_count);

    (sender, receiver)

}
