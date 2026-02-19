use std::sync::{Arc, Weak};

use crossbeam::queue::SegQueue;

use crate::{ChannelSharedDetails, SendResult, WakerPermitQueue};

use delegate::delegate;

use std::fmt::Debug;

pub struct Sender<T>
{

    shared_details: Arc<ChannelSharedDetails<SegQueue<T>, WakerPermitQueue>>,
    senders_count: Arc<()>,
    receivers_count: Weak<()>

}

impl<T> Sender<T>
{

    ///
    /// Create a new channel Sender object.
    /// 
    pub fn new(shared_details: &Arc<ChannelSharedDetails<SegQueue<T>, WakerPermitQueue>>, senders_count: Arc<()>, receivers_count: Weak<()>) -> Self
    {

        Self
        {

            shared_details: shared_details.clone(),
            senders_count: senders_count.clone(),
            receivers_count

        }

    }

    //Disabled

    /*
    pub fn try_send(&self, value: T) -> SendResult<T>
    {

        if self.receivers_count.strong_count() > 0
        {

            self.shared_details.message_queue_ref().push(value);

            self.shared_details.notifier_ref().add_permit();

            return Ok(());

        }

        Err(value)

    }
    */

    ///
    /// Sends a value, only if there are any receiver objects still existent.
    /// 
    /// Returns it in a Result::Err variant otherwise.
    /// 
    pub fn send(&self, value: T) -> SendResult<T>
    {

        let res = self.shared_details.notifier_ref().add_permit();

        match res
        {

            Some(_val) =>
            {

                self.shared_details.message_queue_ref().push(value);

                Ok(())

            }
            None =>
            {

                Err(value)

            }

        }

        /*
        if self.receivers_count.strong_count() > 0
        {

            self.shared_details.message_queue_ref().push(value);

            self.shared_details.notifier_ref().add_permit();

            return Ok(());

        }

        Err(value)
        */

    }

    ///
    /// Sends a value regardless of whether or not there are still any receiver objects that are instantiated.
    /// 
    pub fn send_regardless(&self, value: T)
    {

        self.shared_details.message_queue_ref().push(value);

        self.shared_details.notifier_ref().add_permit();

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
