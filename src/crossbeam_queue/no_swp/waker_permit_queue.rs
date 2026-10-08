use std::error::Error;

use std::fmt::Display;

use std::future::Future;

use std::hint;
use std::sync::atomic::{AtomicUsize, AtomicBool, Ordering};

use std::collections::{HashMap, HashSet, VecDeque};

use std::task::{Poll, Waker};

//use core::result::Result;

use inc_dec::{IncDecSelf, IntIncDecSelf};

use pastey::paste;

use accessorise::impl_val_getter;

use std::fmt::Debug;

use crossbeam_queue::SegQueue;

use delegate::delegate;

use crate::atomic::{add_permits, remove_permits};
use crate::spin_a_bit;

#[derive(Debug)]
pub struct WakerPermitQueueInternals
{

    pub no_permits_queue: SegQueue<Waker>, //Wakers that were enqueued because the were no permits available.
    pub permits: AtomicUsize,
    pub is_closed: AtomicBool

}

impl WakerPermitQueueInternals
{

    pub fn new() -> Self
    {

        Self
        {

            no_permits_queue: SegQueue::new(),
            permits: AtomicUsize::new(0),
            is_closed: AtomicBool::new(false)

        }

    }

    pub fn with_permits(permits: usize) -> Self
    {

        Self
        {

            no_permits_queue: SegQueue::new(),
            permits: AtomicUsize::new(permits),
            is_closed: AtomicBool::new(false)

        }

    }

    pub fn permits(&self) -> usize
    {

        self.permits.load(Ordering::SeqCst) //Ordering::Acquire)

    }

    pub fn add_permits(&self, permits: usize) -> Option<usize>
    {

        if self.is_closed()
        {

            return None;

        }

        let current_permits = self.permits();

        Some(add_permits(&self.permits, current_permits, permits))

        /*
        loop
        {

            /*
            if permits == 0
            {

                return Some(0);

            }
            */

            if let Some(new_permits) = current_permits.checked_add(permits)
            {

                let res = self.permits.compare_exchange(current_permits, new_permits, Ordering::SeqCst, Ordering::SeqCst);

                match res
                {

                    Ok(_num) =>
                    {

                        return Some(permits);

                    }
                    Err(num) =>
                    {

                        current_permits = num;

                    }

                }

            }
            else
            {

                permits = usize::MAX - current_permits;
                
            }
            
        }
        */

    }

    pub fn remove_permits(&self, permits: usize) -> Option<usize>
    {

        if self.is_closed()
        {

            return None;

        }

        let current_permits = self.permits();

        Some(remove_permits(&self.permits, current_permits, permits))

        /*
        loop
        {

            /*
            if permits == 0
            {

                return Some(0);

            }
            */

            if let Some(new_permits) = current_permits.checked_sub(permits)
            {

                let res = self.permits.compare_exchange(current_permits, new_permits, Ordering::SeqCst, Ordering::SeqCst);

                match res
                {

                    Ok(_num) =>
                    {

                        return Some(permits);

                    }
                    Err(num) =>
                    {

                        current_permits = num;

                    }

                }

            }
            else
            {

                permits = current_permits;
                
            }
            
        }
        */

    }

    pub fn is_closed(&self) -> bool
    {

        self.is_closed.load(Ordering::Acquire)

    }

    pub fn close(&self)
    {

        self.is_closed.store(true,Ordering::Release);

        //Yield the thread, spin-loop a bit?

        spin_a_bit();

        while let Some(waker) = self.no_permits_queue.pop()
        {

            waker.wake();

        }

    }

}

#[derive(Debug)]
pub struct WakerPermitQueue
{

    waker_queue_internals: WakerPermitQueueInternals

}

impl WakerPermitQueue
{

    pub fn new() -> Self
    {

        Self
        {

            waker_queue_internals: WakerPermitQueueInternals::new()

        }

    }

    pub fn with_permits(permits: usize) -> Self
    {

        Self
        {

            waker_queue_internals: WakerPermitQueueInternals::with_permits(permits)

        }

    }

