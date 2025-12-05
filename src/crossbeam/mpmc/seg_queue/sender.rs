use std::sync::{Arc, Weak};

use crossbeam::queue::SegQueue;

use crate::{ChannelSharedDetails, SendResult, WakerPermitQueue};

use delegate::delegate;

pub struct Sender<T>
{

    shared_details: Arc<ChannelSharedDetails<SegQueue<T>, WakerPermitQueue>>,
    senders_count: Arc<()>,
    receivers_count: Weak<()>

}

impl<T> Sender<T>
{

    pub fn new(shared_details: &Arc<ChannelSharedDetails<SegQueue<T>, WakerPermitQueue>>, senders_count: Arc<()>, receivers_count: &Arc<()>) -> Self
    {

        Self
        {

            shared_details: shared_details.clone(),
            senders_count: senders_count.clone(),
            receivers_count: Arc::downgrade(receivers_count)

        }

    }

    pub fn try_send(&self, value: T) -> SendResult<T>
    {

        if self.receivers_count.strong_count() > 0
        {

            self.shared_details.notifier_ref().add_permit();

            self.shared_details.message_queue_ref().push(value);

            return Ok(());

        }

        Err(value)

    }

    pub fn send(&self, value: T) -> SendResult<T>
    {

        if self.receivers_count.strong_count() > 0
        {

            self.shared_details.message_queue_ref().push(value);

            self.shared_details.notifier_ref().add_permit();

            return Ok(());

        }

        Err(value)

    }

    delegate!
    {

        to self.shared_details.message_queue_ref()
        {
        
            pub fn is_empty(&self) -> bool;

            pub fn len(&self) -> usize;

        }

    }

    pub fn strong_count(&self) -> usize
    {

        Arc::strong_count(&self.senders_count)

    }

    pub fn weak_count(&self) -> usize
    {

        Arc::weak_count(&self.senders_count)
        
    }

    delegate!
    {

        to self.receivers_count
        {

            #[call(strong_count)]
            pub fn receiver_strong_count(&self) -> usize;

            #[call(weak_count)]
            pub fn receiver_weak_count(&self) -> usize;

        }

    }


}

impl<T> Clone for Sender<T>
{

    fn clone(&self) -> Self
    {

        Self
        { 
            
            shared_details: self.shared_details.clone(),
            senders_count: self.senders_count.clone(),
            receivers_count: self.receivers_count.clone()
        
        }

    }

}

