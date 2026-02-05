use std::sync::Arc;

use crossbeam_queue::SegQueue;

use crate::{ChannelSharedDetails, SingleWakerMultiPermit};

use super::{Sender, Receiver};

pub fn channel<T>() -> (Sender<T>, Receiver<T>)
{

    let shared_details = Arc::new(ChannelSharedDetails::new(SegQueue::<T>::new(), SingleWakerMultiPermit::new()));

    let senders_count = Arc::new(());

    let weak_senders_count = Arc::downgrade(&senders_count);

    let receivers_count = Arc::new(());

    let weak_receivers_count = Arc::downgrade(&receivers_count);

    let sender = Sender::new(&shared_details, senders_count, weak_receivers_count);

    let receiver = Receiver::new(shared_details, weak_senders_count, receivers_count);

    (sender, receiver)

}
