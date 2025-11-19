use std::error::Error;

use std::fmt::Display;

use std::future::Future;

use std::sync::{Mutex, MutexGuard};

use std::sync::atomic::{AtomicUsize, Ordering};

use std::collections::{HashSet, VecDeque};

use std::task::{Poll, Waker};

//use core::result::Result;

use crossbeam::queue;
use futures::stream::iter;
use inc_dec::IncDecSelf;
use paste::paste;

use accessorise::impl_get_val;

use crate::QueuedWaker;

pub struct LimitedNotifierInternals
{

    pub queue: VecDeque<QueuedWaker>,
    pub handle: usize,
    pub active_handles: HashSet<usize>,
    pub permits: usize

}


impl LimitedNotifierInternals
{

    pub fn new() -> Self
    {

        Self
        {

            queue: VecDeque::new(),
            handle: 0,
            active_handles: HashSet::new(),
            permits: 0

        }

    }

    pub fn with_capacity(capacity: usize) -> Self
    {

        Self
        {

            queue: VecDeque::with_capacity(capacity),
            handle: 0,
            active_handles: HashSet::with_capacity(capacity),
            permits: 0

        }

    }

    pub fn with_permits(permits: usize) -> Self
    {

        Self
        {

            queue: VecDeque::with_capacity(permits),
            handle: 0,
            active_handles: HashSet::with_capacity(permits),
            permits: permits

        }

    }

    pub fn with_capacity_and_permits(capacity: usize, permits: usize) -> Self
    {

        Self
        {

            queue: VecDeque::with_capacity(capacity),
            handle: 0,
            active_handles: HashSet::with_capacity(capacity),
            permits

        }

    }

}

pub struct LimitedNotifier
{

    limted_notifier: Mutex<Option<LimitedNotifierInternals>>

}

impl LimitedNotifier
{

    pub fn new() -> Self
    {

        Self
        {

            limted_notifier: Mutex::new(Some(LimitedNotifierInternals::new()))

        }

    }

    pub fn with_capacity(capacity: usize) -> Self
    {

        Self
        {

            limted_notifier: Mutex::new(Some(LimitedNotifierInternals::with_capacity(capacity)))

        }

    }

    pub fn with_permits(permits: usize) -> Self
    {

        Self
        {

            limted_notifier: Mutex::new(Some(LimitedNotifierInternals::with_capacity(permits)))

        }

    }

    pub fn with_capacity_and_permits(capacity: usize, permits: usize) -> Self
    {

        Self
        {

            limted_notifier: Mutex::new(Some(LimitedNotifierInternals::with_capacity_and_permits(capacity, permits)))

        }

    }

    fn clear_poison_get_mg(&self) -> MutexGuard<'_, Option<LimitedNotifierInternals>>
    {

        let lock_result = self.limted_notifier.lock();

        match lock_result
        {

            Ok(mg) =>
            {

                mg

            }
            Err(err) =>
            {

                self.limted_notifier.clear_poison();

                err.into_inner()

            }

        }

    }

    pub fn avalible_permits(&self) -> Option<usize>
    {

        let mut mg = self.clear_poison_get_mg();

        if let Some(val) = &mut *mg
        {

            return Some(val.permits);

        } 

        None

    }

    pub fn add_permits(&self, count: usize) -> bool
    {

        if count == 0
        {

            return false;

        }

        let mut mg = self.clear_poison_get_mg();

        if let Some(val) = &mut *mg
        {

            let permits = val.permits;

            if let Some(mut permits) = permits.checked_add(count)
            {

                val.permits = permits;

                while permits > 0
                {

                    //Check for wakers and wake them if present.

                    let opt_front_waker = val.queue.pop_front();

                    if let Some(front_waker) = opt_front_waker
                    {

                        //does the waker need to be marked as "should wake"?

                        front_waker.wake();

                    }
                    else
                    {

                        break;

                    }

                    permits.mm();

                    
                }

                return true;

            }

        }

        false

    }

    pub fn add_permit(&self)
    {

        self.add_permits(1);

    }

