use tokio::{sync::{AcquireError, Semaphore, SemaphorePermit}, time::{error::Elapsed, timeout}};

use delegate::delegate;

use std::{future::Future, sync::atomic::{AtomicUsize, Ordering}, time::Duration};

///
///Receivers side: Permits to receive popped objects. Permit count starts at zero (empty queue).
///
///Senders side: Permits to send objects. Permit count starts at the length of the queue (On permit for each empty sot in the queue (if applicable)).
///
#[derive(Debug)]
pub struct SemaphoreController
{

    sem: Semaphore

}

impl SemaphoreController
{

    pub fn new() -> Self
    {

        Self
        {

            sem: Semaphore::new(0)

        }

    }

    pub fn with_permits(count: usize) -> Self
    {
        
        Self
        {

            sem: Semaphore::new(count)

        }

    }

    delegate!
    {

        to self.sem
        {

            pub fn available_permits(&self) -> usize;

            pub fn add_permits(&self, n: usize);

            pub fn close(&self);

            pub fn is_closed(&self) -> bool;

            pub async fn acquire(&self) -> Result<SemaphorePermit<'_>, AcquireError>;

        }

    }

    pub fn has_permits(&self) -> bool
    {

        self.sem.available_permits() > 0

    }

    pub fn add_permit(&self)
    {

        self.sem.add_permits(1);

    }

    pub fn forget_permit(&self) -> usize
    {

       self.sem.forget_permits(1)

    }

    pub async fn acquire_timeout(&self, duration: Duration) -> Result<Result<SemaphorePermit, AcquireError>, Elapsed>
    {

        let res = self.sem.acquire();

        timeout(duration, res).await

    }

}