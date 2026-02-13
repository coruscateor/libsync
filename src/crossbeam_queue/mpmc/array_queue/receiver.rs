use std::sync::{Arc, Weak};

use crossbeam_queue::ArrayQueue;

use crate::{ChannelSharedDetails, ReceiveError, ReceiveResult, SendResult, LimitedWakerPermitQueue};

use delegate::delegate;

use std::fmt::Debug;

pub struct Receiver<T>
{

    shared_details: Arc<ChannelSharedDetails<ArrayQueue<T>, LimitedWakerPermitQueue>>,
    senders_count: Weak<()>,
    receivers_count: Arc<()>

}

impl<T> Receiver<T>
{

    ///
    /// Create a new channel Receiver object.
    /// 
    pub fn new(shared_details: Arc<ChannelSharedDetails<ArrayQueue<T>, LimitedWakerPermitQueue>>, senders_count: Weak<()>, receivers_count: Arc<()>) -> Self
    {

        Self
        {

            shared_details,
            senders_count,
            receivers_count

        }

    }

    ///
    /// Attempt to receive a value without waiting.
    /// 
    /// Returns an error if the channels queue is empty and there are no instantiated Senders detected.
    /// 
    pub fn try_recv(&self) -> ReceiveResult<T>
    {

        let res = self.shared_details.notifier_ref().remove_permit();

        match res
        {

            Some(val) =>
            {

                if val
                {

                    loop
                    {
                        
                        if let Some(message) = self.shared_details.message_queue_ref().pop()
                        {

                            return Ok(message);

                        }
                        else if self.senders_count.strong_count() == 0
                        {

                            return Err(ReceiveError::Closed);

                        }

                    }

                }

            }
            None =>
            {

                //The LimitedWakerPermitQueue is closed, try to take a value anyway.

                if let Some(message) = self.shared_details.message_queue_ref().pop()
                {

                    return Ok(message);

                }
                else
                {

                    return Err(ReceiveError::Closed);
                    
                }


            }

        }

        Err(ReceiveError::Empty)

    }

    ///
    /// Attempt to receive a value.
    /// 
    /// Returns an error if the channels queue is empty and there are no instantiated Senders detected.
    /// 
    pub async fn recv(&self) -> Result<T, ()> //ReceiveResult<T>
    {

        let res = self.shared_details.notifier_ref().decrement_permits_or_wait().await;

        match res
        {

            Ok(_) =>
            {

                loop
                {

                    if let Some(message) = self.shared_details.message_queue_ref().pop()
                    {

                        return Ok(message);

                    }
                    else if self.is_closed() //self.senders_count.strong_count() == 0
                    {

                        return Err(()); //Err(ReceiveError::NoSenders);

                    }
                    
                }

            }
            Err(_err) =>
            {

                if let Some(message) = self.shared_details.message_queue_ref().pop()
                {

                    return Ok(message);

                }

            }

        }

        Err(())

        //Err(ReceiveError::Empty)

    }

    delegate!
    {

        to self.shared_details.message_queue_ref()
        {
        
            ///
            /// Is the channel empty?
            /// 
            pub fn is_empty(&self) -> bool;

            ///
            /// How many messages are in the channels queue?
            /// 
            pub fn len(&self) -> usize;

            pub fn capacity(&self) -> usize;

            pub fn is_full(&self) -> bool;

        }

    }

    ///
    /// The total number of Receiver instances.
    /// 
    pub fn strong_count(&self) -> usize
    {

        Arc::strong_count(&self.receivers_count)

    }

    ///
    /// The total number of potential Receiver instances.
    /// 
    pub fn weak_count(&self) -> usize
    {

        Arc::weak_count(&self.receivers_count)
        
    }

    delegate!
    {

        to self.senders_count
        {

            ///
            /// The total number of Sender instances.
            /// 
            #[call(strong_count)]
            pub fn senders_strong_count(&self) -> usize;

            ///
            /// The total number of potential Sender instances.
            /// 
            #[call(weak_count)]
            pub fn senders_weak_count(&self) -> usize;

        }

    }

    pub fn is_closed(&self) -> bool
    {

        self.senders_strong_count() == 0

    }

    ///
    /// How many free queue elements do we have?
    /// 
    pub fn head_room(&self) -> usize
    {

        let queue_ref = self.shared_details.message_queue_ref();

        queue_ref.capacity() - queue_ref.len()

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
