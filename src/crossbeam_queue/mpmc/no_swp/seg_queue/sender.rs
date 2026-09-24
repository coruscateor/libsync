use std::{sync::{Arc, Weak, atomic::AtomicBool}, task::Waker};

use crossbeam_queue::SegQueue;

use delegate::delegate;

use std::fmt::Debug;

use super::WeakSender;

use crate::{ChannelSharedDetailsWithEmptyQueue, SendResult};

pub struct Sender<T>
    where T: Unpin
{

    shared_details: Arc<ChannelSharedDetailsWithEmptyQueue<SegQueue<T>, SegQueue<Waker>, AtomicBool>>,
    senders_count: Arc<()>,
    receivers_count: Weak<()>

}

impl<T> Sender<T>
    where T: Unpin
{

    ///
    /// Create a new channel Sender object.
    /// 
    pub fn new(shared_details: Arc<ChannelSharedDetailsWithEmptyQueue<SegQueue<T>, SegQueue<Waker>, AtomicBool>>, senders_count: Arc<()>, receivers_count: Weak<()>) -> Self
    {

        Self
        {

            shared_details,
            senders_count: senders_count.clone(),
            receivers_count

        }

    }

    pub fn try_send(&self, value: T) -> SendResult<T>
    {

        if self.is_closed()
        {

            return SendResult::Err(value);

        }

        self.shared_details.message_queue.push(value);

        if let Some(waker) = self.shared_details.empty_queue.pop()
        {

            waker.wake();

        }

        SendResult::Ok(())

    }

    ///
    /// Sends a value, only if there are any receiver objects still existent.
    /// 
    /// Returns it in a Result::Err variant otherwise.
    /// 
    pub fn send(&self, value: T)
    {

        self.shared_details.message_queue.push(value);

        if let Some(waker) = self.shared_details.empty_queue.pop()
        {

            waker.wake();

        }

    }

    delegate!
    {

        to self.shared_details.message_queue
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

        self.shared_details.is_closed()

        //self.receivers_strong_count() == 0

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
    where T: Unpin
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
    where T: Debug + Unpin
{

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Sender").field("shared_details", &self.shared_details).field("senders_count", &self.senders_count).field("receivers_count", &self.receivers_count).finish()
    }
    
}

impl<T> Drop for Sender<T>
    where T: Unpin
{

    fn drop(&mut self)
    {

        if self.strong_count() == 1 && !self.is_closed()
        {

            self.shared_details.set_closed();

            while let Some(waker) = self.shared_details.empty_queue.pop()
            {

                waker.wake();

            }
            
        }

    }

}
