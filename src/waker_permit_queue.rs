use std::error::Error;

use std::fmt::Display;

use std::future::Future;

#[cfg(feature="use_std_sync")]
use std::sync::{Mutex, MutexGuard};

use std::sync::atomic::{AtomicUsize, Ordering};

use std::collections::{HashMap, HashSet, VecDeque};

use std::task::{Poll, Waker};

//use core::result::Result;

use inc_dec::{IncDecSelf, IntIncDecSelf};

use paste::paste;

use accessorise::impl_get_val;

use crate::QueuedWaker;

use std::fmt::Debug;

#[cfg(feature="use_std_sync")]
use std::sync::TryLockError;

#[cfg(feature="use_parking_lot_sync")]
use parking_lot::Mutex;

#[cfg(feature="use_parking_lot_fair_sync")]
use parking_lot::FairMutex;

use super::PreferredMutexType;

#[derive(Debug)]
pub struct WakerPermitQueueInternals
{

    pub no_permits_queue: VecDeque<QueuedWaker>,
    pub id: usize,
    pub active_ids: HashMap<usize, bool>, //Waker Handle, should've awoken
    pub permits: usize

}

impl WakerPermitQueueInternals
{

    pub fn new() -> Self
    {

        Self
        {

            no_permits_queue: VecDeque::new(),
            id: 0,
            active_ids: HashMap::new(),
            permits: 0

        }

    }

    pub fn with_capacity(capacity: usize) -> Self
    {

        Self
        {

            no_permits_queue: VecDeque::with_capacity(capacity),
            id: 0,
            active_ids: HashMap::with_capacity(capacity),
            permits: 0

        }

    }

    pub fn with_permits(permits: usize) -> Self
    {

        Self
        {

            no_permits_queue: VecDeque::new(), //VecDeque::with_capacity(permits),
            id: 0,
            active_ids: HashMap::new(), //HashMap::with_capacity(permits),
            permits: permits

        }

    }

    pub fn with_capacity_and_permits(capacity_and_permits: usize) -> Self
    {

        Self
        {

            no_permits_queue: VecDeque::with_capacity(capacity_and_permits),
            id: 0,
            active_ids: HashMap::with_capacity(capacity_and_permits),
            permits: capacity_and_permits

        }

    }

    pub fn with_capacity_and_permits_separate(capacity: usize, permits: usize) -> Self
    {

        Self
        {

            no_permits_queue: VecDeque::with_capacity(capacity),
            id: 0,
            active_ids: HashMap::with_capacity(capacity),
            permits

        }

    }

}

/*
impl Debug for WakerPermitQueueInternals
{

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WakerPermitQueueInternals").field("no_permits_queue", &self.no_permits_queue).field("id", &self.id).field("active_ids", &self.active_ids).field("permits", &self.permits).finish()
    }

}
*/

#[derive(Debug)]
pub struct WakerPermitQueue
{

    internals: PreferredMutexType<Option<WakerPermitQueueInternals>>

}

impl WakerPermitQueue
{

    pub fn new() -> Self
    {

        Self
        {

            internals: PreferredMutexType::new(Some(WakerPermitQueueInternals::new()))

        }

    }

    pub fn with_capacity(capacity: usize) -> Self
    {

        Self
        {

            internals: PreferredMutexType::new(Some(WakerPermitQueueInternals::with_capacity(capacity)))

        }

    }

    pub fn with_permits(permits: usize) -> Self
    {

        Self
        {

            internals: PreferredMutexType::new(Some(WakerPermitQueueInternals::with_capacity(permits)))

        }

    }

    pub fn with_capacity_and_permits(capacity_and_permits: usize) -> Self
    {

        Self
        {

            internals: PreferredMutexType::new(Some(WakerPermitQueueInternals::with_capacity_and_permits(capacity_and_permits)))

        }

    }

    pub fn with_capacity_and_permits_separate(capacity: usize, permits: usize) -> Self
    {

        Self
        {

            internals: PreferredMutexType::new(Some(WakerPermitQueueInternals::with_capacity_and_permits_separate(capacity, permits)))

        }

    }

