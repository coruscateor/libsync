use std::collections::hash_map::Entry;
use std::error::Error;

use std::fmt::Display;

use std::future::Future;

use std::marker::PhantomData;

#[cfg(feature="use_std_sync")]
use std::sync::{Mutex, MutexGuard};

use std::sync::atomic::{AtomicUsize, Ordering};

use std::collections::{HashMap, HashSet, VecDeque};

use std::task::{Poll, Waker};

use pastey::paste;

use accessorise::impl_val_getter;

use inc_dec::{IncDecSelf, IntIncDecSelf};

use std::fmt::Debug;

use crate::{ItemUpdater, PreferredMutexType};

pub struct WakerQueueWithUpdatedItemInternals<T, U>
    where U: ItemUpdater<T>, 
          T: Clone + PartialEq + Unpin
{

    pub queue: VecDeque<Waker>,
    pub item: T,
    phantom_data: PhantomData<U>
}

impl<T, U> WakerQueueWithUpdatedItemInternals<T, U>
    where U: ItemUpdater<T>, 
          T: Clone + PartialEq + Unpin
{

    pub fn new() -> Self
    {

        Self
        {

            queue: VecDeque::new(),
            item: U::init(),
            phantom_data: PhantomData::default()

        }

    }

    pub fn with_capacity(capacity: usize) -> Self
    {

        Self
        {

            queue: VecDeque::with_capacity(capacity),
            item: U::init(),
            //item_updater
            phantom_data: PhantomData::default()

        }

    }

}

impl<T, U> Debug for WakerQueueWithUpdatedItem<T, U>
    where U: ItemUpdater<T> + Debug, 
          T: Clone + PartialEq + Unpin + Debug
{

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WakerQueueWithUpdatedItem").field("waker_queue_internals", &self.waker_queue_internals).finish()
    }
    
}

pub struct WakerQueueWithUpdatedItem<T, U>
    where U: ItemUpdater<T>, 
          T: Clone + PartialEq + Unpin
{

    waker_queue_internals: PreferredMutexType<Option<WakerQueueWithUpdatedItemInternals<T, U>>>

}

