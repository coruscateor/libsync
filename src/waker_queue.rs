use std::error::Error;

use std::fmt::Display;

use std::future::Future;

#[cfg(feature="use_std_sync")]
use std::sync::{Mutex, MutexGuard};

use std::sync::atomic::{AtomicUsize, Ordering};

use std::collections::{HashMap, HashSet, VecDeque};

use std::task::{Poll, Waker};

use pastey::paste;

use accessorise::impl_val_getter;

use inc_dec::{IncDecSelf, IntIncDecSelf};

use crate::{PreferredMutexType, QueuedWaker};

pub struct WakerQueueInternals
{

    pub queue: VecDeque<QueuedWaker>,
    pub latest_id: usize,
    pub active_ids: HashMap<usize, bool>

}

impl WakerQueueInternals
{

    pub fn new() -> Self
    {

        Self
        {

            queue: VecDeque::new(),
            latest_id: 0,
            active_ids: HashMap::new()

        }

    }

    pub fn with_capacity(capacity: usize) -> Self
    {

        Self
        {

            queue: VecDeque::with_capacity(capacity),
            latest_id: 0,
            active_ids: HashMap::with_capacity(capacity)

        }

    }

}

pub struct WakerQueue
{

    waker_queue_internals: PreferredMutexType<Option<WakerQueueInternals>>

}

impl WakerQueue
{

    pub fn new() -> Self
    {

        Self
        {

            waker_queue_internals: PreferredMutexType::new(Some(WakerQueueInternals::new()))

        }

    }

    pub fn with_capacity(size: usize) -> Self
    {

        Self
        {

            waker_queue_internals: PreferredMutexType::new(Some(WakerQueueInternals::with_capacity(size)))

        }

    }

    #[cfg(feature="use_std_sync")]
    fn get_mg(&self) -> MutexGuard<'_, Option<WakerQueueInternals>>
    {

        let lock_result = self.waker_queue_internals.lock();

        match lock_result
        {

            Ok(mg) =>
            {

                mg

            }
            Err(err) =>
            {

                self.waker_queue_internals.clear_poison();

                err.into_inner()

            }

        }

    }

    pub fn active_ids_len(&self) -> Option<usize>
    {

        #[cfg(feature="use_std_sync")]
        let mut mg = self.get_mg();

        #[cfg(any(feature="use_parking_lot_sync", feature="use_parking_lot_fair_sync"))]
        let mut mg = self.waker_queue_internals.lock();

        if let Some(val) = &mut *mg
        {

            return Some(val.active_ids.len());

        } 

        None

    }

    pub fn active_ids_capacity(&self) -> Option<usize>
    {

        #[cfg(feature="use_std_sync")]
        let mut mg = self.get_mg();

        #[cfg(any(feature="use_parking_lot_sync", feature="use_parking_lot_fair_sync"))]
        let mut mg = self.waker_queue_internals.lock();

        if let Some(val) = &mut *mg
        {

            return Some(val.active_ids.capacity());

        } 

        None

    }

    pub fn is_closed(&self) -> bool
    {

        #[cfg(feature="use_std_sync")]
        let mg = self.get_mg();

        #[cfg(any(feature="use_parking_lot_sync", feature="use_parking_lot_fair_sync"))]
        let mg = self.waker_queue_internals.lock();

        mg.is_none()

    }

