use std::{sync::{Arc, Weak}, time::Duration};

use crossbeam_queue::ArrayQueue;

use super::WeakReceiver;

use delegate::delegate;

use std::fmt::Debug;

pub struct Receiver<T>
{

    shared_details: Arc<ArrayQueue<T>>,
    senders_count: Weak<()>,
    receivers_count: Arc<()>

}

impl<T> Receiver<T>
{

    ///
    /// Create a new channel Receiver object.
    /// 
    pub fn new(shared_details: Arc<ArrayQueue<T>>, senders_count: Weak<()>, receivers_count: Arc<()>) -> Self
    {

        Self
        {

            shared_details,
            senders_count,
            receivers_count

        }

    }

    ///
    /// Attempt to receive a value.
    /// 
    /// Returns an error if the channels queue is empty and there are no instantiated Senders detected.
    /// 
    pub async fn recv(&self) -> Option<T>
    {

        self.shared_details.pop()

    }

    pub fn recv_sync(&self) -> Option<T>
    {

        self.shared_details.pop()

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

        let queue_ref = &self.shared_details;

        queue_ref.capacity() - queue_ref.len()

    }

    pub fn downgrade(&self) -> WeakReceiver<T>
    {

        WeakReceiver::new(&self.shared_details, &self.senders_count, &self.receivers_count)

    }

    pub fn same_channel(&self, other: &Self) -> bool
    {

        Arc::ptr_eq(&self.shared_details, &other.shared_details) 

    }

    pub fn shared_details_ptr_addr(&self) -> usize
    {

        Arc::as_ptr(&self.shared_details).addr()

    }

    pub fn same_channel_sender(&self, other: &super::Sender<T>) -> bool
    {

        self.shared_details_ptr_addr() == other.shared_details_ptr_addr()

    }

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

/*
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
*/
