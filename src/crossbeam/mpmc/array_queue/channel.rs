use std::sync::Arc;

use crossbeam::queue::ArrayQueue;

use crate::{ChannelSharedDetails, LimitedWakerPermitQueue};

use super::{Sender, Receiver};

pub fn channel<T>(size: usize)  -> (Sender<T>, Receiver<T>)
{

    let queue = ArrayQueue::<T>::new(size);

    let lwpq = LimitedWakerPermitQueue::new(size);

    let shared_details = Arc::new(ChannelSharedDetails::new(queue, lwpq));

    let senders_count = Arc::new(());

    let weak_sender_count = Arc::downgrade(&senders_count);

    let receivers_count = Arc::new(());

    let weak_receivers_count = Arc::downgrade(&receivers_count);

    let sender = Sender::new(&shared_details, senders_count, weak_receivers_count);

    let receiver = Receiver::new(shared_details, weak_sender_count, receivers_count);

    (sender, receiver)

}
