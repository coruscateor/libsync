use std::error::Error;

use std::fmt::Display;

use std::future::Future;

use std::sync::atomic::{AtomicBool, Ordering};

use std::task::{Poll, Waker};

use pastey::paste;

use accessorise::impl_val_getter;

use inc_dec::{IncDecSelf, IntIncDecSelf};

use crossbeam_queue::SegQueue;

use delegate::delegate;

use crate::spin_a_bit;

pub struct WakerQueueInternals
{

    pub queue: SegQueue<Waker>,
    pub is_closed: AtomicBool

}

impl WakerQueueInternals
{

    pub fn new() -> Self
    {

        Self
        {

            queue: SegQueue::new(),
            is_closed: AtomicBool::new(false)

        }

    }

    pub fn is_closed(&self) -> bool
    {

        self.is_closed.load(Ordering::Acquire)

    }

    pub fn close(&self)
    {

        self.is_closed.store(true,Ordering::Release)

    }

}

pub struct WakerQueue
{

    waker_queue_internals: WakerQueueInternals

}

impl WakerQueue
{

    pub fn new() -> Self
    {

        Self
        {

            waker_queue_internals: WakerQueueInternals::new()

        }

    }

    delegate!
    {

        to self.waker_queue_internals
        {

            pub fn is_closed(&self) -> bool;

        }

    }

    pub fn wake_me<'a>(&'a self) -> WakerQueueWakeMe<'a>
    {

        WakerQueueWakeMe::new(self)

    }

    pub fn notify_one(&self) -> Option<bool>
    {


        if let Some(waker) = self.waker_queue_internals.queue.pop()
        {

            waker.wake();

            Some(true)

        }
        else
        {

            if self.waker_queue_internals.is_closed()
            {

                None

            }
            else
            {

                Some(false)
                
            }
            
        }

    }
    
    pub fn notify_waiters(&self) -> usize
    {

        let res = self.waker_queue_internals.queue.len();

        while let Some(waker) = self.waker_queue_internals.queue.pop()
        {

            waker.wake();

        }

        res

    }

    pub fn close(&self)
    {

        self.waker_queue_internals.close();

        //Yield the thread, spin-loop a bit?

        spin_a_bit();

        while let Some(waker) = self.waker_queue_internals.queue.pop()
        {

            waker.wake();

        }

    }

}

#[derive(Debug)]
pub struct WakerQueueWakeMeClosedError
{
}

impl WakerQueueWakeMeClosedError
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

impl Display for WakerQueueWakeMeClosedError
{

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        
        write!(f, "WakerQueue Closed")

    }

}

impl Error for WakerQueueWakeMeClosedError
{    
}

pub struct WakerQueueWakeMe<'a>
{

    waker_queue_ref: &'a WakerQueue,
    ready_next: bool

}

impl<'a> WakerQueueWakeMe<'a>
{

    pub fn new(waker_queue_ref: &'a WakerQueue) -> Self
    {

        Self
        {

            waker_queue_ref,
            ready_next: false

        }

    }

}

impl Future for WakerQueueWakeMe<'_>
{

    type Output = Result<(), WakerQueueWakeMeClosedError>;

    fn poll(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> Poll<Self::Output>
    {

        let mut_self = self.get_mut();

        if mut_self.ready_next
        {

            mut_self.ready_next = false;

            return Poll::Ready(Ok(()));

        }

        mut_self.ready_next = true;

        //The task is going to "sleep". Update the WQI so it can be woken up later.

        let waker = cx.waker().clone();

        if mut_self.waker_queue_ref.waker_queue_internals.is_closed()
        {

            return Poll::Ready(Err(WakerQueueWakeMeClosedError::new()));

        }

        mut_self.waker_queue_ref.waker_queue_internals.queue.push(waker);

        Poll::Pending

    }

}