    pub fn avalible_permits(&self) -> Option<usize>
    {

        if self.waker_queue_internals.is_closed()
        {

            None

        }
        else
        {

            Some(self.waker_queue_internals.permits())
            
        }

    }

    delegate!
    {

        to self.waker_queue_internals
        {

            pub fn is_closed(&self) -> bool;

            pub fn close(&self);

            pub fn add_permits(&self, permits: usize) -> Option<usize>;

            pub fn remove_permits(&self, permits: usize) -> Option<usize>;

        }

    }

    /*
    pub fn add_permits(&self, permits: usize) -> Option<bool>
    {

        if self.waker_queue_internals.is_closed()
        {

            None

        }
        else
        {
        
            let from_max = usize::MAX - permits;

            let current_permits = self.waker_queue_internals.permits();

            if current_permits <= from_max
            {

                self.waker_queue_internals.permits.fetch_add(permits, Ordering::Release);

                Some(true)

            }
            else
            {

                Some(false)
                
            }

        }

    }
    */

    pub fn add_permit(&self) -> Option<bool>
    {

        if let Some(val) = self.add_permits(1)
        {

            Some(val == 1)

        }
        else
        {
            
            None

        }

    }

    /*
    pub fn remove_permits(&self, permits: usize) -> Option<bool>
    {

        if self.waker_queue_internals.is_closed()
        {

            None

        }
        else
        {

            let current_permits = self.waker_queue_internals.permits();

            if current_permits >= permits
            {

                self.waker_queue_internals.permits.fetch_sub(permits, Ordering::Release);

                Some(true)

            }
            else
            {

                Some(false)
                
            }

        }

    }
    */

    pub fn remove_permit(&self) -> Option<bool>
    {

        if let Some(val) = self.remove_permits(1)
        {

            Some(val == 1)

        }
        else
        {
            
            None

        }

    }
    
    pub fn decrement_permits_or_wait<'a>(&'a self) -> WakerPermitQueueDecrementPermitsOrWait<'a>
    {

        WakerPermitQueueDecrementPermitsOrWait::new(self)

    }
    
}

impl Drop for WakerPermitQueue
{

    fn drop(&mut self)
    {

        //Is this call necessary?
        
        self.close();

    }

}

#[derive(Debug, PartialEq)]
pub struct WakerPermitQueueClosedError
{
}

impl WakerPermitQueueClosedError
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

impl Display for WakerPermitQueueClosedError
{

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        
        write!(f, "WakerPermitQueue is closed")

    }

}

impl Error for WakerPermitQueueClosedError
{    
}

#[derive(Debug)]
pub struct WakerPermitQueueDecrementPermitsOrWait<'a>
{

    waker_permit_queue_ref: &'a WakerPermitQueue

}

impl<'a> WakerPermitQueueDecrementPermitsOrWait<'a>
{

    pub fn new(waker_permit_queue_ref: &'a WakerPermitQueue) -> Self
    {

        Self
        {

            waker_permit_queue_ref

        }

    }
    
}

//Handles "sleeping", "waking" and permit incrementation/decrementation.

impl Future for WakerPermitQueueDecrementPermitsOrWait<'_>
{

    type Output = Result<(), WakerPermitQueueClosedError>;

    fn poll(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> Poll<Self::Output>
    {

        let waker_queue_internals = &self.waker_permit_queue_ref.waker_queue_internals;

        //Is there a queue?

        if waker_queue_internals.no_permits_queue.is_empty()
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

                return Poll::Ready(WakerPermitQueueClosedError::err());
                
            }

            /*
            let permits = waker_queue_internals.permits();

            if permits > 0
            {



            }

            //"Take" a permit.

            waker_queue_internals.permits.fetch_sub(1, Ordering::Acquire);

            if let Some(new_permits) = permits.checked_sub(1)
            {

                val.permits = new_permits;

                //There was at least one permit available so we don't need to wait.

                return Poll::Ready(Ok(()));

            }
            */

        }

        //The task is going to "sleep". Update the WQI so it can be woken up later.

        let waker = cx.waker().clone();

        waker_queue_internals.no_permits_queue.push(waker);

        Poll::Pending
        
    }

}
