use std::{collections::btree_map::Values, default, sync::{Arc, Weak}};

use crossbeam_queue::ArrayQueue;

use crate::{BoundedSendError, ChannelSharedDetails, LimitedWakerPermitQueue, SendResult};

use delegate::delegate;

use std::fmt::Debug;

pub struct Sender<T>
{

    shared_details: Arc<ChannelSharedDetails<ArrayQueue<T>, LimitedWakerPermitQueue>>,
    senders_count: Arc<()>,
    receivers_count: Weak<()>

}

impl<T> Sender<T>
{

    ///
    /// Create a new channel Sender object.
    /// 
    pub fn new(shared_details: &Arc<ChannelSharedDetails<ArrayQueue<T>, LimitedWakerPermitQueue>>, senders_count: Arc<()>, receivers_count: Weak<()>) -> Self
    {

        Self
        {

            shared_details: shared_details.clone(),
            senders_count, //: senders_count.clone(),
            receivers_count

        }

    }

    pub fn try_send(&self, value: T) -> Result<(), BoundedSendError<T>>
    {

        match self.shared_details.notifier_ref().add_permit()
        {

            Some(true) =>
            {

                if let Err(mut val) = self.shared_details.message_queue_ref().push(value)
                {

                    //This shouldn't happen... that often.

                    loop
                    {

                        if let Err(val_again) = self.shared_details.message_queue_ref().push(val)
                        {

                            val = val_again;

                            if self.shared_details.notifier_ref().is_closed()
                            {

                                return Err(BoundedSendError::Closed(val));

                            }

                        }
                        else
                        {

                            break;
                            
                        }
                
                        
                    }

                }

                Ok(())

            }
            Some(false) =>
            {

                Err(BoundedSendError::Full(value))

            }
            None =>
            {

                Err(BoundedSendError::Closed(value))

            }
            
        } 

    }

    ///
    /// Sends a value, only if there are any receiver objects still existent.
    /// 
    /// Returns it in a Result::Err variant otherwise.
    /// 
    pub async fn send(&self, value: T) -> SendResult<T>
    {

        let res = self.shared_details.notifier_ref().increment_permits_or_wait().await;

        match res
        {

            Ok(_) =>
            {

                if let Err(mut val) = self.shared_details.message_queue_ref().push(value)
                {

                    // This should hopefully never happen

                    loop
                    {

                        if let Err(val_again) = self.shared_details.message_queue_ref().push(val)
                        {

                            if self.is_closed()
                            {

                                return Err(val_again);

                            }

                            val = val_again;
                            
                        }
                        else
                        {

                            break;
                            
                        }
                        
                    }

                }

                Ok(())

            }
            Err(_err) =>
            {

                Err(value)

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

            pub fn capacity(&self) -> usize;

            pub fn is_full(&self) -> bool;

        }

    }

    ///
    /// The total number of Sender instances.
    /// 
    pub fn strong_count(&self) -> usize
    {

        Arc::strong_count(&self.senders_count)

    }

    ///
    /// The total number of potential Sender instances.
    /// 
    pub fn weak_count(&self) -> usize
    {

        Arc::weak_count(&self.senders_count)
        
    }

    delegate!
    {

        to self.receivers_count
        {

            ///
            /// The total number of Receiver instances.
            /// 
            #[call(strong_count)]
            pub fn receivers_strong_count(&self) -> usize;

            ///
            /// The total number of potential Receiver instances.
            /// 
            #[call(weak_count)]
            pub fn receivers_weak_count(&self) -> usize;

        }

    }

    pub fn is_closed(&self) -> bool
    {

        self.receivers_strong_count() == 0

    }

    ///
    /// How many free queue elements do we have?
    /// 
    pub fn head_room(&self) -> usize
    {

        let queue_ref = self.shared_details.message_queue_ref();

        queue_ref.capacity() - queue_ref.len()

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

impl<T> Debug for Sender<T>
    where T: Debug
{

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Sender").field("shared_details", &self.shared_details).field("senders_count", &self.senders_count).field("receivers_count", &self.receivers_count).finish()
    }
    
}

impl<T> Drop for Sender<T>
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
