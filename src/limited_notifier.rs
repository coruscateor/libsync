use std::error::Error;

use std::fmt::Display;

use std::future::Future;

use std::sync::Mutex;

use std::sync::atomic::{AtomicUsize, Ordering};

use std::collections::VecDeque;

use std::task::{Poll, Waker};

//use core::result::Result;

use crossbeam::queue;
use futures::stream::iter;
use paste::paste;

use accessorise::impl_get_val;

pub struct LimitedNotifier
{

    //max_number_of_permits: usize,
    avalible_permits: AtomicUsize,
    waker_queue: Mutex<Option<VecDeque<Waker>>>

}

impl LimitedNotifier
{

    pub fn new() -> Self //max_number_of_permits: usize) -> Self
    {

        Self
        {

            //max_number_of_permits,
            avalible_permits: AtomicUsize::new(0),
            waker_queue: Mutex::new(Some(VecDeque::new())) //with_capacity(max_number_of_permits))

        }

    }

    pub fn with_permits(count: usize) -> Self
    {

        Self
        {

            avalible_permits: AtomicUsize::new(count),
            waker_queue: Mutex::new(Some(VecDeque::with_capacity(count)))

        }

    }

    pub fn with_capacity(size: usize) -> Self
    {

        Self
        {

            avalible_permits: AtomicUsize::new(0),
            waker_queue: Mutex::new(Some(VecDeque::with_capacity(size)))

        }

    }

    pub fn avalible_permits(&self) -> usize
    {

        self.avalible_permits.load(Ordering::Acquire)

    }

    pub fn add_permits(&self, count: usize)
    {

        if count == 0
        {

            return;

        }

        self.avalible_permits.fetch_add(count, Ordering::SeqCst);

        //Check for wakers and wake them if present.

    }

    pub fn add_permit(&self)
    {

        self.add_permits(1);

    }

    pub fn remove_permits(&self, count: usize)
    {

        if count == 0
        {

            return;

        }

        self.avalible_permits.fetch_sub(count, Ordering::SeqCst);

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

        let taken_opt_queue;

        {

            let lock_result = self.waker_queue.lock();

            let mut opt_queue;

            match lock_result
            {

                Ok(mg) =>
                {

                    opt_queue = mg;

                }
                Err(err) =>
                {

                    self.waker_queue.clear_poison();

                    opt_queue = err.into_inner();

                }

            }

            taken_opt_queue = opt_queue.take();

        }

        if let Some(mut queue) = taken_opt_queue
        {

            for item in queue.drain(..)
            {

                item.wake();

            }

        }


    }

    pub fn is_closed(&self) -> bool
    {

        let lock_result = self.waker_queue.lock();

        let opt_queue;

        match lock_result
        {

            Ok(mg) =>
            {

                opt_queue = mg;

            }
            Err(err) =>
            {

                self.waker_queue.clear_poison();

                opt_queue = err.into_inner();

            }

        }

        opt_queue.is_none()

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