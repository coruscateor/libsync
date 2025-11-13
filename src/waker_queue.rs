use std::error::Error;

use std::fmt::Display;

use std::future::Future;

use std::sync::{Mutex, MutexGuard};

use std::sync::atomic::{AtomicUsize, Ordering};

use std::collections::{HashMap, VecDeque};

use std::task::{Poll, Waker};

use paste::paste;

use accessorise::impl_get_val;

use inc_dec::IntIncDecSelf;

pub struct QueuedWaker
{

    waker: Waker,
    handle: usize

}

impl QueuedWaker
{

    pub fn new(waker: Waker, handle: usize) -> Self
    {

        Self
        {

            waker,
            handle

        }

    }

    impl_get_val!(handle, usize);

    pub fn wake(self)
    {

        self.waker.wake();

    }
    
}

pub struct WakerQueueInternals
{

    pub queue: VecDeque<QueuedWaker>,
    pub handle: usize,
    pub handle_states: HashMap<usize, bool>

}

impl WakerQueueInternals
{

    pub fn new() -> Self
    {

        Self
        {

            queue: VecDeque::new(),
            handle: 0,
            handle_states: HashMap::new()

        }

    }

    pub fn with_capacity(capacity: usize) -> Self
    {

        Self
        {

            queue: VecDeque::with_capacity(capacity),
            handle: 0,
            handle_states: HashMap::with_capacity(capacity)

        }

    }


}

pub struct WakerQueue
{

    waker_queue_internals: Mutex<Option<WakerQueueInternals>>

}

impl WakerQueue
{

    pub fn new() -> Self
    {

        Self
        {

            waker_queue_internals: Mutex::new(Some(WakerQueueInternals::new()))

        }

    }

    pub fn with_capacity(size: usize) -> Self
    {

        Self
        {

            waker_queue_internals: Mutex::new(Some(WakerQueueInternals::with_capacity(size)))

        }

    }

    fn clear_poison_get_mg(&self) -> MutexGuard<'_, Option<WakerQueueInternals>>
    {

        let lock_result = self.waker_queue_internals.lock();

        match lock_result
        {

            Ok(mg) =>
            {

                mg

            }
            Err(err) =>
            {

                self.waker_queue_internals.clear_poison();

                err.into_inner()

            }

        }

    }

    pub fn when_open_ref<T, F>(&self, func: F) -> Option<T>
        where F: Fn(&WakerQueueInternals) -> T
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
        where F: Fn(&mut WakerQueueInternals) -> T
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

        self.when_open_ref(|internals|
        {

            internals.queue.len()

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

        self.when_open_ref(|internals| {

            internals.queue.capacity()

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

                    if let Some(front_waker) = val.queue.pop_front()
                    {

                        waker = front_waker;

                    }
                    else
                    {

                        return false;
                        
                    }

                    val.handle_states.entry(key)

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

    pub fn wake_me<'a>(&'a self) -> WakerQueueWakeMe<'a>
    {

        WakerQueueWakeMe::new(self)

        /*
        let mg = self.clear_poison_get_mg();

        match &*mg
        {

            Some(val) =>
            {

                let new_handle = val.handle.wpp();

                

            }
            None => None
            
        }
        */

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

    waker_queue_ref: &'a WakerQueue,
    opt_waker_handle: Option<usize>

}

impl<'a> WakerQueueWakeMe<'a>
{

    pub fn new(waker_queue_ref: &'a WakerQueue) -> Self //, waker_handle: usize) -> Self
    {

        Self
        {

            waker_queue_ref,
            opt_waker_handle: None //Some(waker_handle)

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

                    /*
                    if val.queue.is_empty()
                    {

                        return Poll::Ready(Ok(()));

                    }
                    else
                    {

                        let waker = cx.waker().clone();

                        val.push_back(waker);
                        
                    }
                    */

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