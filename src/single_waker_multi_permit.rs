
use std::fmt::Display;

use std::future::Future;

//use std::sync::{Mutex, MutexGuard};

use std::io::SeekFrom;
use std::sync::atomic::{AtomicBool, Ordering};

use std::task::{Poll, Waker};

use std::error::Error;

#[cfg(feature="use_std_sync")]
use std::sync::{Mutex, MutexGuard};

#[cfg(feature="use_std_sync")]
use std::sync::TryLockError;

/*
#[cfg(feature="use_parking_lot_sync")]
use parking_lot::Mutex;

#[cfg(feature="use_parking_lot_fair_sync")]
use parking_lot::FairMutex;
*/

use crate::PreferredMutexType;

#[derive(Debug, PartialEq, Eq)]
pub enum SingleWakerMultiPermitError
{

    Closed,
    Occupied

}

//Disabeld

/*
#[derive(Debug)]
pub struct SingleWakerMultiPermitError
{
}

impl SingleWakerMultiPermitError
{

    pub fn new() -> Self
    {

        Self
        {}

    }

}
*/

impl Display for SingleWakerMultiPermitError
{

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {

        match self
        {

            SingleWakerMultiPermitError::Closed =>
            {

                write!(f, "SingleWakerMultiPermit is closed")

            }
            SingleWakerMultiPermitError::Occupied =>
            {

                write!(f, "SingleWakerMultiPermit is occupied")

            }

        }    

    }

}

impl Error for SingleWakerMultiPermitError
{    
}

#[derive(Debug)]
pub struct SingleWakerMultiPermitInternalState
{

    pub opt_waker: Option<Waker>,
    pub shouldve_awoken: bool, //Option<bool>,
    pub permits: usize

}

impl SingleWakerMultiPermitInternalState
{

    pub fn new() -> Self
    {

        Self
        {

            opt_waker: None,
            shouldve_awoken: false,
            permits: 0

        }

    }
    
}

#[derive(Debug)]
pub struct SingleWakerMultiPermit
{

    internal_mut_state: PreferredMutexType<Option<SingleWakerMultiPermitInternalState>>

}

impl SingleWakerMultiPermit
{

    pub fn new() -> Self
    {

        Self
        {

            internal_mut_state: PreferredMutexType::new(Some(SingleWakerMultiPermitInternalState::new()))

        }

    }

    #[cfg(feature="use_std_sync")]
    fn get_mg(&self) -> MutexGuard<'_, Option<SingleWakerMultiPermitInternalState>>
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

    pub fn avalible_permits(&self) -> Option<usize>
    {

        #[cfg(feature="use_std_sync")]
        let mut mg = self.get_mg();

        #[cfg(any(feature="use_parking_lot_sync", feature="use_parking_lot_fair_sync"))]
        let mut mg = self.internal_mut_state.lock();

        if let Some(val) = &mut *mg
        {

            return Some(val.permits);

        } 

        None

    }

    pub fn is_closed(&self) -> bool
    {

        #[cfg(feature="use_std_sync")]
        let mg = self.get_mg();

        #[cfg(any(feature="use_parking_lot_sync", feature="use_parking_lot_fair_sync"))]
        let mg = self.internal_mut_state.lock();

        mg.is_none()

    }

    pub fn is_occupied(&self) -> Option<bool>
    {

        #[cfg(feature="use_std_sync")]
        let mg = self.get_mg();

        #[cfg(any(feature="use_parking_lot_sync", feature="use_parking_lot_fair_sync"))]
        let mg = self.internal_mut_state.lock();

        if let Some(val) = &*mg
        {

            Some(val.opt_waker.is_some())

        }
        else
        {

            None
            
        }

    }

    pub fn add_permit(&self) -> Option<bool>
    {

        let opt_waker;

        {

            #[cfg(feature="use_std_sync")]
            let mut mg = self.get_mg();

            #[cfg(any(feature="use_parking_lot_sync", feature="use_parking_lot_fair_sync"))]
            let mut mg = self.internal_mut_state.lock();

            if let Some(val) = &mut *mg
            {

                let permits = val.permits;

                if let Some(resultant_permits) = permits.checked_add(1)
                {

                    val.permits = resultant_permits;

                    opt_waker = val.opt_waker.take();

                    val.shouldve_awoken = opt_waker.is_some();

                }
                else
                {

                    return Some(false);
                    
                }

            }
            else
            {

                return None;

            }

        }
     
        if let Some(waker) = opt_waker
        {

            //Wake the waker outside the mg.

            waker.wake();
            
        }

        Some(true)

    }

