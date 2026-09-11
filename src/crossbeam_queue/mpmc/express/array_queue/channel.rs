use std::sync::Arc;

use crossbeam_queue::ArrayQueue;

use super::{Sender, Receiver};

pub fn channel<T>(size: usize) -> (Sender<T>, Receiver<T>)
{

    let queue = ArrayQueue::<T>::new(size);

    let shared_details = Arc::new(queue);

    let senders_count = Arc::new(());

    let weak_sender_count = Arc::downgrade(&senders_count);

    let receivers_count = Arc::new(());

    let weak_receivers_count = Arc::downgrade(&receivers_count);

    let sender = Sender::new(shared_details.clone(), senders_count, weak_receivers_count);

    let receiver = Receiver::new(shared_details, weak_sender_count, receivers_count);

    (sender, receiver)

}
