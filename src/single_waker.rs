
use std::fmt::Display;

use std::future::Future;

//use std::sync::{Mutex, MutexGuard};

use std::sync::atomic::{AtomicBool, Ordering};

use std::task::{Poll, Waker};

use std::error::Error;

#[cfg(feature="use_std_sync")]
use std::sync::{Mutex, MutexGuard};

#[cfg(feature="use_std_sync")]
use std::sync::TryLockError;

#[cfg(feature="use_parking_lot_sync")]
use parking_lot::Mutex;

#[cfg(feature="use_parking_lot_fair_sync")]
use parking_lot::FairMutex;


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

pub struct SingleWakerInternalState
{

    pub opt_waker: Option<Waker>,
    pub shouldve_awoken: bool

}

impl SingleWakerInternalState
{

    pub fn new() -> Self
    {

        Self
        {

            opt_waker: None,
            shouldve_awoken: false

        }

    }
    
}

pub struct SingleWaker
{

    internal_state: Mutex<SingleWakerInternalState>

    //waker_mutex: Mutex<Option<Waker>>,
    //shouldve_awoken: AtomicBool

}

impl SingleWaker
{

    pub fn new() -> Self
    {

        Self
        {

            internal_state: Mutex::new(SingleWakerInternalState::new())

            //waker_mutex: Mutex::new(None),
            //shouldve_awoken: AtomicBool::new(false)

        }

    }

    #[cfg(feature="use_std_sync")]
    fn get_mg(&self) -> MutexGuard<'_, SingleWakerInternalState> //Option<Waker>>
    {

        let lock_result = self.internal_state.lock();

        match lock_result
        {

            Ok(mg) =>
            {

                mg

            }
            Err(err) =>
            {

                self.internal_state.clear_poison();

                err.into_inner()

            }

        }

    }

    pub fn wake(&self) -> bool
    {

        #[cfg(feature="use_std_sync")]
        let mut mg = self.get_mg();

        #[cfg(any(feature="use_parking_lot_sync", feature="use_parking_lot_fair_sync"))]
        let mut mg = self.internals.lock();

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
