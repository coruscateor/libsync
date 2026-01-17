use std::sync::{Arc, Weak};

use crossbeam::queue::SegQueue;

use crate::{ChannelSharedDetails, ReceiveError, ReceiveResult, SendResult, WakerPermitQueue};

use delegate::delegate;

use std::fmt::Debug;

pub struct Receiver<T>
{

    shared_details: Arc<ChannelSharedDetails<SegQueue<T>, WakerPermitQueue>>,
    senders_count: Weak<()>,
    receivers_count: Arc<()>

}

impl<T> Receiver<T>
{

    pub fn new(shared_details: Arc<ChannelSharedDetails<SegQueue<T>, WakerPermitQueue>>, senders_count: Weak<()>, receivers_count: Arc<()>) -> Self
    {

        Self
        {

            shared_details,
            senders_count,
            receivers_count

        }

    }

    pub async fn recv(&self) -> ReceiveResult<T>
    {

        let _ = self.shared_details.notifier_ref().decrement_permits_or_wait().await;

        if let Some(message) = self.shared_details.message_queue_ref().pop() //.expect("Permits must match message count");
        {

            Ok(message)

        }
        else
        {

            if self.senders_count.strong_count() == 0
            {

                return Err(ReceiveError::NoSenders);

            }

            Err(ReceiveError::Empty)

        }

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

        Arc::strong_count(&self.receivers_count)

    }

    pub fn weak_count(&self) -> usize
    {

        Arc::weak_count(&self.receivers_count)
        
    }

    delegate!
    {

        to self.senders_count
        {

            #[call(strong_count)]
            pub fn senders_strong_count(&self) -> usize;

            #[call(weak_count)]
            pub fn senders_weak_count(&self) -> usize;

        }

    }

    //recv_or_timeout

}

impl<T> Clone for Receiver<T>
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

impl<T> Debug for Receiver<T>
    where T: Debug
{

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Receiver").field("shared_details", &self.shared_details).field("senders_count", &self.senders_count).field("receivers_count", &self.receivers_count).finish()
    }
    
}

impl<T> Drop for Receiver<T>
{

    fn drop(&mut self)
    {

        if self.strong_count() == 1
        {

            //Engage free-for-all mode.

            self.shared_details.notifier_ref().close();

        }
    
    }

}
