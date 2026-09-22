use std::sync::Arc;

use crate::PreferredMutexType;

use super::ChannelSharedDetails;

use super::{Sender, Receiver};

use std::collections::{HashMap, VecDeque};

pub fn channel<T>(capacity: usize) -> (Sender<T>, Receiver<T>)
    where T: Unpin
{

    let shared_details = Arc::new(PreferredMutexType::new(ChannelSharedDetails::new(capacity, VecDeque::new(), VecDeque::new())));

    let senders_count = Arc::new(());

    let weak_senders_count = Arc::downgrade(&senders_count);

    let receivers_count = Arc::new(());

    let weak_receivers_count = Arc::downgrade(&receivers_count);

    let sender = Sender::new(shared_details.clone(), senders_count, weak_receivers_count);

    let receiver = Receiver::new(shared_details, weak_senders_count, receivers_count);

    (sender, receiver)

}

pub fn channel_with_capacities<T>(capacity: usize, waker_queue_capacities: usize) -> (Sender<T>, Receiver<T>)
    where T: Unpin
{

    let shared_details = Arc::new(PreferredMutexType::new(ChannelSharedDetails::new(capacity, VecDeque::with_capacity(waker_queue_capacities), VecDeque::with_capacity(waker_queue_capacities))));

    let senders_count = Arc::new(());

    let weak_senders_count = Arc::downgrade(&senders_count);

    let receivers_count = Arc::new(());

    let weak_receivers_count = Arc::downgrade(&receivers_count);

    let sender = Sender::new(shared_details.clone(), senders_count, weak_receivers_count);

    let receiver = Receiver::new(shared_details, weak_senders_count, receivers_count);

    (sender, receiver)

}
