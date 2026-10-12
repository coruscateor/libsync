use std::error::Error;

use std::fmt::Display;

use std::future::Future;

use std::marker::PhantomData;

use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

use std::collections::{HashMap, HashSet, VecDeque};

use std::task::{Poll, Waker};

use pastey::paste;

use accessorise::impl_val_getter;

use inc_dec::{IncDecSelf, IntIncDecSelf};

use std::fmt::Debug;

#[cfg(feature="use_std_sync")]
use crate::get_mg;
use crate::{ItemUpdater, PreferredMutexType};

use crossbeam_queue::SegQueue;

use delegate::delegate;

use crate::spin_a_bit;

use crate::atomic::AtomicValueOrMutex;

//use crossbeam::atomic::AtomicCell;

pub struct WakerQueueWithUpdatedItemInternals<T, U>
    where U: ItemUpdater<T>, 
          T: Clone + PartialEq + Unpin + 'static
{

    pub queue: SegQueue<Waker>,
    pub item: AtomicValueOrMutex<T>, //AtomicCell<T>,
    pub is_closed: AtomicBool,
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

            queue: SegQueue::new(),
            item: AtomicValueOrMutex::new(U::init()),
            is_closed: AtomicBool::new(false),
            phantom_data: PhantomData::default()

        }

    }

    pub fn is_closed(&self) -> bool
    {

        self.is_closed.load(Ordering::Acquire)

    }

    pub fn close(&self)
    {

        self.is_closed.store(true,Ordering::Release)

    }

}

impl<T, U> Debug for WakerQueueWithUpdatedItem<T, U>
    where U: ItemUpdater<T> + Debug, 
          T: Clone + PartialEq + Unpin + Debug + 'static
{

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WakerQueueWithUpdatedItem").field("waker_queue_internals", &self.waker_queue_internals).finish()
    }

}

pub struct WakerQueueWithUpdatedItem<T, U>
    where U: ItemUpdater<T>, 
          T: Clone + PartialEq + Unpin + 'static
{

    waker_queue_internals: WakerQueueWithUpdatedItemInternals<T, U>

}

impl<T, U> WakerQueueWithUpdatedItem<T, U>
    where U: ItemUpdater<T>, 
          T: Clone + PartialEq + Unpin + 'static
{

    pub fn new() -> Self
    {

        Self
        {

            waker_queue_internals: WakerQueueWithUpdatedItemInternals::new()

        }

    }

    delegate!
    {

        to self.waker_queue_internals
        {

            pub fn is_closed(&self) -> bool;

        }

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

        if let Some(waker) = self.waker_queue_internals.queue.pop()
        {

            let mut fun = |val_mut: &mut T|
            {

                U::update( val_mut);

            };

            self.waker_queue_internals.item.update(&mut fun);

            waker.wake();

            Some(true)

        }
        else
        {

            if self.is_closed()
            {

                None

            }
            else
            {

                Some(false)
                
            }
            
        }

    }

    pub fn notify_one_no_update(&self) -> Option<bool>
    {

        if let Some(waker) = self.waker_queue_internals.queue.pop()
        {

            waker.wake();

            Some(true)

        }
        else
        {

            if self.is_closed()
            {

                None

            }
            else
            {

                Some(false)
                
            }
            
        }

    }
    
    pub fn notify_waiters(&self) -> Option<usize>
    {

        if self.is_closed()
        {

            return None;

        }

        let mut fun=  |to_update: &mut T|
        {

            U::update(to_update)

        };

        self.waker_queue_internals.item.update(&mut fun);

        let res = Some(self.waker_queue_internals.queue.len());

        while let Some(waker) = self.waker_queue_internals.queue.pop()
        {

            waker.wake();

        }

        res

    }

    pub fn notify_waiters_no_update(&self) -> Option<usize>
    {

        if self.is_closed()
        {

            return None;

        }

        let res = Some(self.waker_queue_internals.queue.len());

        while let Some(waker) = self.waker_queue_internals.queue.pop()
        {

            waker.wake();

        }

        res

    }

    pub fn update_dont_notify(&self) -> bool
    {

        if self.is_closed()
        {

            return false;

        }

        let mut fun=  |to_update: &mut T|
        {

            U::update(to_update)

        };

        self.waker_queue_internals.item.update(&mut fun);

        true

    }

    pub fn close(&self)
    {

        self.waker_queue_internals.close();

        spin_a_bit();

        while let Some(waker) = self.waker_queue_internals.queue.pop()
        {

            waker.wake();

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
        {
        }

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
          T: Clone + PartialEq + Unpin + 'static
{

    waker_queue_ref: &'a WakerQueueWithUpdatedItem<T, U>,
    current_item: T

}

impl<'a, T, U> WakerQueueWakeMeWithItem<'a, T, U>
    where U: ItemUpdater<T>, 
          T: Clone + PartialEq + Unpin + 'static
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
          T: Clone + PartialEq + Unpin + 'static
{

    type Output = Result<T, WakerQueueWakeMeWithItemClosedError>;

    fn poll(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> Poll<Self::Output>
    {

        let waker_queue_internals = &self.waker_queue_ref.waker_queue_internals;

        let value = waker_queue_internals.item.clone_value();

        if value == self.current_item
        {

            //The items are the same so we skip waiting.

            return Poll::Ready(Ok(value));

        }

        //The task is going to "sleep". Update the WQI so it can be woken up later.

        let waker = cx.waker().clone();

        if waker_queue_internals.is_closed()
        {

            return Poll::Ready(WakerQueueWakeMeWithItemClosedError::err());

        }

        waker_queue_internals.queue.push(waker);

        Poll::Pending

    }

}

impl<T, U> Debug for WakerQueueWakeMeWithItem<'_, T, U>
    where U: ItemUpdater<T> + Debug, 
          T: Clone + PartialEq + Unpin + Debug + 'static
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
        {
        }

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
          T: Clone + PartialEq + Unpin + 'static
{

    waker_queue_ref: &'a WakerQueueWithUpdatedItem<T, U>

}

impl<'a, T, U> WakerQueueWakeMeIgnoreItem<'a, T, U>
    where U: ItemUpdater<T>, 
          T: Clone + PartialEq + Unpin + 'static
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
          T: Clone + PartialEq + Unpin + 'static
{

    type Output = Result<(), WakerQueueWakeMeIgnoreItemClosedError>;

    fn poll(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> Poll<Self::Output>
    {

        let waker_queue_internals = &self.waker_queue_ref.waker_queue_internals;

        //The task is going to "sleep". Update the WQI so it can be woken up later.

        let waker = cx.waker().clone();

        if waker_queue_internals.is_closed()
        {

            return Poll::Ready(WakerQueueWakeMeIgnoreItemClosedError::err());

        }

        waker_queue_internals.queue.push(waker);

        Poll::Pending

    }

}
