use std::{sync::{Arc, Weak, atomic::AtomicBool}, task::Waker};

use crossbeam_queue::{ArrayQueue, SegQueue};

use super::Receiver;

use crate::{ChannelSharedDetails, ChannelSharedDetailsWithBothQueues, LimitedWakerPermitQueue};

pub struct WeakReceiver<T>
    where T: Unpin
{

    shared_details: Weak<ChannelSharedDetailsWithBothQueues<ArrayQueue<T>, SegQueue<Waker>, AtomicBool>>,
    senders_count: Weak<()>,
    receivers_count: Weak<()>

}

impl<T> WeakReceiver<T>
    where T: Unpin
{

    pub fn new(shared_details: &Arc<ChannelSharedDetailsWithBothQueues<ArrayQueue<T>, SegQueue<Waker>, AtomicBool>>, senders_count: &Weak<()>, receivers_count: &Arc<()>) -> Self
    {

        Self
        {

            shared_details: Arc::downgrade(shared_details),
            senders_count: senders_count.clone(),
            receivers_count: Arc::downgrade(receivers_count)

        }

    }
    
    pub fn upgrade(&self) -> Option<Receiver<T>>
    {

        if let Some(shared_details) = self.shared_details.upgrade() && let Some(receivers_count) = self.receivers_count.upgrade()
        {

            Some(Receiver::new(shared_details, self.senders_count.clone(), receivers_count))

        }
        else
        {

            None
            
        }

    }

    ///
    /// The total number of Receiver instances.
    /// 
    pub fn strong_count(&self) -> usize
    {

        self.receivers_count.strong_count()

    }

    ///
    /// The total number of potential Receiverr instances.
    /// 
    pub fn weak_count(&self) -> usize
    {

        self.receivers_count.weak_count()
        
    }

}