    /*
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
    */

    pub fn remove_permit(&self) -> Option<bool>
    {

        #[cfg(feature="use_std_sync")]
        let mut mg = self.get_mg();

        #[cfg(any(feature="use_parking_lot_sync", feature="use_parking_lot_fair_sync"))]
        let mut mg = self.internal_mut_state.lock();

        if let Some(val) = &mut *mg
        {

            let permits = val.permits;

            if let Some(resultant_permits) = permits.checked_sub(1)
            {

                val.permits = resultant_permits;

                return Some(true);

            }
            else
            {

                //val.permits = 0;

                return Some(false);
                
            }

        }
    
        None

    }

    pub fn decrement_permits_or_wait<'a>(&'a self) -> SingleWakerMultiPermitDecrementPermitsOrWait<'a>
    {

        SingleWakerMultiPermitDecrementPermitsOrWait::new(self)

    }

    pub fn close(&self)
    {

        let mut opt_waker = None;

        {

            #[cfg(feature="use_std_sync")]
            let mut mg = self.get_mg();

            #[cfg(any(feature="use_parking_lot_sync", feature="use_parking_lot_fair_sync"))]
            let mut mg = self.internal_mut_state.lock();

            if let Some(internal_mut_state) = &mut *mg
            {

                opt_waker = internal_mut_state.opt_waker.take();

                *mg = None;

            }

        }

        if let Some(waker) = opt_waker
        {

            waker.wake();

        }

    }

}

pub struct SingleWakerMultiPermitDecrementPermitsOrWait<'a>
{

    single_waker_multi_permit_ref: &'a SingleWakerMultiPermit,
    is_active: bool

}

impl<'a> SingleWakerMultiPermitDecrementPermitsOrWait<'a>
{

    pub fn new(single_waker_multi_permit_ref: &'a SingleWakerMultiPermit) -> Self
    {

        Self
        {

            single_waker_multi_permit_ref,
            is_active: false                //Is waiting/has waited

        }

    }
    
}

impl<'a> Future for SingleWakerMultiPermitDecrementPermitsOrWait<'a>
{

    type Output = Result<(), SingleWakerMultiPermitError>;

    fn poll(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> Poll<Self::Output>
    {

        #[cfg(feature="use_std_sync")]
        let mut mg = self.single_waker_multi_permit_ref.get_mg();

        #[cfg(any(feature="use_parking_lot_sync", feature="use_parking_lot_fair_sync"))]
        let mut mg = self.single_waker_multi_permit_ref.internal_mut_state.lock();

        if let Some(val) = &mut *mg
        {

            if self.is_active && !val.shouldve_awoken
            {

                return Poll::Pending;

            }

            let permits = val.permits;

            if let Some(new_permits) = permits.checked_sub(1)
            {

                val.permits = new_permits;

                return Poll::Ready(Ok(()));

            }
            else if val.opt_waker.is_none()
            {

                let waker = cx.waker().clone();

                val.opt_waker = Some(waker);

                val.shouldve_awoken = false;

                let self_mut = self.get_mut();

                self_mut.is_active = true;

                return Poll::Pending;

            }
            else
            {

                return Poll::Ready(Err(SingleWakerMultiPermitError::Occupied));
                
            }

        }

        Poll::Ready(Err(SingleWakerMultiPermitError::Closed)) //new()))
        
    }

}

impl<'a> Drop for SingleWakerMultiPermitDecrementPermitsOrWait<'a>
{

    fn drop(&mut self)
    {
        
        if self.is_active
        {

            #[cfg(feature="use_std_sync")]
            let mut mg = self.single_waker_multi_permit_ref.get_mg();

            #[cfg(any(feature="use_parking_lot_sync", feature="use_parking_lot_fair_sync"))]
            let mut mg = self.single_waker_multi_permit_ref.internal_mut_state.lock();

            if let Some(val) = &mut *mg
            {

                val.opt_waker = None;

                //val.shouldve_awoken = false;

            }

        }
    
    }

}