    pub fn wake_me<'a>(&'a self) -> WakerQueueWakeMe<'a>
    {

        WakerQueueWakeMe::new(self)

    }

    pub fn notify_one(&self) -> Option<bool>
    {

        let waker;

        {

            #[cfg(feature="use_std_sync")]
            let mut mg = self.get_mg();

            #[cfg(any(feature="use_parking_lot_sync", feature="use_parking_lot_fair_sync"))]
            let mut mg = self.waker_queue_internals.lock();

            match &mut *mg
            {

                Some(val) =>
                {

                    if let Some(front_waker) = val.queue.pop_front()
                    {

                        if let Some(shouldve_awoken) = val.active_ids.get_mut(&front_waker.id())
                        {

                            *shouldve_awoken = true;

                        }

                        waker = front_waker;

                    }
                    else
                    {

                        return Some(false);
                        
                    }

                }
                None =>
                {
                    
                   return None;

                }

            }

        }

        waker.wake();

        Some(true)

    }

    pub fn notify_last(&self) -> Option<bool>
    {

        let waker;

        {

            #[cfg(feature="use_std_sync")]
            let mut mg = self.get_mg();

            #[cfg(any(feature="use_parking_lot_sync", feature="use_parking_lot_fair_sync"))]
            let mut mg = self.waker_queue_internals.lock();

            match &mut *mg
            {

                Some(val) =>
                {

                    if let Some(back_waker) = val.queue.pop_back()
                    {

                        if let Some(shouldve_awoken) = val.active_ids.get_mut(&back_waker.id())
                        {

                            *shouldve_awoken = true;

                        }

                        waker = back_waker;

                    }
                    else
                    {

                        return Some(false);
                        
                    }

                }
                None =>
                {
                    
                    return None;

                }

            }

        }

        waker.wake();

        Some(true)

    }
    
    pub fn notify_waiters(&self) -> Option<usize>
    {

        #[cfg(feature="use_std_sync")]
        let mut mg = self.get_mg();

        #[cfg(any(feature="use_parking_lot_sync", feature="use_parking_lot_fair_sync"))]
        let mut mg = self.waker_queue_internals.lock();

        match &mut *mg
        {

            Some(val) =>
            {

                let res = Some(val.queue.len());

                while let Some(front_waker) = val.queue.pop_front()
                {

                    if let Some(shouldve_awoken) = val.active_ids.get_mut(&front_waker.id())
                    {

                        *shouldve_awoken = true;

                    }

                    front_waker.wake();

                }

                res

            }
            None =>
            {
                
                None

            }

        }

    }

    pub fn notify_waiters_buffered(&self, buffer: &mut VecDeque<QueuedWaker>) -> Option<usize>
    {

        buffer.clear();

        let res;

        {

            #[cfg(feature="use_std_sync")]
            let mut mg = self.get_mg();

            #[cfg(any(feature="use_parking_lot_sync", feature="use_parking_lot_fair_sync"))]
            let mut mg = self.waker_queue_internals.lock();

            match &mut *mg
            {

                Some(val) =>
                {

                    buffer.reserve(val.queue.len());

                    res = Some(val.queue.len());

                    while let Some(front_waker) = val.queue.pop_front()
                    {

                        if let Some(shouldve_awoken) = val.active_ids.get_mut(&front_waker.id())
                        {

                            *shouldve_awoken = true;

                        }

                        buffer.push_back(front_waker);

                    }

                }
                None =>
                {
                    
                    return None;

                }

            }

            while let Some(front_waker) = buffer.pop_front()
            {

                front_waker.wake();

            }

            res

        }

    }

    pub fn close(&self)
    {

        let opt_internals;

        {

            #[cfg(feature="use_std_sync")]
            let mut mg = self.get_mg();

            #[cfg(any(feature="use_parking_lot_sync", feature="use_parking_lot_fair_sync"))]
            let mut mg = self.waker_queue_internals.lock();

            opt_internals = mg.take();

        }

        if let Some(mut waker_queue_internals) = opt_internals
        {

            for item in waker_queue_internals.queue.drain(..)
            {

                item.wake();

            }

        }

    }

}

#[derive(Debug)]
pub struct WakerQueueWakeMeClosedError
{
}

impl WakerQueueWakeMeClosedError
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

impl Display for WakerQueueWakeMeClosedError
{

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        
        write!(f, "WakerQueue Closed")

    }

}

impl Error for WakerQueueWakeMeClosedError
{    
}

pub struct WakerQueueWakeMe<'a>
{