    #[cfg(feature="use_std_sync")]
    fn get_mg(&self) -> MutexGuard<'_, Option<WakerPermitQueueInternals>>
    {

        let lock_result = self.internals.lock();

        match lock_result
        {

            Ok(mg) =>
            {

                mg

            }
            Err(err) =>
            {

                self.internals.clear_poison();

                err.into_inner()

            }

        }

    }

    #[cfg(feature="use_std_sync")]
    fn try_get_mg(&self) -> Option<MutexGuard<'_, Option<WakerPermitQueueInternals>>>
    {

        let lock_result = self.internals.try_lock();

        match lock_result
        {

            Ok(val) =>
            {

                Some(val)

            }
            Err(err) =>
            {

                match err
                {

                    TryLockError::Poisoned(poison_error) =>
                    {

                        self.internals.clear_poison();

                        Some(poison_error.into_inner())

                    }
                    TryLockError::WouldBlock =>
                    {

                        None

                    }

                }

            }

        }

    }

    pub fn avalible_permits(&self) -> Option<usize>
    {

        #[cfg(feature="use_std_sync")]
        let mut mg = self.get_mg();

        #[cfg(any(feature="use_parking_lot_sync", feature="use_parking_lot_fair_sync"))]
        let mut mg = self.internals.lock();

        if let Some(val) = &mut *mg
        {

            return Some(val.permits);

        } 

        None

    }

    //Disabled
    
    /*
    pub fn try_add_permits(&self, count: usize) -> bool
    {

        if count == 0
        {

            return false;

        }

        let mut mg;

        let opt_mg = self.try_get_mg();

        if let Some(val) = opt_mg
        {

            mg = val;

        }
        else
        {

            return false;


        }

        if let Some(val) = &mut *mg
        {

            let permits = val.permits;

            if let Some(mut permits) = permits.checked_add(count)
            {

                val.permits = permits;

                while permits > 0
                {

                    //Check for wakers and wake them if present.

                    let opt_front_waker = val.no_permits_queue.pop_front();

                    if let Some(front_waker) = opt_front_waker
                    {

                        if let Some(shouldve_awoken) = val.active_ids.get_mut(&front_waker.id())
                        {

                            *shouldve_awoken = true;

                        }

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

    pub fn try_add_permit(&self) -> bool
    {

        self.try_add_permits(1)
        
    }
    */

    pub fn add_permits(&self, count: usize, buffer: &mut VecDeque<QueuedWaker>) -> bool
    {

        if count == 0
        {

            return false;

        }

        let mut added_permits = false;

        {

            #[cfg(feature="use_std_sync")]
            let mut mg = self.get_mg();

            #[cfg(any(feature="use_parking_lot_sync", feature="use_parking_lot_fair_sync"))]
            let mut mg = self.internals.lock();

            if let Some(val) = &mut *mg
            {

                let permits = val.permits;

                if let Some(mut permits) = permits.checked_add(count)
                {

                    added_permits = true;

                    val.permits = permits;

                    while permits > 0
                    {

                        //Check for wakers and wake them if present.

                        let opt_front_waker = val.no_permits_queue.pop_front();

                        if let Some(front_waker) = opt_front_waker
                        {

                            if let Some(shouldve_awoken) = val.active_ids.get_mut(&front_waker.id())
                            {

                                *shouldve_awoken = true;

                            }

                            buffer.push_back(front_waker);

                            //does the waker need to be marked as "should wake"?

                            //front_waker.wake();

                        }
                        else
                        {

                            break;

                        }

                        permits.mm();
                        
                    }

                    //return true;

                }

            }

        }

        for item in buffer.drain(..)
        {

            item.wake();

        }

        added_permits

    }

    pub fn add_permit(&self) -> bool
    {

        let mut opt_waker = None;

        {

            #[cfg(feature="use_std_sync")]
            let mut mg = self.get_mg();

            #[cfg(any(feature="use_parking_lot_sync", feature="use_parking_lot_fair_sync"))]
            let mut mg = self.internals.lock();

            if let Some(val) = &mut *mg
            {

                let permits = val.permits;

                if let Some(permits) = permits.checked_add(1)
                {

                    val.permits = permits;

                    //Check for wakers and wake them if present.

                    opt_waker = val.no_permits_queue.pop_front();

                    if let Some(front_waker) = &opt_waker
                    {

                        if let Some(shouldve_awoken) = val.active_ids.get_mut(&front_waker.id())
                        {

                            *shouldve_awoken = true;

                        }

                        //opt_waker = Some(front_waker);

                        //does the waker need to be marked as "should wake"?

                        //front_waker.wake();
                    
                    }

                }
                else
                {

                    return false;
                    
                }

            }

        }

        if let Some(waker) = opt_waker
        {

            //Wake the waker outside the mg.

            waker.wake();

            //return true;
            
        }

        true

    }

