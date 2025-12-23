
use std::fmt::Display;

use std::future::Future;

use std::sync::{Mutex, MutexGuard};

use std::sync::atomic::{AtomicBool, Ordering};

use std::task::{Poll, Waker};

use std::error::Error;

#[derive(Debug)]
pub struct SingleWakerError
{
}

impl SingleWakerError
{

    pub fn new() -> Self
    {

        Self
        {}

    }

}

impl Display for SingleWakerError
{

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        
        write!(f, "Error: Pre-existing Waker in SingleWakerError")

    }

}

impl Error for SingleWakerError
{    
}

pub struct SingleWaker
{

    waker_mutex: Mutex<Option<Waker>>,
    shouldve_awoken: AtomicBool

}

impl SingleWaker
{

    pub fn new() -> Self
    {

        Self
        {

            waker_mutex: Mutex::new(None),
            shouldve_awoken: AtomicBool::new(false)

        }

    }

    fn get_mg(&self) -> MutexGuard<'_, Option<Waker>>
    {

        let lock_result = self.waker_mutex.lock();

        match lock_result
        {

            Ok(mg) =>
            {

                mg

            }
            Err(err) =>
            {

                self.waker_mutex.clear_poison();

                err.into_inner()

            }

        }

    }

    pub fn wake(&self) -> bool
    {

        let mut mg = self.get_mg();

        if let Some(waker) = mg.take()
        {

            self.shouldve_awoken.store(true, Ordering::Relaxed);

            waker.wake();

            return true;

        }

        false

    }

}

impl Future for SingleWaker
{

    type Output = Result<(), SingleWakerError>;

    fn poll(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> Poll<Self::Output>
    {

        let shouldve_awoken = self.shouldve_awoken.load(Ordering::Relaxed);

        if shouldve_awoken
        {

            return Poll::Ready(Ok(()));

        }

        let mut mg = self.get_mg();
    
        if mg.is_none()
        {

            let waker = cx.waker().clone();

            *mg = Some(waker);

            return Poll::Pending;

        }

        Poll::Ready(Err(SingleWakerError::new()))
        
    }

}

impl Drop for SingleWaker
{

    fn drop(&mut self)
    {
        
        self.wake();
        
    }

}
