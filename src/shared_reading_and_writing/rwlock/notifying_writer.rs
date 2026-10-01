use std::{fmt::{Display, write}, ops::{Deref, DerefMut}, sync::Arc};

#[cfg(feature="use_std_sync")]
use std::sync::{ RwLockReadGuard, RwLockWriteGuard, TryLockError };

#[cfg(any(feature="use_parking_lot_sync", feature="use_parking_lot_fair_sync"))]
use parking_lot::{ RwLockReadGuard, RwLockWriteGuard };

use std::fmt::Debug;

use crate::{ItemUpdater, WakerQueueWithUpdatedItem};

pub struct NotifyingWriter<'a, T, I, U>
    where U: ItemUpdater<I>, 
          I: Clone + PartialEq + Unpin
{

    write_guard: RwLockWriteGuard<'a, T>,
    notifier_ref: &'a WakerQueueWithUpdatedItem<I, U>

}

impl<'a, T, I, U> NotifyingWriter<'a, T, I, U>
    where U: ItemUpdater<I>, 
          I: Clone + PartialEq + Unpin
{

    pub fn new(write_guard: RwLockWriteGuard<'a, T>, notifier_ref: &'a WakerQueueWithUpdatedItem<I, U>) -> Self
    {

        Self
        {

            write_guard,
            notifier_ref

        }

    }

}

impl<'a, T, I, U> Deref for NotifyingWriter<'a, T, I, U>
    where U: ItemUpdater<I>, 
          I: Clone + PartialEq + Unpin
{

    type Target = T;

    fn deref(&self) -> &Self::Target
    {

        &*self.write_guard
        
    }

}

impl<'a, T, I, U> DerefMut for NotifyingWriter<'a, T, I, U>
    where U: ItemUpdater<I>, 
          I: Clone + PartialEq + Unpin
{

    fn deref_mut(&mut self) -> &mut Self::Target
    {
        
        &mut *self.write_guard
        
    }

}

impl<'a, T, I, U> Debug for NotifyingWriter<'a, T, I, U>
    where U: ItemUpdater<I> + Debug, 
          I: Clone + PartialEq + Unpin + Debug,
          T: Debug
{

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("NotifyingWriter").field("write_guard", &self.write_guard).field("notifier_ref", &self.notifier_ref).finish()
    }

}


impl<'a, T, I, U> Display for NotifyingWriter<'a, T, I, U>
    where U: ItemUpdater<I>, 
          I: Clone + PartialEq + Unpin,
          T: Display
{

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {

        write!(f, "{}", &self.write_guard)

    }

}

impl<'a, T, I, U> Drop for NotifyingWriter<'a, T, I, U>
    where U: ItemUpdater<I>, 
          I: Clone + PartialEq + Unpin
{

    fn drop(&mut self)
    {
        
        self.notifier_ref.notify_waiters();

    }

}
