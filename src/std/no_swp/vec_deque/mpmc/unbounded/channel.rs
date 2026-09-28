use std::sync::Arc;

use crate::PreferredMutexType;

use super::ChannelSharedDetails;

use super::{Sender, Receiver};

use std::collections::{HashMap, VecDeque};

pub fn channel<T>() -> (Sender<T>, Receiver<T>)
{

    let shared_details = Arc::new(PreferredMutexType::new(ChannelSharedDetails::new(VecDeque::new(), VecDeque::new())));

    let senders_count = Arc::new(());

    let weak_senders_count = Arc::downgrade(&senders_count);

    let receivers_count = Arc::new(());

    let weak_receivers_count = Arc::downgrade(&receivers_count);

    let sender = Sender::new(shared_details.clone(), senders_count, weak_receivers_count);

    let receiver = Receiver::new(shared_details, weak_senders_count, receivers_count);

    (sender, receiver)

}

pub fn channel_with_capacities<T>(message_queue_capacity: usize, empty_waker_queue_capacities: usize) -> (Sender<T>, Receiver<T>)
{

    let shared_details = Arc::new(PreferredMutexType::new(ChannelSharedDetails::new(VecDeque::with_capacity(message_queue_capacity), VecDeque::with_capacity(empty_waker_queue_capacities))));

    let senders_count = Arc::new(());

    let weak_senders_count = Arc::downgrade(&senders_count);

    let receivers_count = Arc::new(());

    let weak_receivers_count = Arc::downgrade(&receivers_count);

    let sender = Sender::new(shared_details.clone(), senders_count, weak_receivers_count);

    let receiver = Receiver::new(shared_details, weak_senders_count, receivers_count);

    (sender, receiver)

}
