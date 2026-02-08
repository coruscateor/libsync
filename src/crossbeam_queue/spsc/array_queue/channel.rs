use std::sync::Arc;

use crossbeam_queue::ArrayQueue;

use crate::{ChannelSharedDetails, LimitedSingleWakerMultiPermit};

use super::{Sender, Receiver};

pub fn channel<T>(size: usize) -> (Sender<T>, Receiver<T>)
{

    let shared_details = Arc::new(ChannelSharedDetails::new(ArrayQueue::<T>::new(size), LimitedSingleWakerMultiPermit::new(size)));

    let senders_count = Arc::new(());

    let weak_senders_count = Arc::downgrade(&senders_count);

    let receivers_count = Arc::new(());

    let weak_receivers_count = Arc::downgrade(&receivers_count);

    let sender = Sender::new(&shared_details, senders_count, weak_receivers_count);

    let receiver = Receiver::new(shared_details, weak_senders_count, receivers_count);

    (sender, receiver)

}
