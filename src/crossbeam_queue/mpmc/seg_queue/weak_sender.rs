use std::sync::{Arc, Weak};

use crossbeam_queue::SegQueue;

use super::Sender;

use crate::{ChannelSharedDetails, WakerPermitQueue};



pub struct WeakSender<T>
{

    shared_details: Weak<ChannelSharedDetails<SegQueue<T>, WakerPermitQueue>>,
    senders_count: Weak<()>,
    receivers_count: Weak<()>

}

impl<T> WeakSender<T>
{

    pub fn new(shared_details: &Arc<ChannelSharedDetails<SegQueue<T>, WakerPermitQueue>>, senders_count: &Arc<()>, receivers_count: &Weak<()>) -> Self
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