    pub fn remove_permits(&self, count: usize) -> bool
    {

        if count == 0
        {

            return false;

        }

        #[cfg(feature="use_std_sync")]
        let mut mg = self.get_mg();

        #[cfg(any(feature="use_parking_lot_sync", feature="use_parking_lot_fair_sync"))]
        let mut mg = self.internals.lock();

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

    pub fn remove_permit(&self) -> bool
    {

        self.remove_permits(1)

    }

    pub fn try_decrement_permits(&self) -> bool
    {

        #[cfg(feature="use_std_sync")]
        let opt_mg = self.try_get_mg();

        #[cfg(any(feature="use_parking_lot_sync", feature="use_parking_lot_fair_sync"))]
        let opt_mg = self.internals.try_lock();

        if let Some(mut mg) = opt_mg
        {

            match &mut *mg
            {

                Some(val) =>
                {

                    //"Take" a permit.

                    let permits = val.permits;

                    if let Some(new_permits) = permits.checked_sub(1)
                    {

                        val.permits = new_permits;

                        return true;

                    }

                }
                None => {}

            }

        }

        false        

    }
    pub fn decrement_permits_or_wait<'a>(&'a self) -> WakerPermitQueueDecrementPermitsOrWait<'a>
    {

        WakerPermitQueueDecrementPermitsOrWait::new(self)

    }

    pub fn close(&self)
    {

        let opt_internals;

        {

            #[cfg(feature="use_std_sync")]
            let mut mg = self.get_mg();

            #[cfg(any(feature="use_parking_lot_sync", feature="use_parking_lot_fair_sync"))]
            let mut mg = self.internals.lock();

            opt_internals = mg.take();

        }

        if let Some(mut internals) = opt_internals
        {

            for item in internals.no_permits_queue.drain(..)
            {

                item.wake();

            }

        }

    }

    pub fn is_closed(&self) -> bool
    {

        #[cfg(feature="use_std_sync")]
        let mg = self.get_mg();

        #[cfg(any(feature="use_parking_lot_sync", feature="use_parking_lot_fair_sync"))]
        let mg = self.internals.lock();

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

#[derive(Debug, PartialEq)]
pub struct WakerPermitQueueClosedError
{
}

impl WakerPermitQueueClosedError
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

impl Display for WakerPermitQueueClosedError
{

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        
        write!(f, "WakerPermitQueue Closed")

    }

}

impl Error for WakerPermitQueueClosedError
{    
}

pub struct WakerPermitQueueDecrementPermitsOrWait<'a>
{

    waker_permit_queue_ref: &'a WakerPermitQueue,
    opt_waker_id: Option<usize>

}

impl<'a> WakerPermitQueueDecrementPermitsOrWait<'a>
{

    pub fn new(waker_permit_queue_ref: &'a WakerPermitQueue) -> Self
    {

        Self
        {

            waker_permit_queue_ref,
            opt_waker_id: None

        }

    }
    
}

//Handles "sleeping", "waking" and permit incrementation/decrementation.

impl Future for WakerPermitQueueDecrementPermitsOrWait<'_>
{

    type Output = Result<(), WakerPermitQueueClosedError>;

