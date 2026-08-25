use std::{collections::hash_map::Entry, sync::{Arc, Weak}, task::Poll};

#[cfg(feature="use_std_sync")]
use crate::get_mg;

use super::ChannelSharedDetails;

use crate::{BoundedSendError, PreferredMutexType, QueuedWaker};

use delegate::delegate;
use inc_dec::IntIncDecSelf;

use std::fmt::Debug;

pub struct Receiver<T>
{

    shared_details: Arc<PreferredMutexType<ChannelSharedDetails<T>>>,
    senders_count: Weak<()>,
    receivers_count: Arc<()>

}

impl<T> Receiver<T>
{

    pub fn new(shared_details: Arc<PreferredMutexType<ChannelSharedDetails<T>>>, senders_count: Weak<()>, receivers_count: Arc<()>) -> Self
    {

        Self
        {

            shared_details,
            senders_count,
            receivers_count
            
        }

    }

    pub fn recv<'a>(&'a self) -> RecvFuture<'a, T>
    {

        RecvFuture::new(self)

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
            /// The total number of Receiver instances.
            /// 
            #[call(strong_count)]
            pub fn senders_strong_count(&self) -> usize;

            ///
            /// The total number of potential Receiver instances.
            /// 
            #[call(weak_count)]
            pub fn senders_weak_count(&self) -> usize;

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

    //pub fn downgrade(&self) -> WeakSender<T>

    pub fn same_channel(&self, other: &Self) -> bool
    {

        Arc::ptr_eq(&self.shared_details, &other.shared_details) 

    }

    pub fn shared_details_ptr_addr(&self) -> usize
    {

        Arc::as_ptr(&self.shared_details).addr()

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

impl<T> Drop for Receiver<T>
{

    fn drop(&mut self)
    {

        if Arc::strong_count(&self.shared_details) == 1
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

pub struct RecvFuture<'a, T>
{

    receiver_ref: &'a Receiver<T>,
    opt_waker_id: Option<usize>

}

impl<'a, T> RecvFuture<'a, T>
{

    pub fn new(receiver_ref: &'a Receiver<T>) -> Self
    {

        Self
        {

            receiver_ref,
            opt_waker_id: None

        }

    }

}

impl<'a, T> Future for RecvFuture<'a, T>
{

    type Output = Result<T, ()>;

    fn poll(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> Poll<Self::Output>
    {
        
        #[cfg(feature="use_std_sync")]
        let mut mg = get_mg(&self.receiver_ref.shared_details);

        #[cfg(any(feature="use_parking_lot_sync", feature="use_parking_lot_fair_sync"))]
        let mut mg = self.receiver_ref.shared_details.lock();

        if mg.is_closed
        {

            let opt_message = mg.message_queue.pop_front();

            match opt_message
            {

                Some(message) =>
                {

                    return Poll::Ready(Ok(message));

                }
                None =>
                {

                    return Poll::Ready(Err(()));

                }

            }

        }

        if let Some(id) = self.opt_waker_id
        {

            if let Some(res) = mg.try_pop(id)
            {

                return Poll::Ready(Ok(res));

            }

            /*
            //if let Entry::Occupied(entry) = mg.active_ids.entry(id)
            if let Some(shouldve_awoken) = mg.active_ids.get(&id)
            {

                //let shouldve_awoken = entry.get();

                if *shouldve_awoken
                {

                    let opt_front = mg.message_queue.pop_front();

                    if let Some(front) = opt_front
                    {

                        //entry.remove_entry();

                        mg.active_ids.remove(&id);

                        return Poll::Ready(Ok(front));

                    }

                }

                return Poll::Pending;
                
            }
            */

        }

        let id = mg.push_waker(cx);

        let mut_self = self.get_mut();

        mut_self.opt_waker_id = Some(id);

        Poll::Pending

    }

}

impl<'a, T> Debug for RecvFuture<'a, T>
    where T: Debug
{

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RecvFuture").field("receiver_ref", &self.receiver_ref).field("opt_waker_id", &self.opt_waker_id).finish()
    }

}

impl<'a, T> Drop for RecvFuture<'a, T>
{

    fn drop(&mut self)
    {

        if let Some(id) = self.opt_waker_id
        {

            #[cfg(feature="use_std_sync")]
            let mut mg = get_mg(&self.receiver_ref.shared_details);

            #[cfg(any(feature="use_parking_lot_sync", feature="use_parking_lot_fair_sync"))]
            let mut mg = self.receiver_ref.shared_details.lock();
            
            mg.remove_waker(id);

            //mg.active_ids.remove(&id);

        }

        // SAFETY: `self` is pinned till after dropped.
        //unsafe { Drop::pin_drop(std::pin::Pin::new_unchecked(self)) }
    }

}