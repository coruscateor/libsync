use std::error::Error;

use std::fmt::Display;

use std::future::Future;

use std::sync::{Mutex, MutexGuard};

use std::sync::atomic::{AtomicUsize, Ordering};

use std::collections::VecDeque;

use std::task::{Poll, Waker};

pub struct WakerQueue
{

    waker_queue: Mutex<Option<VecDeque<Waker>>>

}

impl WakerQueue
{

    pub fn new() -> Self
    {

        Self
        {

            waker_queue: Mutex::new(Some(VecDeque::new()))

        }

    }

    pub fn with_permits(count: usize) -> Self
    {

        Self
        {

            waker_queue: Mutex::new(Some(VecDeque::with_capacity(count)))

        }

    }

    pub fn with_capacity(size: usize) -> Self
    {

        Self
        {

            waker_queue: Mutex::new(Some(VecDeque::with_capacity(size)))

        }

    }

    fn clear_poison_get_mg(&self) -> MutexGuard<'_, Option<VecDeque<Waker>>>
    {

        let lock_result = self.waker_queue.lock();

        match lock_result
        {

            Ok(mg) =>
            {

                mg

            }
            Err(err) =>
            {

                self.waker_queue.clear_poison();

                err.into_inner()

            }

        }

    }

    pub fn when_open_ref<T, F>(&self, func: F) -> Option<T>
        where F: Fn(&VecDeque<Waker>) -> T
    {

        let mg = self.clear_poison_get_mg();

        match &*mg
        {

            Some(val) =>
            {

                Some(func(val))

            }
            None => None
            
        }

    }

    pub fn when_open_mut<T, F>(&self, func: F) -> Option<T>
        where F: Fn(&mut VecDeque<Waker>) -> T
    {

        let mut mg = self.clear_poison_get_mg();

        match &mut *mg
        {

            Some(val) =>
            {

                Some(func(val))

            }
            None => None
            
        }

    }

    pub fn len(&self) -> Option<usize>
    {

        self.when_open_ref(|queue| {

            queue.len()

        })

        /*
            let mg = self.clear_poison_get_mg();

            match &*mg
            {

                Some(val) =>
                {

                    Some(val.len())

                }
                None => None

            }
        */

    }

    pub fn capacity(&self) -> Option<usize>
    {

        self.when_open_ref(|queue| {

            queue.capacity()

        })

    }

    pub fn try_wake_one(&self) -> bool
    {

        let waker;

        {

            let mut mg = self.clear_poison_get_mg();

            match &mut *mg
            {

                Some(val) =>
                {

                    if let Some(front_waker) = val.pop_front()
                    {

                        waker = front_waker;

                    }
                    else
                    {

                        return false;
                        
                    }

                }
                None =>
                {
                    
                    return false;

                }

            }

        }

        waker.wake();

        true

    }

    pub fn wake_me()
    {



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
        {}

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

    waker_queue_ref: &'a WakerQueue

}

impl<'a> WakerQueueWakeMe<'a>
{

    pub fn new(waker_queue_ref: &'a WakerQueue) -> Self
    {

        Self
        {

            waker_queue_ref

        }

    }

}

impl Future for WakerQueueWakeMe<'_>
{

    type Output = Result<(), WakerQueueWakeMeClosedError>;

    fn poll(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> Poll<Self::Output>
    {

        {

            let mut mg = self.waker_queue_ref.clear_poison_get_mg();

            match &mut *mg
            {

                Some(val) =>
                {

                    if val.is_empty()
                    {

                        return Poll::Ready(Ok(()));

                    }
                    else
                    {

                        let waker = cx.waker().clone();

                        val.push_back(waker);
                        
                    }

                }
                None =>
                {

                    return Poll::Ready(WakerQueueWakeMeClosedError::err());

                }

            }

        }

        Poll::Pending

    }

}