    pub fn remove_permits(&self, count: usize) -> bool
    {

        if count == 0
        {

            return false;

        }

        let mut mg = self.clear_poison_get_mg();

        if let Some(val) = &mut *mg
        {

            let permits = val.permits;

            if let Some(permits) = permits.checked_sub(count)
            {

                val.permits = permits;

                return true;

            }

        }

        false

    }

    pub fn remove_permit(&self)
    {

        self.remove_permits(1);

    }

    pub fn aquire<'a>(&'a self) -> LimitedNotifierAquire<'a>
    {

        LimitedNotifierAquire::new(self)

    }

    pub fn close(&self)
    {

        let opt_internals;

        {

            let mut mg = self.clear_poison_get_mg();

            opt_internals = mg.take();

        }

        if let Some(mut internals) = opt_internals
        {

            for item in internals.queue.drain(..)
            {

                item.wake();

            }

        }

    }

    pub fn is_closed(&self) -> bool
    {

        let mg = self.clear_poison_get_mg();

        mg.is_none()

    }

    //impl_get_val!(max_number_of_permits, usize);

    /*
    pub fn add_permit(&self)
    {

        self.avalible_permits.compare_exchange(current, new, success, failure)

    }
    */
    
}

#[derive(Debug)]
pub struct LimitedNotifierClosedError
{
}

impl LimitedNotifierClosedError
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

impl Display for LimitedNotifierClosedError
{

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        
        write!(f, "LimitedNotifier Closed")

    }

}

impl Error for LimitedNotifierClosedError
{    
}

pub struct LimitedNotifierAquire<'a>
{

    limted_notifier: &'a LimitedNotifier

}

impl<'a> LimitedNotifierAquire<'a>
{

    pub fn new(limted_notifier: &'a LimitedNotifier) -> Self
    {

        Self
        {

            limted_notifier

        }

    }

    fn no_permits(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> Poll<Result<(), LimitedNotifierClosedError>>
    {

        //Add the task waker to the queue.

        let lock_result = self.limted_notifier.waker_queue.lock();

        let mut opt_queue;

        match lock_result
        {

            Ok(mg) =>
            {

                opt_queue = mg;

            }
            Err(err) =>
            {

                self.limted_notifier.waker_queue.clear_poison();

                opt_queue = err.into_inner();

            }

        }

        match &mut *opt_queue
        {

            Some(queue) =>
            {

                let waker = cx.waker().clone();

                queue.push_back(waker);

                return Poll::Pending;

            }
            None =>
            {

                return Poll::Ready(LimitedNotifierClosedError::err());

            }

        }

    }
    
}

impl Future for LimitedNotifierAquire<'_>
{

    type Output = Result<(), LimitedNotifierClosedError>;

    fn poll(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> Poll<Self::Output>
    {

        let permits = self.limted_notifier.avalible_permits();

        //Is it closed?

        if permits == 0
        {

            return self.no_permits(cx);
        
        }
        else
        {

            match permits.checked_sub(1)
            {

                Some(val) =>
                {

                    let res = self.limted_notifier.avalible_permits.compare_exchange(permits, val, Ordering::AcqRel, Ordering::Relaxed);

                    match res
                    {
                        Ok(val) =>
                        {

                            if val == permits
                            {

                                return Poll::Ready(Ok(()));

                            }

                        }
                        Err(_err) =>
                        {



                        }

                    }

                    loop
                    {

                        let permits = self.limted_notifier.avalible_permits();

                        if permits == 0
                        {

                            return self.no_permits(cx);

                        }

                        let res = self.limted_notifier.avalible_permits.compare_exchange(permits, val, Ordering::AcqRel, Ordering::Relaxed);

                        match res
                        {

                            Ok(val) =>
                            {

                                if val == permits
                                {

                                    return Poll::Ready(Ok(()));

                                }

                            }
                            _ =>
                            {
                            }
                            
                        }

                    }

                }
                None =>
                {

                    return self.no_permits(cx);

                }

            }

            //return Poll::Ready(Ok(()));
            
        }        

    }

}