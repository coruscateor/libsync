use std::collections::hash_map::Entry;
use std::error::Error;

use std::fmt::Display;

use std::future::Future;

#[cfg(feature="use_std_sync")]
use std::sync::{MutexGuard, TryLockError};

use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

use std::collections::{HashMap, HashSet, VecDeque};

use std::task::{Poll, Waker};

//use core::result::Result;

use inc_dec::{IncDecSelf, IntIncDecSelf};

use pastey::paste;

use accessorise::impl_val_getter;

use std::fmt::Debug;

use crate::PreferredMutexType;
use crate::atomic::{add_permits, remove_permits};

use crossbeam_queue::SegQueue;

use delegate::delegate;

use crate::spin_a_bit;

#[derive(Debug)]
pub struct LimitedWakerPermitQueueInternals
{

    pub no_permits_queue: SegQueue<Waker>, //Wakers that were enqueued because the were no permits available.
    pub permits: AtomicUsize,
    pub max_permits_queue: SegQueue<Waker>, //Wakers that were enqueued because the maximum number of permits had been reached.
    pub is_closed: AtomicBool

}

impl LimitedWakerPermitQueueInternals
{

    pub fn new() -> Self
    {

        Self
        {

            no_permits_queue: SegQueue::new(),
            permits: AtomicUsize::new(0),
            max_permits_queue: SegQueue::new(),
            is_closed: AtomicBool::new(false)

        }

    }

    pub fn with_permits(permits: usize) -> Self
    {

        Self
        {

            no_permits_queue: SegQueue::new(), //VecDeque::with_capacity(permits),
            permits: AtomicUsize::new(permits),
            max_permits_queue: SegQueue::new(),
            is_closed: AtomicBool::new(false)

        }

    }

    pub fn permits(&self) -> usize
    {

        self.permits.load(Ordering::SeqCst) //Ordering::Acquire)

    }

    pub fn is_closed(&self) -> bool
    {

        self.is_closed.load(Ordering::Acquire)

    }

    pub fn close(&self)
    {

        self.is_closed.store(true,Ordering::Release)

    }

    pub fn add_permits(&self, permits: usize) -> Option<usize>
    {

        if self.is_closed()
        {

            return None;

        }

        let current_permits = self.permits();

        Some(add_permits(&self.permits, current_permits, permits))

    }

    pub fn remove_permits(&self, permits: usize) -> Option<usize>
    {

        if self.is_closed()
        {

            return None;

        }

        let current_permits = self.permits();

        Some(remove_permits(&self.permits, current_permits, permits))

    }

}

#[derive(Debug)]
pub struct LimitedWakerPermitQueue
{

    internal_state: LimitedWakerPermitQueueInternals,
    max_permits: usize

}

impl LimitedWakerPermitQueue
{

    pub fn new(max_permits: usize) -> Self
    {

        Self
        {

            internal_state: LimitedWakerPermitQueueInternals::new(),
            max_permits

        }

    }

    pub fn with_permits(max_permits: usize, permits: usize) -> Self
    {

        Self
        {

            internal_state: LimitedWakerPermitQueueInternals::with_permits(permits),
            max_permits

        }

    }

    pub fn avalible_permits(&self) -> Option<usize>
    {

        if self.internal_state.is_closed()
        {

            None

        }
        else
        {

            Some(self.internal_state.permits())
            
        }

    }
    
    delegate!
    {

        to self.internal_state
        {

            pub fn is_closed(&self) -> bool;

            pub fn close(&self);

            pub fn add_permits(&self, permits: usize) -> Option<usize>;

            pub fn remove_permits(&self, permits: usize) -> Option<usize>;

        }

    }

    impl_val_getter!(max_permits, usize);

    pub fn has_max_permits(&self) -> Option<bool>
    {

        if let Some(permits) = self.avalible_permits()
        {

            let is_at_max = permits == self.max_permits;

            Some(is_at_max)

        }
        else
        {
            
            None

        }

    }

    pub fn head_room(&self) -> Option<usize>
    {

        if let Some(permits) = self.avalible_permits()
        {

            let head_room = self.max_permits - permits;

            Some(head_room)

        }
        else
        {
            
            None

        }

    }


