#[cfg(feature="tokio")]
use std::time::Duration;
use std::{sync::{Arc, Weak}, task::Poll};

#[cfg(feature="use_std_sync")]
use crate::get_mg;

use super::ChannelSharedDetails;

use crate::{BoundedSendError, PreferredMutexType};

use delegate::delegate;
use futures::executor::block_on;
#[cfg(feature="tokio")]
use tokio::time::{Instant, error::Elapsed};

use std::fmt::Debug;

use super::WeakSender;

pub struct Sender<T>
    where T: Unpin
{

    shared_details: Arc<PreferredMutexType<ChannelSharedDetails<T>>>,
    senders_count: Arc<()>,
    receivers_count: Weak<()>

}

impl<T> Sender<T>
    where T: Unpin
{

    pub fn new(shared_details: Arc<PreferredMutexType<ChannelSharedDetails<T>>>, senders_count: Arc<()>, receivers_count: Weak<()>) -> Self
    {

        Self
        {

            shared_details,
            senders_count,
            receivers_count
            
        }

    }

    pub fn send<'a>(&'a self, value: T) -> SendFuture<'a, T>
    {

        SendFuture::new(self, value)

    }

    pub fn blocking_send(&self, value: T) -> Result<(), BoundedSendError<T>>
    {

        block_on(self.send(value))

    }

    #[cfg(feature="tokio")]
    pub async fn recv_timeout_tokio(&self, value: T, duration: Duration) -> Result<Result<(), BoundedSendError<T>>, Elapsed>
    {

        use tokio::time::timeout;

        let res = self.send(value);

        timeout(duration, res).await

    }

    #[cfg(feature="tokio")]
    pub async fn recv_timeout_at_tokio(&self, value: T, deadline: Instant) -> Result<Result<(), BoundedSendError<T>>, Elapsed>
    {

        use tokio::time::timeout_at;

        let res = self.send(value);

        timeout_at(deadline, res).await

    }

    pub fn is_empty(&self) -> bool
    {

        #[cfg(feature="use_std_sync")]
        let mg = get_mg(&self.shared_details);

        #[cfg(any(feature="use_parking_lot_sync", feature="use_parking_lot_fair_sync"))]
        let mg = self.shared_details.lock();

        mg.message_queue.is_empty()

    }

    pub fn len(&self) -> usize
    {

        #[cfg(feature="use_std_sync")]
        let mg = get_mg(&self.shared_details);

        #[cfg(any(feature="use_parking_lot_sync", feature="use_parking_lot_fair_sync"))]
        let mg = self.shared_details.lock();

        mg.message_queue.len()

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

        #[cfg(feature="use_std_sync")]
        let mg = get_mg(&self.shared_details);

        #[cfg(any(feature="use_parking_lot_sync", feature="use_parking_lot_fair_sync"))]
        let mg = self.shared_details.lock();

        mg.is_closed

    }

    pub fn downgrade(&self) -> WeakSender<T>
    {

        WeakSender::new(&self.shared_details, &self.senders_count, self.receivers_count.clone())

    }

    pub fn same_channel(&self, other: &Self) -> bool
    {

        Arc::ptr_eq(&self.shared_details, &other.shared_details) 

    }

    pub fn shared_details_ptr_addr(&self) -> usize
    {

        Arc::as_ptr(&self.shared_details).addr()

    }

    /*
    pub fn same_channel_receiver(&self, other: &super::Receiver<T>) -> bool
    {

        self.shared_details_ptr_addr() == other.shared_details_ptr_addr()

    }
    */

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

        //if Arc::strong_count(&self.shared_details) == 1
        if self.strong_count() == 1
        {

            #[cfg(feature="use_std_sync")]
            let mut mg = get_mg(&self.shared_details);

            #[cfg(any(feature="use_parking_lot_sync", feature="use_parking_lot_fair_sync"))]
            let mut mg = self.shared_details.lock();

            mg.is_closed = true;

            //Engage free-for-all mode.

            for waker in mg.when_empty_waker_queue.drain(..)
            {

                waker.wake();

            }

        }
    
    }

}

static EXPECTED_VALUE_NOT_FOUND_MESSAGE: &str = "Error: Expected value not found.";

pub struct SendFuture<'a, T>
    where T: Unpin
{

    sender_ref: &'a Sender<T>,
    opt_waker_id: Option<usize>,
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
            opt_waker_id: None,
            opt_value: Some(value)

        }

    }

}

impl<'a, T> Future for SendFuture<'a, T>
    where T: Unpin
{

    type Output = Result<(), BoundedSendError<T>>;

    fn poll(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> Poll<Self::Output>
    {
        
        let waker;

        {

            let mut_self = self.get_mut();

            let value = mut_self.opt_value.take().expect(EXPECTED_VALUE_NOT_FOUND_MESSAGE);

            #[cfg(feature="use_std_sync")]
            let mut mg = get_mg(&mut_self.sender_ref.shared_details);

            #[cfg(any(feature="use_parking_lot_sync", feature="use_parking_lot_fair_sync"))]
            let mut mg = mut_self.sender_ref.shared_details.lock();

            if mg.is_closed
            {

                return Poll::Ready(Err(BoundedSendError::Closed(value)));

            }

            if mg.message_queue.len() < mg.capacity()
            {

                mg.message_queue.push_back(value);

                let opt_waker = mg.when_empty_waker_queue.pop_front();

                if let Some(the_waker) = opt_waker
                {

                    waker = the_waker;

                    let opt_entry = mg.active_ids.get_mut(&waker.id());
                    
                    if let Some(entry) = opt_entry
                    {

                        *entry = true;

                    }

                }
                else
                {

                    return Poll::Ready(Ok(()));

                }

            }
            else
            {

                return Poll::Ready(Err(BoundedSendError::Full(value)));

            }

        }

        waker.wake();

        Poll::Ready(Ok(()))

    }

}

impl<'a, T> Debug for SendFuture<'a, T>
    where T: Debug + Unpin
{

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SendFuture").field("sender_ref", &self.sender_ref).field("opt_waker_id", &self.opt_waker_id).field("opt_value", &self.opt_value).finish()
    }

}

impl<'a, T> Drop for SendFuture<'a, T>
    where T: Unpin
{

    fn drop(&mut self)
    {

        if let Some(id) = self.opt_waker_id
        {

            #[cfg(feature="use_std_sync")]
            let mut mg = get_mg(&self.sender_ref.shared_details);

            #[cfg(any(feature="use_parking_lot_sync", feature="use_parking_lot_fair_sync"))]
            let mut mg = self.sender_ref.shared_details.lock();
            
            mg.remove_waker(id);

            //mg.active_ids.remove(&id);

        }

        // SAFETY: `self` is pinned till after dropped.
        //unsafe { Drop::pin_drop(std::pin::Pin::new_unchecked(self)) }
    }

}
