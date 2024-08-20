use tokio::{sync::{AcquireError, Semaphore, SemaphorePermit}, time::{error::Elapsed, timeout}};

use delegate::delegate;

use std::{future::Future, sync::atomic::{AtomicUsize, Ordering}, time::Duration};

use crate::ScopedIncrementer;

//For when the queue is empty.

#[derive(Debug)]
pub struct ChannelSemaphore
{

    sem: Semaphore,
    waiting_count: AtomicUsize

}

impl ChannelSemaphore
{

    pub fn new() -> Self
    {

        Self
        {

            sem: Semaphore::new(0),
            waiting_count: AtomicUsize::new(0)

        }

    }

    delegate!
    {

        to self.sem
        {

            pub fn available_permits(&self) -> usize;

            pub fn close(&self);

            pub fn is_closed(&self) -> bool;

        }

    }

    pub fn waiting_count(&self) -> usize
    {

        self.waiting_count.load(Ordering::SeqCst)

    }

    pub fn has_waiters(&self) -> bool
    {

        self.waiting_count() > 0

    }

    fn add_permit(&self)
    {

        self.sem.add_permits(1);

    }

    pub fn try_add_permit(&self) -> bool
    {

        if self.sem.is_closed()
        {

            return false;

        }

        if self.has_waiters()
        {

            self.add_permit();

            return true;

        }

        false

    }

    pub async fn try_acquire(&self) -> Option<Result<SemaphorePermit<'_>, AcquireError>>
    {

        if !self.has_waiters()
        {

            return None;

        }

        let _incremented = ScopedIncrementer::new(&self.waiting_count);

        let res = self.sem.acquire().await;

        Some(res)

    }

    pub async fn acquire_timeout(&self, duration: Duration) -> Result<Result<SemaphorePermit, AcquireError>, Elapsed>
    {
        
        let _incremented = ScopedIncrementer::new(&self.waiting_count);

        let res = self.sem.acquire();

        timeout(duration, res).await

    }

    pub async fn try_acquire_timeout(&self, duration: Duration) -> Option<Result<Result<SemaphorePermit, AcquireError>, Elapsed>>
    {

        if !self.has_waiters()
        {

            return None;

        }
        
        Some(self.acquire_timeout(duration).await)

    }

    /*
    pub async fn try_acquire_future<'a, F>(&'a self) -> Option<(F, ScopedIncrementer)>
        where F: Future<Output = Result<SemaphorePermit<'a>, AcquireError>>
    {

        if !self.has_waiters()
        {

            return None;

        }
        
        let incremented = ScopedIncrementer::new(&self.waiting_count);

        let res = self.sem.acquire();

        Some((res, incremented))

    }
    */

}