    pub fn add_permit(&self) -> Option<bool>
    {

        if let Some(val) = self.add_permits(1)
        {

            Some(val > 0)

        }
        else
        {

            None
            
        }

    }

    pub fn remove_permit(&self) -> Option<bool>
    {

        if let Some(val) = self.remove_permits(1)
        {

            Some(val > 0)

        }
        else
        {

            None
            
        }

    }

    pub fn decrement_permits_or_wait<'a>(&'a self) -> LimitedWakerPermitQueueDecrementPermitsOrWait<'a>
    {

        LimitedWakerPermitQueueDecrementPermitsOrWait::new(self)

    }

    pub fn increment_permits_or_wait<'a>(&'a self) -> LimitedWakerPermitQueueIncrementPermitsOrWait<'a>
    {

        LimitedWakerPermitQueueIncrementPermitsOrWait::new(self)

    }
    
}

impl Drop for LimitedWakerPermitQueue
{

    fn drop(&mut self)
    {

        //Is this call necessary?
        
        self.close();

    }

}

#[derive(Debug, PartialEq)]
pub struct LimitedWakerPermitQueueClosedError
{
}

impl LimitedWakerPermitQueueClosedError
{

    pub fn new() -> Self
    {

        Self
        {
        }

    }

    pub fn err() -> Result<(), Self>
    {

        Err(Self::new())

    }

}

impl Display for LimitedWakerPermitQueueClosedError
{

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        
        write!(f, "LimitedWakerPermitQueue is closed")

    }

}

impl Error for LimitedWakerPermitQueueClosedError
{    
}

#[derive(Debug)]
pub struct LimitedWakerPermitQueueDecrementPermitsOrWait<'a>
{

    waker_permit_queue_ref: &'a LimitedWakerPermitQueue
}

impl<'a> LimitedWakerPermitQueueDecrementPermitsOrWait<'a>
{

    pub fn new(waker_permit_queue_ref: &'a LimitedWakerPermitQueue) -> Self
    {

        Self
        {

            waker_permit_queue_ref

        }

    }
    
}

//Handles "sleeping", "waking" and permit incrementation/decrementation.

impl Future for LimitedWakerPermitQueueDecrementPermitsOrWait<'_>
{

    type Output = Result<(), LimitedWakerPermitQueueClosedError>;

    fn poll(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> Poll<Self::Output>
    {

        let internal_state = &self.waker_permit_queue_ref.internal_state;

        //Is there a queue?

        if internal_state.no_permits_queue.is_empty()
        {

            if let Some(val) = self.waker_permit_queue_ref.remove_permit()
            {

                if val
                {

                    return Poll::Ready(Ok(()));

                }

            }
            else
            {

                return Poll::Ready(LimitedWakerPermitQueueClosedError::err());
                
            }

        }

        //The task is going to "sleep". Update the WQI so it can be woken up later.

        let waker = cx.waker().clone();

        internal_state.no_permits_queue.push(waker);

        Poll::Pending
        
    }

}

#[derive(Debug)]
pub struct LimitedWakerPermitQueueIncrementPermitsOrWait<'a>
{

    waker_permit_queue_ref: &'a LimitedWakerPermitQueue

}

impl<'a> LimitedWakerPermitQueueIncrementPermitsOrWait<'a>
{

    pub fn new(waker_permit_queue_ref: &'a LimitedWakerPermitQueue) -> Self
    {

        Self
        {

            waker_permit_queue_ref

        }

    }
    
}

//Handles "sleeping", "waking" and permit incrementation.

impl Future for LimitedWakerPermitQueueIncrementPermitsOrWait<'_>
{

    type Output = Result<(), LimitedWakerPermitQueueClosedError>;

    fn poll(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> Poll<Self::Output>
    {

        let internal_state = &self.waker_permit_queue_ref.internal_state;

        //Is there a queue?

        if internal_state.max_permits_queue.is_empty()
        {

            if let Some(val) = self.waker_permit_queue_ref.add_permit()
            {

                if val
                {

                    return Poll::Ready(Ok(()));

                }

            }
            else
            {

                return Poll::Ready(LimitedWakerPermitQueueClosedError::err());
                
            }

        }

        //The task is going to "sleep". Update the WQI so it can be woken up later.

        let waker = cx.waker().clone();

        internal_state.max_permits_queue.push(waker);

        Poll::Pending

    }

}
