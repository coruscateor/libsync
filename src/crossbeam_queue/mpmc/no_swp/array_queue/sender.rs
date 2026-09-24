use std::{collections::btree_map::Values, default, sync::{Arc, Weak, atomic::AtomicBool}, task::{Poll, Waker}, time::Duration};

use crossbeam_queue::{ArrayQueue, SegQueue};

use futures::executor::block_on;

use crate::{BoundedSendError, ChannelSharedDetails, ChannelSharedDetailsWithBothQueues, LimitedWakerPermitQueue, SendResult, TimeoutBoundedSendError}; //, auto_waker

use super::WeakSender;

use delegate::delegate;

use std::fmt::Debug;

use crate::{BoundedSendResult, EXPECTED_VALUE_NOT_FOUND_MESSAGE}; //AutoWaker, 

pub struct Sender<T>
    where T: Unpin
{

    shared_details: Arc<ChannelSharedDetailsWithBothQueues<ArrayQueue<T>, SegQueue<Waker>, AtomicBool>>,
    senders_count: Arc<()>,
    receivers_count: Weak<()>

}

impl<T> Sender<T>
    where T: Unpin
{

    ///
    /// Create a new channel Sender object.
    /// 
    pub fn new(shared_details: Arc<ChannelSharedDetailsWithBothQueues<ArrayQueue<T>, SegQueue<Waker>, AtomicBool>>, senders_count: Arc<()>, receivers_count: Weak<()>) -> Self
    {

        Self
        {

            shared_details,
            senders_count,
            receivers_count

        }

    }

    pub fn try_send(&self, value: T) -> BoundedSendResult<T>
    {

        if self.is_closed()
        {

            return Err(BoundedSendError::Closed(value));

        }

        match self.shared_details.message_queue.push(value)
        {

            Ok(_) =>
            {

                if let Some(waker) = self.shared_details.empty_queue.pop()
                {

                    waker.wake();

                }

                Ok(())

            }
            Err(value) =>
            {

                Err(BoundedSendError::Full(value))

            }
            
        }

    }

    ///
    /// Sends a value, only if there are any receiver objects still existent.
    /// 
    /// Returns it in a Result::Err variant otherwise.
    /// 
    pub async fn send<'a>(&'a self, value: T) -> SendFuture<'a, T>
    {

        SendFuture::new(self, value)

    }

    pub fn blocking_send(&self, value: T) -> SendResult<T>
    {

        let fut = SendFuture::new(self, value);

        block_on(fut)

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

        self.shared_details.is_closed()

        //self.receivers_strong_count() == 0

    }

    ///
    /// How many free queue elements do we have?
    /// 
    pub fn head_room(&self) -> usize
    {

        let queue_ref = &self.shared_details.message_queue;

        queue_ref.capacity() - queue_ref.len()

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
    where T: Unpin + Debug
{

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Sender").field("shared_details", &self.shared_details).field("senders_count", &self.senders_count).field("receivers_count", &self.receivers_count).finish()
    }
    
}

pub struct SendFuture<'a, T>
    where T: Unpin
{

    sender_ref: &'a Sender<T>,
    opt_value: Option<T>

}

impl<'a, T> SendFuture<'a, T>
    where T: Unpin
{

    pub fn new(sender_ref: &'a Sender<T>, value: T) -> Self
    {

        Self
        {

            sender_ref,
            opt_value: Some(value)

        }

    }

}

impl<'a, T> Future for SendFuture<'a, T>
    where T: Unpin
{

    type Output = SendResult<T>; //Result<(), SendError<T>>;

    fn poll(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> Poll<Self::Output>
    {

        let mut_self = self.get_mut();

        let value = mut_self.opt_value.take().expect(EXPECTED_VALUE_NOT_FOUND_MESSAGE);

        if mut_self.sender_ref.is_closed()
        {

            return Poll::Ready(SendResult::Err(value));

        }

        if let Err(value) = mut_self.sender_ref.shared_details.message_queue.push(value)
        {

            let waker = cx.waker().clone();

            //let auto_waker = AutoWaker::new(waker);

            if let Err(value) = mut_self.sender_ref.shared_details.message_queue.push(value)
            {

                if mut_self.sender_ref.is_closed()
                {

                    return Poll::Ready(Err(value));

                }

                //Cannot push the value

                mut_self.sender_ref.shared_details.full_queue.push(waker);

                mut_self.opt_value = Some(value);

                return Poll::Pending;

            }

        }

        if let Some(waker) = mut_self.sender_ref.shared_details.empty_queue.pop()
        {

            waker.wake();

        }

        Poll::Ready(SendResult::Ok(()))

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
