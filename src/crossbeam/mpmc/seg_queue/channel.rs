use std::sync::{atomic::AtomicUsize, Arc};

use crossbeam::queue::SegQueue;

use super::{Sender, Receiver};

use crate::SharedDetails;

pub fn channel<T>() -> (Sender<T>, Receiver<T>)
{

    //let shared_details = Arc::new((SegQueue::<T>::new(), AtomicUsize::new(0), ()));
    
    let shared_details = Arc::new(SharedDetails::new(SegQueue::<T>::new(), ()));

    let sender_count = Arc::new(());

    let weak_sender_count = Arc::downgrade(&sender_count);

    let receiver_count = Arc::new(());

    let sender = Sender::new(&shared_details, sender_count, &receiver_count);

    let recevier = Receiver::new(shared_details, weak_sender_count, receiver_count);

    (sender, recevier)

}
