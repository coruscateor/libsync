use std::sync::Arc;

#[cfg(feature="use_std_sync")]
use std::sync::{ RwLockReadGuard, RwLockWriteGuard, TryLockError };

use accessorise::impl_get_ref;
#[cfg(any(feature="use_parking_lot_sync", feature="use_parking_lot_fair_sync"))]
use parking_lot::{ RwLockReadGuard, RwLockWriteGuard };

use crate::{ItemUpdater, PreferredRwLockType, shared_reading_and_writing::NotifyingSharedInternals};

use pastey::paste;

use delegate::delegate;

use super::Reader;

pub struct NotifyingSharedReader<T, I, U>
    where U: ItemUpdater<I>, 
          I: Clone + PartialEq + Unpin
{

    internals: Arc<NotifyingSharedInternals<T, I, U>>,
    current_item: I

}

impl<T, I, U> NotifyingSharedReader<T, I, U>
    where U: ItemUpdater<I>, 
          I: Clone + PartialEq + Unpin
{

    pub fn new(internals: Arc<NotifyingSharedInternals<T, I, U>>) -> Self //, current_item: I) -> Self
    {

        Self
        {

            internals,
            current_item: U::init()

        }

    }

    //impl_get_val_clone!(current_item, I);

    impl_get_ref!(current_item, I);

    #[cfg(feature="use_std_sync")]
    fn read_get_rg(&self) -> RwLockReadGuard<'_, T>
    {

        let lock_result = self.internals.rw_lock.read();

        match lock_result
        {

            Ok(mg) =>
            {

                mg

            }
            Err(err) =>
            {

                self.internals.rw_lock.clear_poison();

                err.into_inner()

            }

        }

    }

    async fn wake_me_with_item(&mut self)
    {

        match self.internals.notifier.wake_me_with_item(self.current_item.clone()).await
        {

            Ok(val) =>
            {

                self.current_item = val;

            }
            Err(_) => {}
        }

    }

    #[cfg(feature="use_std_sync")]
    pub async fn read(&mut self) -> Reader<'_, T>
    {

        self.wake_me_with_item().await;
        
        Reader::new(self.read_get_rg())

    }

    #[cfg(any(feature="use_parking_lot_sync", feature="use_parking_lot_fair_sync"))]
    pub async fn read(&self) -> Reader<'_, T>
    {

        self.wake_me_with_item().await;

        Reader::new(self.rw_lock.read())

    }

    pub async fn read_clone(&mut self) -> T
        where T: Clone
    {

        self.wake_me_with_item().await;

        (*self.read().await).clone()

    }

    #[cfg(feature="use_std_sync")]
    pub fn read_dont_wait(&self) -> Reader<'_, T>
    {
        
        Reader::new(self.read_get_rg())

    }

    #[cfg(any(feature="use_parking_lot_sync", feature="use_parking_lot_fair_sync"))]
    pub async fn read_dont_wait(&self) -> Reader<'_, T>
    {

        Reader::new(self.rw_lock.read())

    }

    pub fn read_clone_dont_wait(&self) -> T
        where T: Clone
    {

        (*self.read_dont_wait()).clone()

    }

    delegate!
    {

        to self.internals
        {

            pub fn is_closed(&self);

        }

    }

}
