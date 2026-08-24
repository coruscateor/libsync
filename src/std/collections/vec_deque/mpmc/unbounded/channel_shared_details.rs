use std::fmt::Debug;

use pastey::paste;

use accessorise::impl_ref_getter;

use std::collections::{HashMap, VecDeque};

use std::task::Waker;

use crate::QueuedWaker;

///
/// For containing the objects that are shared between the sender and the receiver parts of a channel.
/// 
pub struct ChannelSharedDetails<T>
{

    pub message_queue: VecDeque<T>,
    pub waker_queue: VecDeque<QueuedWaker>,
    pub is_closed: bool,
    pub active_ids: HashMap<usize, bool>

}

impl<T> ChannelSharedDetails<T>
{

    pub fn new(message_queue: VecDeque<T>, waker_queue: VecDeque<QueuedWaker>, active_ids: HashMap<usize, bool>) -> Self
    {

        Self
        {

            message_queue,
            waker_queue,
            is_closed: false,
            active_ids

        }

    }

    //impl_ref_getter!(message_queue, VecDeque<T>);

    //impl_ref_getter!(notifier, N);

}

impl<T> Debug for ChannelSharedDetails<T>
    where T: Debug,
{

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ChannelSharedDetails").field("message_queue", &self.message_queue).field("waker_queue", &self.waker_queue).finish()
    }
    
}
