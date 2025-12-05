use std::sync::{Arc, Weak};

use crossbeam::queue::SegQueue;

use crate::{ChannelSharedDetails, ReceiveError, ReceiveResult, SendResult, WakerPermitQueue};

use delegate::delegate;

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

            if self.senders_count.strong_count() > 0
            {

                return Err(ReceiveError::NoSenders);

            }

            Err(ReceiveError::Empty)

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