impl<T, U> WakerQueueWithUpdatedItem<T, U>
    where U: ItemUpdater<T>, 
          T: Clone + PartialEq + Unpin
{

    pub fn new() -> Self
    {

        Self
        {

            waker_queue_internals: PreferredMutexType::new(Some(WakerQueueWithUpdatedItemInternals::new()))

        }

    }

    pub fn with_capacity(size: usize) -> Self
    {

        Self
        {

            waker_queue_internals: PreferredMutexType::new(Some(WakerQueueWithUpdatedItemInternals::with_capacity(size)))

        }

    }

    #[cfg(feature="use_std_sync")]
    fn get_mg(&self) -> MutexGuard<'_, Option<WakerQueueWithUpdatedItemInternals<T, U>>>
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

    pub fn is_closed(&self) -> bool
    {

        #[cfg(feature="use_std_sync")]
        let mg = self.get_mg();

        #[cfg(any(feature="use_parking_lot_sync", feature="use_parking_lot_fair_sync"))]
        let mg = self.waker_queue_internals.lock();

        mg.is_none()

    }

    pub fn get_item(&self) -> Option<T>
        where T: Clone
    {

        #[cfg(feature="use_std_sync")]
        let mg = self.get_mg();

        #[cfg(any(feature="use_parking_lot_sync", feature="use_parking_lot_fair_sync"))]
        let mg = self.waker_queue_internals.lock();

        if let Some(val) = &*mg
        {

            return Some(val.item.clone());

        } 

        None

    }

    pub fn wake_me_with_item<'a>(&'a self, current_item: T) -> WakerQueueWakeMeWithItem<'a, T, U>
    {

        WakerQueueWakeMeWithItem::new(self, current_item)

    }

    pub fn wake_me_ignore_item<'a>(&'a self) -> WakerQueueWakeMeIgnoreItem<'a, T, U>
    {

        WakerQueueWakeMeIgnoreItem::new(self)

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

                        U::update(&mut val.item);

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

    pub fn notify_one_no_update(&self) -> Option<bool>
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

                        U::update(&mut val.item);

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

    pub fn notify_last_no_update(&self) -> Option<bool>
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

                U::update(&mut val.item);

                while let Some(front_waker) = val.queue.pop_front()
                {

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

    pub fn notify_waiters_no_update(&self) -> Option<usize>
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

    pub fn notify_waiters_buffered(&self, buffer: &mut VecDeque<Waker>) -> Option<usize>
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

                    U::update(&mut val.item);

                    while let Some(front_waker) = val.queue.pop_front()
                    {

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


    pub fn notify_waiters_buffered_no_update(&self, buffer: &mut VecDeque<Waker>) -> Option<usize>
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

    pub fn update_dont_notify(&self) -> bool
    {

        #[cfg(feature="use_std_sync")]
        let mut mg = self.get_mg();

        #[cfg(any(feature="use_parking_lot_sync", feature="use_parking_lot_fair_sync"))]
        let mut mg = self.waker_queue_internals.lock();

        match &mut *mg
        {

            Some(val) =>
            {

                U::update(&mut val.item);

                true

            }
            None =>
            {

                false

            }

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

impl<T, U> Debug for WakerQueueWithUpdatedItemInternals<T, U>
    where U: ItemUpdater<T> + Debug, 
          T: Clone + PartialEq + Unpin + Debug
{

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WakerQueueWithUpdatedItemInternals").field("queue", &self.queue).field("item", &self.item).field("phantom_data", &self.phantom_data).finish()
    }

}

#[derive(Debug)]
pub struct WakerQueueWakeMeWithItemClosedError
{
}

impl WakerQueueWakeMeWithItemClosedError
{

    pub fn new() -> Self
    {

        Self
        {}

    }

    pub fn err<T>() -> Result<T, Self>
    {

        Err(Self::new())

    }

}

impl Display for WakerQueueWakeMeWithItemClosedError
{

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        
        write!(f, "WakerQueueWithUpdatedItem Closed")

    }

}

impl Error for WakerQueueWakeMeWithItemClosedError
{    
}

pub struct WakerQueueWakeMeWithItem<'a, T, U>
    where U: ItemUpdater<T>, 
          T: Clone + PartialEq + Unpin
{

    waker_queue_ref: &'a WakerQueueWithUpdatedItem<T, U>,
    current_item: T

}

impl<'a, T, U> WakerQueueWakeMeWithItem<'a, T, U>
    where U: ItemUpdater<T>, 
          T: Clone + PartialEq + Unpin
{

    pub fn new(waker_queue_ref: &'a WakerQueueWithUpdatedItem<T, U>, current_item: T) -> Self
    {

        Self
        {

            waker_queue_ref,
            current_item

        }

    }

}

impl<T, U> Future for WakerQueueWakeMeWithItem<'_, T, U>
    where U: ItemUpdater<T>, 
          T: Clone + PartialEq + Unpin
{

    type Output = Result<T, WakerQueueWakeMeWithItemClosedError>;

    fn poll(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> Poll<Self::Output>
    {

        let mut_self = self.get_mut();

        #[cfg(feature="use_std_sync")]
        let mut mg = mut_self.waker_queue_ref.get_mg();

        #[cfg(any(feature="use_parking_lot_sync", feature="use_parking_lot_fair_sync"))]
        let mut mg = mut_self.waker_queue_ref.waker_queue_internals.lock();

        match &mut *mg
        {

            Some(val) =>
            {

                if val.item == mut_self.current_item
                {

                    //The items are the same so we skip waiting.

                    return Poll::Ready(Ok(val.item.clone()));

                }

                //The task is going to "sleep". Update the WQI so it can be woken up later.

                let waker = cx.waker().clone();

                val.queue.push_back(waker);

            }
            None =>
            {

                return Poll::Ready(WakerQueueWakeMeWithItemClosedError::err());

            }

        }

        Poll::Pending

    }

}

impl<T, U> Debug for WakerQueueWakeMeWithItem<'_, T, U>
    where U: ItemUpdater<T> + Debug, 
          T: Clone + PartialEq + Unpin + Debug
{

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WakerQueueWakeMeWithItem").field("waker_queue_ref", &self.waker_queue_ref).field("current_item", &self.current_item).finish()
    }
    
}

// Ignore the item

#[derive(Debug)]
pub struct WakerQueueWakeMeIgnoreItemClosedError
{
}

impl WakerQueueWakeMeIgnoreItemClosedError
{

    pub fn new() -> Self
    {

        Self
        {}

    }

    pub fn err<T>() -> Result<T, Self>
    {

        Err(Self::new())

    }

}

impl Display for WakerQueueWakeMeIgnoreItemClosedError
{

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        
        write!(f, "WakerQueueWithUpdatedItem Closed")

    }

}

impl Error for WakerQueueWakeMeIgnoreItemClosedError
{    
}

pub struct WakerQueueWakeMeIgnoreItem<'a, T, U>
    where U: ItemUpdater<T>, 
          T: Clone + PartialEq + Unpin
{

    waker_queue_ref: &'a WakerQueueWithUpdatedItem<T, U>

}

impl<'a, T, U> WakerQueueWakeMeIgnoreItem<'a, T, U>
    where U: ItemUpdater<T>, 
          T: Clone + PartialEq + Unpin
{

    //Ignore current_item

    pub fn new(waker_queue_ref: &'a WakerQueueWithUpdatedItem<T, U>) -> Self
    {

        Self
        {

            waker_queue_ref

        }

    }

}

impl<T, U> Future for WakerQueueWakeMeIgnoreItem<'_, T, U>
    where U: ItemUpdater<T>, 
          T: Clone + PartialEq + Unpin
{

    type Output = Result<(), WakerQueueWakeMeIgnoreItemClosedError>;

    fn poll(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> Poll<Self::Output>
    {

        let mut_self = self.get_mut();

        #[cfg(feature="use_std_sync")]
        let mut mg = mut_self.waker_queue_ref.get_mg();

        #[cfg(any(feature="use_parking_lot_sync", feature="use_parking_lot_fair_sync"))]
        let mut mg = mut_self.waker_queue_ref.waker_queue_internals.lock();

        match &mut *mg
        {

            Some(val) =>
            {

                //The task is going to "sleep". Update the WQI so it can be woken up later.

                let waker = cx.waker().clone();

                val.queue.push_back(waker);

            }
            None =>
            {

                return Poll::Ready(WakerQueueWakeMeIgnoreItemClosedError::err());

            }

        }

        Poll::Pending

    }

}
