use std::collections::hash_map::Entry;
use std::fmt::Debug;

use inc_dec::{IncDecSelf, IntIncDecSelf};
use pastey::paste;

use accessorise::impl_val_getter;

use std::collections::{HashMap, VecDeque};

use std::task::{Context, Poll, Waker};

use crate::QueuedWaker;

///
/// For containing the objects that are shared between the sender and the receiver parts of a channel.
/// 
pub struct ChannelSharedDetails<T>
{

    pub message_queue: VecDeque<T>,
    pub when_empty_waker_queue: VecDeque<QueuedWaker>,
    pub when_full_waker_queue: VecDeque<QueuedWaker>,
    pub is_closed: bool,
    pub latest_id: usize,
    pub active_ids: HashMap<usize, bool>,
    capacity: usize

}

impl<T> ChannelSharedDetails<T>
{

    pub fn new(capacity: usize, when_empty_waker_queue: VecDeque<QueuedWaker>, when_full_waker_queue: VecDeque<QueuedWaker>, active_ids: HashMap<usize, bool>) -> Self
    {

        Self
        {

            message_queue: VecDeque::with_capacity(capacity),
            when_empty_waker_queue,
            when_full_waker_queue,
            is_closed: false,
            latest_id: 0,
            active_ids,
            capacity

        }

    }

    impl_val_getter!(capacity, usize);

    pub fn try_pop(&mut self, waker_id: usize) -> Option<T>
    {

        if let Entry::Occupied(entry) = self.active_ids.entry(waker_id)
        //if let Some(shouldve_awoken) = self.active_ids.get(&waker_id)
        {

            let shouldve_awoken = entry.get();

            if *shouldve_awoken
            {

                let opt_front = self.message_queue.pop_front();

                if let Some(front) = opt_front
                {

                    entry.remove_entry();

                    return Some(front);

                    //self.active_ids.remove(&waker_id);

                    //return Some(Poll::Ready(Ok(front)));

                }

            }

            //return Some(Poll::Pending);

        }

        None

        //Poll::Pending

    }

    pub fn push_waker(&mut self, cx: &Context<'_>) -> usize
    {

        let mut id;

        loop
        {
            
            id = self.latest_id.wpp();

            let entry = self.active_ids.entry(id);

            if let Entry::Vacant(vacant_entry) = entry
            {

                vacant_entry.insert(false);

                break;

            } 

        }

        let waker = cx.waker().clone();

        let queued_waker = QueuedWaker::new(waker, id);

        self.when_empty_waker_queue.push_back(queued_waker);

        id

    }

    pub fn remove_waker(&mut self, id: usize)
    {

        self.active_ids.remove(&id);

        let mut index = 0;

        let mut index_found = false;

        //Remove the queued waker.

        for item in self.when_empty_waker_queue.iter()
        {

            if id == item.id()
            {

                index_found = true;

                break;

            }  

            index.pp();
            
        }

        if index_found
        {

            self.when_empty_waker_queue.remove(index);

        }

    }

    //impl_ref_getter!(message_queue, VecDeque<T>);

    //impl_ref_getter!(notifier, N);

}

impl<T> Debug for ChannelSharedDetails<T>
    where T: Debug,
{

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ChannelSharedDetails").field("message_queue", &self.message_queue).field("when_empty_waker_queue", &self.when_empty_waker_queue).field("is_closed", &self.is_closed).field("active_ids", &self.active_ids).finish()
    }

}