    fn poll(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> Poll<Self::Output>
    {

        match self.opt_waker_id
        {

            Some(id) =>
            {

                #[cfg(feature="use_std_sync")]
                let mut mg = self.waker_permit_queue_ref.get_mg();

                #[cfg(any(feature="use_parking_lot_sync", feature="use_parking_lot_fair_sync"))]
                let mut mg = self.waker_permit_queue_ref.internals.lock();

                match &mut *mg
                {

                    Some(val) =>
                    {

                        //Make sure this is a proper wakup.

                        if let Some(shouldve_awoken) = val.active_ids.get(&id)
                        {

                            if *shouldve_awoken
                            {

                                //"Take" a permit.

                                let permits = val.permits;

                                if let Some(new_permits) = permits.checked_sub(1)
                                {

                                    val.permits = new_permits;

                                }
                                else
                                {

                                    //The value of the permits should've been greater that one so this tasks get to proceed if it wakes up spuriously next time.

                                    let waker = cx.waker().clone();

                                    let queued_waker = QueuedWaker::new(waker, id);

                                    val.no_permits_queue.push_back(queued_waker);

                                    return Poll::Pending;
                                    
                                }

                                val.active_ids.remove(&id);

                                //Drop the mg here?

                                //Make sure the waker id is dropped locally as well.

                                let self_mut = self.get_mut();

                                /*
                                let self_mut = unsafe
                                {
                                    
                                    self.get_unchecked_mut()

                                };
                                */

                                self_mut.opt_waker_id = None;

                                return Poll::Ready(Ok(()));

                            }

                        }
                        else
                        {

                            //make sure this Task doen't get trapped if there's an error. 

                            return Poll::Ready(Ok(()));

                        }

                        /*
                        if !val.active_ids.contains_key(&id)
                        {

                            //The task has been successfully awoken.

                            return Poll::Ready(Ok(()));

                        }
                        */

                    }
                    None =>
                    {

                        return Poll::Ready(WakerPermitQueueClosedError::err());

                    }

                }

            }
            None =>
            {

                //The task is going to "sleep". Update the WQI so it can be woken up later.

                let mut inserted = false;

                let waker = cx.waker().clone();

                let mut id = 0;

                //

                #[cfg(feature="use_std_sync")]
                let mut mg = self.waker_permit_queue_ref.get_mg();

                #[cfg(any(feature="use_parking_lot_sync", feature="use_parking_lot_fair_sync"))]
                let mut mg = self.waker_permit_queue_ref.internals.lock();

                match &mut *mg
                {

                    Some(val) =>
                    {

                        //Is there a queue?

                        if val.no_permits_queue.is_empty()
                        {

                            //"Take" a permit.

                            let permits = val.permits;

                            if let Some(new_permits) = permits.checked_sub(1)
                            {

                                val.permits = new_permits;

                                //There was at least one permit available so we don't need to wait.

                                return Poll::Ready(Ok(()));

                            }

                        }

                        while !inserted
                        {

                            //Find the next avalible id.

                            id = val.id.wpp();

                            inserted = val.active_ids.insert(id, false).is_none(); //.is_some();
                            
                        }

                        let queued_waker = QueuedWaker::new(waker, id);

                        val.no_permits_queue.push_back(queued_waker);

                    }
                    None =>
                    {

                        return Poll::Ready(WakerPermitQueueClosedError::err());

                    }

                }

                //

                //Store the id in the future.

                let self_mut = self.get_mut();

                /*
                let self_mut = unsafe
                {
                    
                    self.get_unchecked_mut()

                };
                */

                self_mut.opt_waker_id = Some(id);                     

            }

        }

        Poll::Pending
        
    }

}

impl Drop for WakerPermitQueueDecrementPermitsOrWait<'_>
{

    fn drop(&mut self)
    {

        // This type does not have any address-senstive states, therefore it does not pin the self reference.

        // https://doc.rust-lang.org/std/pin/index.html#implementing-drop-for-types-with-address-sensitive-states

        // Make sure that the waker id gets removed.
        
        if let Some(id) = self.opt_waker_id
        {

            #[cfg(feature="use_std_sync")]
            let mut mg = self.waker_permit_queue_ref.get_mg();

            #[cfg(any(feature="use_parking_lot_sync", feature="use_parking_lot_fair_sync"))]
            let mut mg = self.waker_permit_queue_ref.internals.lock();

            if let Some(wqi) = &mut *mg
            {

                //Remove the relevant entry from the active ids HashMap.

                wqi.active_ids.remove(&id);

                let mut index = 0;

                let mut index_found = false;

                //Remove the queued waker.

                for item in wqi.no_permits_queue.iter()
                {

                    if id == item.id()
                    {

                        index_found = true;

                        break;

                    }  

                    index.pp();
                    
                }

                if index_found
                {

                    wqi.no_permits_queue.remove(index);

                }

            }
            
        }

    }

}

/*
impl Debug for WakerPermitQueue
{

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WakerPermitQueue").field("internals", &self.internals).finish()
    }

}
*/