use std::sync::{atomic::AtomicUsize, Arc};

use crossbeam::queue::SegQueue;

use super::{Sender, Receiver};

use crate::SharedDetails;

pub fn channel<T>() -> (Sender<T>, Receiver<T>)
{

    //let shared_details = Arc::new((SegQueue::<T>::new(), AtomicUsize::new(0), ()));
    
    let shared_details = Arc::new(SharedDetails::new(SegQueue::<T>::new(), ()));

    let sender_count = Arc::new(());

    (Sender::new(&shared_details, &sender_count), Receiver::new(&shared_details, &sender_count))

}
