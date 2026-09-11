use std::sync::{Arc, Weak};

use crossbeam_queue::SegQueue;

use crate::{ChannelSharedDetails, SendResult, WakerPermitQueue};

use delegate::delegate;

use std::fmt::Debug;

use super::WeakSender;

pub struct Sender<T>
{

    shared_details: Arc<SegQueue<T>>,
    senders_count: Arc<()>,
    receivers_count: Weak<()>

}

impl<T> Sender<T>
{

    ///
    /// Create a new channel Sender object.
    /// 
    pub fn new(shared_details: Arc<SegQueue<T>>, senders_count: Arc<()>, receivers_count: Weak<()>) -> Self
    {

        Self
        {

            shared_details,
            senders_count: senders_count.clone(),
            receivers_count

        }

    }

    ///
    /// Sends a value, only if there are any receiver objects still existent.
    /// 
    /// Returns it in a Result::Err variant otherwise.
    /// 
    pub async fn send(&self, value: T)
    {

        self.shared_details.push(value);

    }

    pub fn send_sync(&self, value: T)
    {

        self.shared_details.push(value);

    }


    delegate!
    {

        to self.shared_details
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

    pub fn downgrade(&self) -> WeakSender<T>
    {

        WeakSender::new(&self.shared_details, &self.senders_count, &self.receivers_count)

    }

    pub fn same_channel(&self, other: &Self) -> bool
    {

        Arc::ptr_eq(&self.shared_details, &other.shared_details) 

    }

    pub fn shared_details_ptr_addr(&self) -> usize
    {

        Arc::as_ptr(&self.shared_details).addr()

    }

    pub fn same_channel_receiver(&self, other: &super::Receiver<T>) -> bool
    {

        self.shared_details_ptr_addr() == other.shared_details_ptr_addr()

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

/*
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
*/
