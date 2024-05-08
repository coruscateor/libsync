use std::sync::{atomic::AtomicUsize, Arc};

use crossbeam::queue::ArrayQueue;

use super::{Sender, Receiver};

use crate::BoundedSharedDetails;

pub fn channel<T>(size: usize) -> (Sender<T>, Receiver<T>)
{

    //let shared_details = Arc::new((ArrayQueue::<T>::new(size), AtomicUsize::new(0), (), AtomicUsize::new(0), (), AtomicUsize::new(0)));
    
    let shared_details = Arc::new(BoundedSharedDetails::new(ArrayQueue::<T>::new(size), (), ()));

    let sender_count = Arc::new(());

    (Sender::new(&shared_details, &sender_count), Receiver::new(&shared_details, &sender_count))

}
