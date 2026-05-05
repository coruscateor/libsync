use std::{mem::{replace, swap, take}, sync::Arc};

#[cfg(feature="use_std_sync")]
use std::sync::{ RwLockReadGuard, RwLockWriteGuard, TryLockError };

#[cfg(any(feature="use_parking_lot_sync", feature="use_parking_lot_fair_sync"))]
use parking_lot::{ RwLockReadGuard, RwLockWriteGuard };

use delegate::delegate;

use crate::{ItemUpdater, PreferredRwLockType, shared_reading_and_writing::{NotifyingSharedInternals, NotifyingSharedReader}};

use super::{SharedReader, Reader, Writer, NotifyingWriter};


pub struct NotifyingSharedWriter<T, I, U>
    where U: ItemUpdater<I>, 
          I: Clone + PartialEq + Unpin
{

    internals: Arc<NotifyingSharedInternals<T, I, U>>

}

impl<T, I, U> NotifyingSharedWriter<T, I, U>
    where U: ItemUpdater<I>, 
          I: Clone + PartialEq + Unpin
{

    pub fn new(object: T) -> Self
    {

        Self
        {

            internals: Arc::new(NotifyingSharedInternals::new(PreferredRwLockType::new(object)))

        }

    }

    pub fn from_internals(internals: Arc<NotifyingSharedInternals<T, I, U>>) -> Self
    {

        Self
        {

            internals

        }

    }

    pub fn current_item(&self) -> Option<I>
    {

        self.internals.notifier.get_item()

    }

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

    #[cfg(feature="use_std_sync")]
    pub async fn read(&self) -> Reader<'_, T>
    {

        let _ = self.internals.notifier.wake_me_ignore_item().await;
        
        Reader::new(self.read_get_rg())

    }

    #[cfg(any(feature="use_parking_lot_sync", feature="use_parking_lot_fair_sync"))]
    pub async fn read(&self) -> Reader<'_, T>
    {

        let _ = self.internals.notifier.wake_me_ignore_item().await;

        Reader::new(self.rw_lock.read())

    }

    pub async fn read_clone(&self) -> T
        where T: Clone
    {

        (*self.read().await).clone()

    }

    //dont_wait

    #[cfg(feature="use_std_sync")]
    pub fn read_dont_wait(&self) -> Reader<'_, T>
    {
        
        Reader::new(self.read_get_rg())

    }

    #[cfg(any(feature="use_parking_lot_sync", feature="use_parking_lot_fair_sync"))]
    pub fn read_dont_wait(&self) -> Reader<'_, T>
    {

        Reader::new(self.rw_lock.read())

    }

    pub fn read_clone_dont_wait(&self) -> T
        where T: Clone
    {

        (*self.read_dont_wait()).clone()

    }

    //Writing

    #[cfg(feature="use_std_sync")]
    fn write_get_wg(&self) -> RwLockWriteGuard<'_, T>
    {

        let lock_result = self.internals.rw_lock.write();

        match lock_result
        {

            Ok(wg) =>
            {

                wg

            }
            Err(err) =>
            {

                self.internals.rw_lock.clear_poison();

                err.into_inner()

            }

        }

    }
    
    #[cfg(feature="use_std_sync")]
    pub fn write(&self) -> NotifyingWriter<'_, T, I, U>
    {

        NotifyingWriter::new(self.write_get_wg(), &self.internals.notifier)

    }

    #[cfg(any(feature="use_parking_lot_sync", feature="use_parking_lot_fair_sync"))]
    pub fn write(&self) -> NotifyingWriter<'_, T, I, U>
    {

        NotifyingWriter::new(self.rw_lock.write())

    }

    pub fn write_clone(&self, item: &T)
        where T: Clone
    {

        (*self.write()) = (*item).clone();

        self.internals.notifier.notify_waiters();

    }

    pub fn write_move(&self, item: T)
    {

        (*self.write()) = item;

        self.internals.notifier.notify_waiters();

    }

    pub fn write_replace(&self, item: T) -> T
        where T: Default
    {

        let res = replace(&mut *self.write(), item);

        self.internals.notifier.notify_waiters();

        res

    }

    pub fn write_swap(&self, item: &mut T)
        where T: Default
    {

        swap(&mut *self.write(), item);

        self.internals.notifier.notify_waiters();

    }

    pub fn write_take(&self, item: &mut T)
        where T: Default
    {

        (*self.write()) = take(item);

        self.internals.notifier.notify_waiters();

    }

    pub fn write_fn<F>(&self, item: &T, write_fn: &F)
        where F: Fn(&mut T, &T)
    {

        write_fn(&mut *self.write(), item);

        self.internals.notifier.notify_waiters();

    }

    pub fn write_fn_mut<F>(&self, item: &mut T, write_fn_mut: &mut F)
        where F: FnMut(&mut T, &mut T)
    {

        write_fn_mut(&mut *self.write(), item);

        self.internals.notifier.notify_waiters();

    }

    pub fn write_fn_once<F>(&self, item: &mut T, write_fn_once: F)
        where F: FnOnce(&mut T, &mut T)
    {

        write_fn_once(&mut *self.write(), item);

        self.internals.notifier.notify_waiters();

    }

    pub fn notify_waiters(&self)
    {

        self.internals.notifier.notify_waiters();

    }

    //Write don't notify

    #[cfg(feature="use_std_sync")]
    pub fn write_dont_notify(&self) -> Writer<'_, T>
    {

        Writer::new(self.write_get_wg())

    }

    #[cfg(any(feature="use_parking_lot_sync", feature="use_parking_lot_fair_sync"))]
    pub fn write_dont_notify(&self) -> Writer<'_, T>
    {

        Writer::new(self.rw_lock.write())

    }

    pub fn write_clone_dont_notify(&self, item: &T)
        where T: Clone
    {

        (*self.write()) = (*item).clone();

    }

    pub fn write_move_dont_notify(&self, item: T)
    {

        (*self.write()) = item;

    }

    pub fn write_replace_dont_notify(&self, item: T) -> T
        where T: Default
    {

        replace(&mut *self.write(), item)

    }

    pub fn write_swap_dont_notify(&self, item: &mut T)
        where T: Default
    {

        swap(&mut *self.write(), item);

    }

    pub fn write_take_dont_notify(&self, item: &mut T)
        where T: Default
    {

        (*self.write()) = take(item);

    }

    pub fn write_fn_dont_notify<F>(&self, item: &T, write_fn: &F)
        where F: Fn(&mut T, &T)
    {

        write_fn(&mut *self.write(), item);

    }

    pub fn write_fn_mut_dont_notify<F>(&self, item: &mut T, write_fn_mut: &mut F)
        where F: FnMut(&mut T, &mut T)
    {

        write_fn_mut(&mut *self.write(), item);

    }

    pub fn write_fn_once_dont_notify<F>(&self, item: &mut T, write_fn_once: F)
        where F: FnOnce(&mut T, &mut T)
    {

        write_fn_once(&mut *self.write(), item);

    }

    //

    #[cfg(feature="use_std_sync")]
    fn try_read_get_rg(&self) -> Option<RwLockReadGuard<'_, T>>
    {

        let lock_result = self.internals.rw_lock.try_read();

        match lock_result
        {

            Ok(rg) =>
            {

               Some(rg)

            }
            Err(err) =>
            {

                match err
                {

                    TryLockError::Poisoned(poison_error) =>
                    {

                        self.internals.rw_lock.clear_poison();

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

    #[cfg(feature="use_std_sync")]
    pub fn try_read(&self) -> Option<Reader<'_, T>>
    {

        if let Some(read_guard) = self.try_read_get_rg()
        {

            Some(Reader::new(read_guard))

        }
        else
        {

            None
            
        }

    }

    #[cfg(any(feature="use_parking_lot_sync", feature="use_parking_lot_fair_sync"))]
    pub fn try_read(&self) -> Option<Reader<'_, T>>
    {

        if let Some(read_guard) = self.rw_lock.try_read()
        {

            Some(Reader::new(read_guard))

        }
        else
        {

            None
            
        }

    }

    #[cfg(feature="use_std_sync")]
    fn try_write_get_wg(&self) -> Option<RwLockWriteGuard<'_, T>>
    {

        let lock_result = self.internals.rw_lock.try_write();

        match lock_result
        {

            Ok(wg) =>
            {

               Some(wg)

            }
            Err(err) =>
            {

                match err
                {

                    TryLockError::Poisoned(poison_error) =>
                    {

                        self.internals.rw_lock.clear_poison();

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

    #[cfg(feature="use_std_sync")]
    pub fn try_write(&self) -> Option<Writer<'_, T>>
    {

        if let Some(write_guard) = self.try_write_get_wg()
        {

            Some(Writer::new(write_guard))

        }
        else
        {
            
            None

        }

    }

    #[cfg(any(feature="use_parking_lot_sync", feature="use_parking_lot_fair_sync"))]
    pub fn try_write(&self) -> Option<Writer<'_, T>>
    {

        if let Some(write_guard) = self.rw_lock.try_write()
        {

            Some(Writer::new(write_guard))

        }
        else
        {
            
            None

        }

    }

    pub fn into_inner(self) -> Option<T>
    {

        if let Some(val) = Arc::into_inner(self.internals)
        {

            val.into_inner()

        }
        else
        {

            None
            
        }

    }

    pub fn strong_count(&self) -> usize
    {

        Arc::strong_count(&self.internals)

    }

    pub fn weak_count(&self) -> usize
    {

        Arc::weak_count(&self.internals)
        
    }

    pub fn get_notifying_shared_reader(&self) -> NotifyingSharedReader<T, I, U>
    {

        NotifyingSharedReader::new(self.internals.clone()) //, self.internals.)

    }

    delegate!
    {

        to self.internals
        {

            pub fn is_closed(&self);

        }

    }

}

impl<T, I, U> Clone for NotifyingSharedWriter<T, I, U>
    where U: ItemUpdater<I>, 
          I: Clone + PartialEq + Unpin
{

    fn clone(&self) -> Self
    {

        Self
        {
            
            internals: self.internals.clone()
        
        }

    }

}