use std::{sync::{Arc, Weak, atomic::AtomicBool}, task::Waker};

use crossbeam_queue::{ArrayQueue, SegQueue};

use crate::ChannelSharedDetailsWithBothQueues;

use super::Sender;

pub struct WeakSender<T>
    where T: Unpin
{

    shared_details: Weak<ChannelSharedDetailsWithBothQueues<ArrayQueue<T>, SegQueue<Waker>, AtomicBool>>,
    senders_count: Weak<()>,
    receivers_count: Weak<()>

}

impl<T> WeakSender<T>
    where T: Unpin
{

    pub fn new(shared_details: &Arc<ChannelSharedDetailsWithBothQueues<ArrayQueue<T>, SegQueue<Waker>, AtomicBool>>, senders_count: &Arc<()>, receivers_count: &Weak<()>) -> Self
    {

        Self
        {

            shared_details: Arc::downgrade(shared_details),
            senders_count: Arc::downgrade(senders_count),
            receivers_count: receivers_count.clone()

        }

    }
    
    pub fn upgrade(&self) -> Option<Sender<T>>
    {

        if let Some(shared_details) = self.shared_details.upgrade() && let Some(senders_count) = self.senders_count.upgrade()
        {

            Some(Sender::new(shared_details, senders_count, self.receivers_count.clone()))

        }
        else
        {

            None
            
        }

    }

    ///
    /// The total number of Sender instances.
    /// 
    pub fn strong_count(&self) -> usize
    {

        self.senders_count.strong_count()

    }

    ///
    /// The total number of potential Sender instances.
    /// 
    pub fn weak_count(&self) -> usize
    {

        self.senders_count.weak_count()
        
    }

}