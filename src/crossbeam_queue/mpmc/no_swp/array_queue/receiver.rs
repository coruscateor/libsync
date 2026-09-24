use std::{sync::{Arc, Weak, atomic::AtomicBool}, task::{Poll, Waker}, time::Duration};

use crossbeam_queue::{ArrayQueue, SegQueue};

use futures::executor::block_on;

use crate::{ChannelSharedDetailsWithBothQueues, ReceiveResult};

use super::{Sender, WeakReceiver};

use delegate::delegate;

use std::fmt::Debug;

pub struct Receiver<T>
    where T: Unpin
{

    shared_details: Arc<ChannelSharedDetailsWithBothQueues<ArrayQueue<T>, SegQueue<Waker>, AtomicBool>>,
    senders_count: Weak<()>,
    receivers_count: Arc<()>

}

impl<T> Receiver<T>
    where T: Unpin
{

    ///
    /// Create a new channel Receiver object.
    /// 
    pub fn new(shared_details: Arc<ChannelSharedDetailsWithBothQueues<ArrayQueue<T>, SegQueue<Waker>, AtomicBool>>, senders_count: Weak<()>, receivers_count: Arc<()>) -> Self
    {

        Self
        {

            shared_details,
            senders_count,
            receivers_count

        }

    }

    ///
    /// Try to receive a value immediately.
    /// 
    pub fn try_recv(&self) -> ReceiveResult<T>
    {

        if self.shared_details.empty_queue.len() > 0
        {

            if self.is_closed()
            {

                return Err(crate::ReceiveError::Closed);

            }

            return Err(crate::ReceiveError::Empty);

        }

        if let Some(value) = self.shared_details.message_queue.pop()
        {

            if let Some(waker) = self.shared_details.full_queue.pop()
            {

                waker.wake();

            }

            return Ok(value);

        }

        if self.is_closed()
        {

            return Err(crate::ReceiveError::Closed);

        }

        Err(crate::ReceiveError::Empty)

    }

    ///
    /// Attempt to receive a value.
    /// 
    /// Returns an error if the channels queue is empty and there are no instantiated Senders detected.
    /// 
    pub fn recv<'a>(&'a self) -> RecvFuture<'a, T>
    {

        RecvFuture::new(self)

    }

    pub fn blocking_recv(&self) -> Result<T, ()>
    {

        let fut = RecvFuture::new(self);

        block_on(fut)

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

        self.shared_details.is_closed()

        //self.senders_strong_count() == 0

    }

    ///
    /// How many free queue elements do we have?
    /// 
    pub fn head_room(&self) -> usize
    {

        let queue_ref = &self.shared_details.message_queue;

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

    pub fn same_channel_sender(&self, other: &Sender<T>) -> bool
    {

        self.shared_details_ptr_addr() == other.shared_details_ptr_addr()

    }

}

impl<T> Clone for Receiver<T>
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

impl<T> Debug for Receiver<T>
    where T: Unpin + Debug
{

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Receiver").field("shared_details", &self.shared_details).field("senders_count", &self.senders_count).field("receivers_count", &self.receivers_count).finish()
    }

}

impl<T> Drop for Receiver<T>
    where T: Unpin
{

    fn drop(&mut self)
    {

        if self.strong_count() == 1 && !self.is_closed()
        {

            self.shared_details.set_closed();

            //Engage free-for-all mode.

            while let Some(waker) = self.shared_details.empty_queue.pop()
            {

                waker.wake();

            }

            while let Some(waker) = self.shared_details.full_queue.pop()
            {

                waker.wake();

            }

        }
    
    }

}

pub struct RecvFuture<'a, T>
    where T: Unpin
{

    receiver_ref: &'a Receiver<T>

}

impl<'a, T> RecvFuture<'a, T>
    where T: Unpin
{

    pub fn new(receiver_ref: &'a Receiver<T>) -> Self
    {

        Self
        {

            receiver_ref

        }

    }

}

impl<'a, T> Future for RecvFuture<'a, T>
    where T: Unpin
{

    type Output = Result<T, ()>;
    
    fn poll(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<Self::Output>
    {

        let my_waker;

        if self.receiver_ref.shared_details.empty_queue.len() == 0
        {

            if let Some(value) = self.receiver_ref.shared_details.message_queue.pop()
            {

                if let Some(waker) = self.receiver_ref.shared_details.full_queue.pop()
                {

                    waker.wake();

                }

                return Poll::Ready(Ok(value));

            }

            my_waker = cx.waker().clone();

            //let auto_waker = AutoWaker::new(waker);

            if let Some(value) = self.receiver_ref.shared_details.message_queue.pop()
            {

                if let Some(waker) = self.receiver_ref.shared_details.full_queue.pop()
                {

                    waker.wake();

                }

                return Poll::Ready(Ok(value));

            }

            if self.receiver_ref.is_closed()
            {

                return Poll::Ready(Err(()));

            }

        }
        else
        {

            my_waker = cx.waker().clone();
            
        }

        self.receiver_ref.shared_details.empty_queue.push(my_waker);

        /*
        if self.receiver_ref.is_closed()
        {

            return Poll::Ready(Err(()));

        }
        */

        Poll::Pending

    }

}
