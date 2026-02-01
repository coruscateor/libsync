
use std::fmt::Display;

use std::future::Future;

//use std::sync::{Mutex, MutexGuard};

use std::sync::atomic::{AtomicBool, Ordering};

use std::task::{Poll, Waker};

use std::error::Error;

#[cfg(feature="use_std_sync")]
use std::sync::{Mutex, MutexGuard};

#[cfg(feature="use_parking_lot_sync")]
use parking_lot::Mutex;

#[cfg(feature="use_parking_lot_fair_sync")]
use parking_lot::FairMutex;

use crate::PreferredMutexType;

#[derive(Debug, PartialEq, Eq)]
pub enum SingleWakerError
{

    Closed,
    Occupied

}

impl Display for SingleWakerError
{

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {

        match self
        {

            SingleWakerError::Closed =>
            {

                write!(f, "SingleWaker is closed")

            }
            SingleWakerError::Occupied =>
            {

                write!(f, "SingleWaker is occupied")

            }

        }    

    }

}

impl Error for SingleWakerError
{    
}

/*
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
        
        write!(f, "Pre-existing Waker in SingleWakerError")

    }

}

impl Error for SingleWakerError
{    
}
*/

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

    internal_mut_state: PreferredMutexType<Option<SingleWakerInternalState>>

    //waker_mutex: Mutex<Option<Waker>>,
    //shouldve_awoken: AtomicBool

}

impl SingleWaker
{

    pub fn new() -> Self
    {

        Self
        {

            internal_mut_state: PreferredMutexType::new(Some(SingleWakerInternalState::new()))

            //waker_mutex: Mutex::new(None),
            //shouldve_awoken: AtomicBool::new(false)

        }

    }

    #[cfg(feature="use_std_sync")]
    fn get_mg(&self) -> MutexGuard<'_, Option<SingleWakerInternalState>> //Option<Waker>>
    {

        let lock_result = self.internal_mut_state.lock();

        match lock_result
        {

            Ok(mg) =>
            {

                mg

            }
            Err(err) =>
            {

                self.internal_mut_state.clear_poison();

                err.into_inner()

            }

        }

    }

    pub fn shouldve_awoken(&self) -> Option<bool>
    {

        #[cfg(feature="use_std_sync")]
        let mg = self.get_mg();

        #[cfg(any(feature="use_parking_lot_sync", feature="use_parking_lot_fair_sync"))]
        let mg = self.internal_mut_state.lock();

        if let Some(val) = &*mg
        {

            Some(val.shouldve_awoken)

        }
        else
        {

            None
            
        }

    }

    pub fn is_closed(&self) -> bool
    {

        #[cfg(feature="use_std_sync")]
        let mg = self.get_mg();

        #[cfg(any(feature="use_parking_lot_sync", feature="use_parking_lot_fair_sync"))]
        let mg = self.internal_mut_state.lock();

        mg.is_none()

    }

    pub fn wake(&self) -> Option<bool>
    {

        #[cfg(feature="use_std_sync")]
        let mut mg = self.get_mg();

        #[cfg(any(feature="use_parking_lot_sync", feature="use_parking_lot_fair_sync"))]
        let mut mg = self.internal_mut_state.lock();

        if let Some(val) = &mut *mg
        {

            if let Some(waker) = val.opt_waker.take()
            {

                val.shouldve_awoken = true;

                waker.wake();

                return Some(true);

            }

            return Some(false);

        }

        None

    }

    pub fn wait<'a>(&'a self) -> SingleWakerWaiter<'a>
    {

        SingleWakerWaiter::new(self)

    }

    pub fn close(&self)
    {

        #[cfg(feature="use_std_sync")]
        let mut mg = self.get_mg();

        #[cfg(any(feature="use_parking_lot_sync", feature="use_parking_lot_fair_sync"))]
        let mut mg = self.internal_mut_state.lock();

        if let Some(val) = &mut *mg
        {

            let opt_waker = val.opt_waker.take();

            *mg = None;

            if let Some(waker) = opt_waker
            {

                //val.shouldve_awoken = true;

                waker.wake();

            }

        }

    }

}

impl Drop for SingleWaker
{

    fn drop(&mut self)
    {
        
        self.close();

    }

}

pub struct SingleWakerWaiter<'a>
{

    single_waiter_ref: &'a SingleWaker

}

impl<'a> SingleWakerWaiter<'a>
{

    pub fn new(single_waiter_ref: &'a SingleWaker) -> Self
    {

        Self
        {

            single_waiter_ref

        }

    }

}

impl<'a> Future for SingleWakerWaiter<'a>
{

    type Output = Result<(), SingleWakerError>;

    fn poll(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> Poll<Self::Output>
    {

        #[cfg(feature="use_std_sync")]
        let mut mg = self.single_waiter_ref.get_mg();

        #[cfg(any(feature="use_parking_lot_sync", feature="use_parking_lot_fair_sync"))]
        let mut mg = self.single_waiter_ref.internal_mut_state.lock();

        if let Some(val) = &mut *mg
        {

            if val.opt_waker.is_some()
            {

                return Poll::Ready(Err(SingleWakerError::Occupied));

            }

            if val.shouldve_awoken
            {

                return Poll::Ready(Ok(()));

            }

            let waker = cx.waker().clone();

            val.shouldve_awoken = false;

            val.opt_waker = Some(waker);

            return Poll::Pending;
            
        }

        Poll::Ready(Err(SingleWakerError::Closed)) //::new()))
        
    }

}
