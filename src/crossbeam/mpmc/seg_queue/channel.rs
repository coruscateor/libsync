use std::sync::Arc;

use crossbeam::queue::SegQueue;

use crate::{ChannelSharedDetails, WakerPermitQueue};

use super::{Sender, Receiver};

pub fn channel<T>() -> (Sender<T>, Receiver<T>)
{

    let shared_details = Arc::new(ChannelSharedDetails::new(SegQueue::<T>::new(), WakerPermitQueue::new()));

    let sender_count = Arc::new(());

    let weak_sender_count = Arc::downgrade(&sender_count);

    let receiver_count = Arc::new(());

    let sender = Sender::new(&shared_details, sender_count, &receiver_count);

    let receiver = Receiver::new(shared_details, weak_sender_count, receiver_count);

    (sender, receiver)

}