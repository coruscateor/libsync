use std::sync::{Arc, Weak};

use crossbeam_queue::SegQueue;

use crate::{ChannelSharedDetails, ReceiveError, ReceiveResult, SendResult, SingleWakerMultiPermit};

use delegate::delegate;

use std::fmt::Debug;

pub struct Receiver<T>
{

    shared_details: Arc<ChannelSharedDetails<SegQueue<T>, SingleWakerMultiPermit>>,
    senders_count: Weak<()>,
    receivers_count: Arc<()>

}

impl<T> Receiver<T>
{

    ///
    /// Create a new channel Receiver object.
    /// 
    pub fn new(shared_details: Arc<ChannelSharedDetails<SegQueue<T>, SingleWakerMultiPermit>>, senders_count: Weak<()>, receivers_count: Arc<()>) -> Self
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
    pub async fn try_recv(&self) -> ReceiveResult<T>
    {

        let res = self.shared_details.notifier_ref().remove_permit();

        if let Some(val) = res
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
        else
        {

            if let Some(message) = self.shared_details.message_queue_ref().pop()
            {

                return Ok(message);

            }
            else
            {

                return Err(ReceiveError::Closed);
                
            }

        }

        Err(ReceiveError::Empty)

    }

    ///
    /// Attempt to receive a value.
    /// 
    /// Returns an error if the channels queue is empty and there are no instantiated Senders detected.
    /// 
    pub async fn recv(&self) -> ReceiveResult<T>
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
                    else
                    {

                        if self.senders_count.strong_count() == 0
                        {

                            return Err(ReceiveError::Closed);

                        }

                        return Err(ReceiveError::Empty);

                    }

                }

            }
            Err(_) =>
            {

                Err(ReceiveError::Closed)

            }

        }

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

    //recv_or_timeout

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
