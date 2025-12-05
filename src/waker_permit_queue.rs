use std::error::Error;

use std::fmt::Display;

use std::future::Future;

use std::sync::{Mutex, MutexGuard};

use std::sync::atomic::{AtomicUsize, Ordering};

use std::collections::{HashMap, HashSet, VecDeque};

use std::task::{Poll, Waker};

//use core::result::Result;

use inc_dec::{IncDecSelf, IntIncDecSelf};
use paste::paste;

use accessorise::impl_get_val;

use crate::QueuedWaker;

use std::sync::TryLockError;

pub struct WakerPermitQueueInternals
{

    pub queue: VecDeque<QueuedWaker>,
    pub handle: usize,
    pub active_handles: HashMap<usize, bool>, //Waker Handle, should've awoken
    pub permits: usize

}

impl WakerPermitQueueInternals
{

    pub fn new() -> Self
    {

        Self
        {

            queue: VecDeque::new(),
            handle: 0,
            active_handles: HashMap::new(),
            permits: 0

        }

    }

    pub fn with_capacity(capacity: usize) -> Self
    {

        Self
        {

            queue: VecDeque::with_capacity(capacity),
            handle: 0,
            active_handles: HashMap::with_capacity(capacity),
            permits: 0

        }

    }

    pub fn with_permits(permits: usize) -> Self
    {

        Self
        {

            queue: VecDeque::with_capacity(permits),
            handle: 0,
            active_handles: HashMap::with_capacity(permits),
            permits: permits

        }

    }

    pub fn with_capacity_and_permits(capacity: usize, permits: usize) -> Self
    {

        Self
        {

            queue: VecDeque::with_capacity(capacity),
            handle: 0,
            active_handles: HashMap::with_capacity(capacity),
            permits

        }

    }

}

pub struct WakerPermitQueue
{

    limted_notifier: Mutex<Option<WakerPermitQueueInternals>>

}

impl WakerPermitQueue
{

    pub fn new() -> Self
    {

        Self
        {

            limted_notifier: Mutex::new(Some(WakerPermitQueueInternals::new()))

        }

    }

    pub fn with_capacity(capacity: usize) -> Self
    {

        Self
        {

            limted_notifier: Mutex::new(Some(WakerPermitQueueInternals::with_capacity(capacity)))

        }

    }

    pub fn with_permits(permits: usize) -> Self
    {

        Self
        {

            limted_notifier: Mutex::new(Some(WakerPermitQueueInternals::with_capacity(permits)))

        }

    }

    pub fn with_capacity_and_permits(capacity: usize, permits: usize) -> Self
    {

        Self
        {

            limted_notifier: Mutex::new(Some(WakerPermitQueueInternals::with_capacity_and_permits(capacity, permits)))

        }

    }

    fn get_mg(&self) -> MutexGuard<'_, Option<WakerPermitQueueInternals>>
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

    fn try_get_mg(&self) -> Option<MutexGuard<'_, Option<WakerPermitQueueInternals>>>
    {

        let lock_result = self.limted_notifier.try_lock();

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

                        self.limted_notifier.clear_poison();

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

        let mut mg = self.get_mg();

        if let Some(val) = &mut *mg
        {

            return Some(val.permits);

        } 

        None

    }

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

                    let opt_front_waker = val.queue.pop_front();

                    if let Some(front_waker) = opt_front_waker
                    {

                        if let Some(shouldve_awoken) = val.active_handles.get_mut(&front_waker.handle())
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

    pub fn add_permits(&self, count: usize) -> bool
    {

        if count == 0
        {

            return false;

        }

        let mut mg = self.get_mg();

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

                        if let Some(shouldve_awoken) = val.active_handles.get_mut(&front_waker.handle())
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

    pub fn add_permit(&self) -> bool
    {

        self.add_permits(1)

    }

    pub fn remove_permits(&self, count: usize) -> bool
    {

        if count == 0
        {

            return false;

        }

        let mut mg = self.get_mg();

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

    pub fn try_decrement_permits(&self) -> bool
    {

        let opt_mg = self.try_get_mg();

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

            let mut mg = self.get_mg();

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

        let mg = self.get_mg();

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
    opt_waker_handle: Option<usize>

}

impl<'a> WakerPermitQueueDecrementPermitsOrWait<'a>
{

    pub fn new(waker_permit_queue_ref: &'a WakerPermitQueue) -> Self
    {

        Self
        {

            waker_permit_queue_ref,
            opt_waker_handle: None

        }

    }
    
}

//Handles "sleeping", "waking" and permit incrementation/decrementation.

impl Future for WakerPermitQueueDecrementPermitsOrWait<'_>
{

    type Output = Result<(), WakerPermitQueueClosedError>;

    fn poll(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> Poll<Self::Output>
    {

        match self.opt_waker_handle
        {

            Some(handle) =>
            {

                let mut mg = self.waker_permit_queue_ref.get_mg();

                match &mut *mg
                {

                    Some(val) =>
                    {

                        //Make sure this is a proper wakup.

                        if let Some(shouldve_awoken) = val.active_handles.get(&handle)
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

                                    return Poll::Pending;
                                    
                                }

                                val.active_handles.remove(&handle);

                                //Drop the mg here?

                                //Make sure the waker handle is dropped locally as well.

                                let self_mut = self.get_mut();

                                /*
                                let self_mut = unsafe
                                {
                                    
                                    self.get_unchecked_mut()

                                };
                                */

                                self_mut.opt_waker_handle = None;

                                return Poll::Ready(Ok(()));

                            }

                        }
                        else
                        {

                            //make sure this Task doen't get trapped if there's an error. 

                            return Poll::Ready(Ok(()));

                        }

                        /*
                        if !val.active_handles.contains_key(&handle)
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

                let mut handle = 0;

                //

                let mut mg = self.waker_permit_queue_ref.get_mg();

                match &mut *mg
                {

                    Some(val) =>
                    {

                        //Is there a queue?

                        if val.queue.is_empty()
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

                            //Find the next avalible handle.

                            handle = val.handle.wpp();

                            inserted = val.active_handles.insert(handle, false).is_some();
                            
                        }

                        let queued_waker = QueuedWaker::new(waker, handle);

                        val.queue.push_back(queued_waker);

                    }
                    None =>
                    {

                        return Poll::Ready(WakerPermitQueueClosedError::err());

                    }

                }

                //

                //Store the handle in the future.

                let self_mut = self.get_mut();

                /*
                let self_mut = unsafe
                {
                    
                    self.get_unchecked_mut()

                };
                */

                self_mut.opt_waker_handle = Some(handle);                     

            }

        }

        Poll::Pending
        
    }

}

impl Drop for WakerPermitQueueDecrementPermitsOrWait<'_>
{

    fn drop(&mut self)
    {

        // Make sure that the waker handle gets removed.
        
        if let Some(handle) = self.opt_waker_handle
        {

            let mut mg = self.waker_permit_queue_ref.get_mg();

            if let Some(wqi) = &mut *mg
            {

                //Remove the relevant entry from the active handles HashMap.

                wqi.active_handles.remove(&handle);

                let mut index = 0;

                let mut index_found = false;

                //Remove the queued waker.

                for item in wqi.queue.iter()
                {

                    if handle == item.handle()
                    {

                        index_found = true;

                        break;

                    }  

                    index.pp();
                    
                }

                if index_found
                {

                    wqi.queue.remove(index);

                }

            }
            
        }

    }

}