use std::sync::{Arc, Weak};

use crossbeam_queue::SegQueue;

use crate::PreferredMutexType;

use super::Sender;

use super::ChannelSharedDetails;

pub struct WeakSender<T>
    where T: Unpin
{

    shared_details: Weak<PreferredMutexType<ChannelSharedDetails<T>>>,
    senders_count: Weak<()>,
    receivers_count: Weak<()>

}

impl<T> WeakSender<T>
    where T: Unpin
{

    pub fn new(shared_details: &Arc<PreferredMutexType<ChannelSharedDetails<T>>>, senders_count: &Arc<()>, receivers_count: Weak<()>) -> Self
    {

        Self
        {

            shared_details: Arc::downgrade(shared_details),
            senders_count: Arc::downgrade(senders_count),
            receivers_count: receivers_count

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

    pub fn receivers_strong_count(&self) -> usize
    {

        self.receivers_count.strong_count()

    }

    pub fn receivers_weak_count(&self) -> usize
    {

        self.receivers_count.weak_count()
        
    }

}