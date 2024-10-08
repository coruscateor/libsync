use std::sync::{atomic::AtomicUsize, Arc};

use crossbeam::queue::SegQueue;

use crate::{BoundedSharedDetails, SharedDetails}; //crossbeam::mpmc::tokio::ChannelSemaphore, 

use tokio::sync::Notify;

use super::{Sender, Receiver};

use crate::tokio_helpers::SemaphoreController;

pub fn channel<T>() -> (Sender<T>, Receiver<T>)
{

    let shared_details = Arc::new(SharedDetails::new(SegQueue::<T>::new(), SemaphoreController::new()));  //ChannelSemaphore::new())); //Notify::new()));

    let sender_count = Arc::new(());

    let weak_sender_count = Arc::downgrade(&sender_count);

    let receiver_count = Arc::new(());

    let sender = Sender::new(&shared_details, sender_count, &receiver_count);

    let receiver = Receiver::new(shared_details, weak_sender_count, receiver_count);

    (sender, receiver)

}