    waker_queue_ref: &'a WakerQueue,
    opt_waker_id: Option<usize>

}

impl<'a> WakerQueueWakeMe<'a>
{

    pub fn new(waker_queue_ref: &'a WakerQueue) -> Self
    {

        Self
        {

            waker_queue_ref,
            opt_waker_id: None

        }

    }

}

impl Future for WakerQueueWakeMe<'_>
{

    type Output = Result<(), WakerQueueWakeMeClosedError>;

    fn poll(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> Poll<Self::Output>
    {

        let mut_self = self.get_mut();

        #[cfg(feature="use_std_sync")]
        let mut mg = mut_self.waker_queue_ref.get_mg();

        #[cfg(any(feature="use_parking_lot_sync", feature="use_parking_lot_fair_sync"))]
        let mut mg = mut_self.waker_queue_ref.waker_queue_internals.lock();

        if let Some(id) =  &mut_self.opt_waker_id
        {

            /*
            #[cfg(feature="use_std_sync")]
            let mut mg = mut_self.waker_queue_ref.get_mg();

            #[cfg(any(feature="use_parking_lot_sync", feature="use_parking_lot_fair_sync"))]
            let mut mg = mut_self.waker_queue_ref.waker_queue_internals.lock();
            */

            match &mut *mg
            {

                Some(val) =>
                {

                    if let Some(shouldve_awoken) = val.active_ids.get(&id)
                    {

                        if *shouldve_awoken
                        {

                            val.active_ids.remove(id);

                            mut_self.opt_waker_id = None;

                            return Poll::Ready(Ok(()));

                        }
                        else
                        {

                            //push my waker back into the queue.

                            let queued_waker = QueuedWaker::new(cx.waker().clone(), *id);

                            val.queue.push_back(queued_waker);

                            return Poll::Pending;
                            
                        }

                    }

                }
                None =>
                {

                    return Poll::Ready(WakerQueueWakeMeClosedError::err());

                }

            }

            //The task is going to "sleep". Update the WQI so it can be woken up later.

            let mut inserted = false;

            let waker = cx.waker().clone();

            let mut id = 0;

            /*
            #[cfg(feature="use_std_sync")]
            let mut mg = mut_self.waker_queue_ref.get_mg();

            #[cfg(any(feature="use_parking_lot_sync", feature="use_parking_lot_fair_sync"))]
            let mut mg = self.waker_queue_ref.waker_queue_internals.lock();
            */

            match &mut *mg
            {

                Some(val) =>
                {

                    while !inserted
                    {

                        //Find the next avalible id.

                        id = val.latest_id.wpp();

                        inserted = val.active_ids.insert(id, false).is_none();
                        
                    }

                    let queued_waker = QueuedWaker::new(waker, id);

                    val.queue.push_back(queued_waker);

                    //let self_mut = mut_self.get_mut();

                    //Make sure this is set.

                    mut_self.opt_waker_id = Some(id);

                }
                None =>
                {

                    return Poll::Ready(WakerQueueWakeMeClosedError::err());

                }

            }                 

        }

        Poll::Pending

    }

}

impl Drop for WakerQueueWakeMe<'_>
{

    fn drop(&mut self)
    {

        // Make sure that the waker id gets removed.
        
        if let Some(id) = self.opt_waker_id
        {

            #[cfg(feature="use_std_sync")]
            let mut mg = self.waker_queue_ref.get_mg();

            #[cfg(any(feature="use_parking_lot_sync", feature="use_parking_lot_fair_sync"))]
            let mut mg = self.waker_queue_ref.waker_queue_internals.lock();

            if let Some(wqi) = &mut *mg
            {

                wqi.active_ids.remove(&id);

                let mut index = 0;

                let mut found = false; 

                for item in wqi.queue.iter()
                {

                    if item.id() == id
                    {

                        found = true;

                        break;

                    }

                    index.pp();

                }

                if found
                {

                    wqi.queue.remove(index);

                }

            }
            
        }

    }

}
