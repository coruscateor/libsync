use std::{mem::{replace, swap, take}, sync::Arc};

use crate::PreferredRwLockType;

use super::{SharedReader, Reader, Writer, WeakSharedWriter};

#[cfg(feature="use_std_sync")]
use crate::sync::{read_get_rg, write_get_wg, try_read_get_rg, try_write_get_wg};

pub struct SharedWriter<T>
{

    rw_lock: Arc<PreferredRwLockType<T>>

}

impl<T> SharedWriter<T>
{

    pub fn new(object: T) -> Self
    {

        Self
        {

            rw_lock: Arc::new(PreferredRwLockType::new(object))

        }

    }

    pub fn from_rw_lock(rw_lock: Arc<PreferredRwLockType<T>>) -> Self
    {

        Self
        {

            rw_lock

        }

    }

    #[cfg(feature="use_std_sync")]
    pub fn read(&self) -> Reader<'_, T>
    {
        
        Reader::new(read_get_rg(&self.rw_lock))

    }

    #[cfg(any(feature="use_parking_lot_sync", feature="use_parking_lot_fair_sync"))]
    pub fn read(&self) -> Reader<'_, T>
    {

        Reader::new(self.rw_lock.read())

    }

    pub fn read_clone(&self) -> T
        where T: Clone
    {

        (*self.read()).clone()

    }
    
    #[cfg(feature="use_std_sync")]
    pub fn write(&self) -> Writer<'_, T>
    {

        Writer::new(write_get_wg(&self.rw_lock))

    }

    #[cfg(any(feature="use_parking_lot_sync", feature="use_parking_lot_fair_sync"))]
    pub fn write(&self) -> Writer<'_, T>
    {

        Writer::new(self.rw_lock.write())

    }

    pub fn write_clone(&self, item: &T)
        where T: Clone
    {

        (*self.write()) = (*item).clone();

    }

    pub fn write_move(&self, item: T)
    {

        (*self.write()) = item;

    }

    pub fn write_replace(&self, item: T) -> T
        where T: Default
    {

        replace(&mut *self.write(), item)

    }

    pub fn write_swap(&self, item: &mut T)
        where T: Default
    {

        swap(&mut *self.write(), item);

    }

    pub fn write_take(&self, item: &mut T)
        where T: Default
    {

        (*self.write()) = take(item);

    }

    pub fn write_fn<F>(&self, item: &T, write_fn: &F)
        where F: Fn(&mut T, &T)
    {

        write_fn(&mut *self.write(), item);

    }

    pub fn write_fn_mut<F>(&self, item: &mut T, write_fn_mut: &mut F)
        where F: FnMut(&mut T, &mut T)
    {

        write_fn_mut(&mut *self.write(), item);

    }

    pub fn write_fn_once<F>(&self, item: &mut T, write_fn_once: F)
        where F: FnOnce(&mut T, &mut T)
    {

        write_fn_once(&mut *self.write(), item);

    }

    #[cfg(feature="use_std_sync")]
    pub fn try_read(&self) -> Option<Reader<'_, T>>
    {

        if let Some(read_guard) = try_read_get_rg(&self.rw_lock)
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
    pub fn try_write(&self) -> Option<Writer<'_, T>>
    {

        if let Some(write_guard) = try_write_get_wg(&self.rw_lock)
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

        if let Some(rw_lock) = Arc::into_inner(self.rw_lock)
        {

            #[cfg(feature="use_std_sync")]
            match rw_lock.into_inner()
            {

                Ok(val) =>
                {

                    return Some(val);

                }
                Err(err) =>
                {

                    Some(err.into_inner())

                }

            }

            #[cfg(any(feature="use_parking_lot_sync", feature="use_parking_lot_fair_sync"))]
            Some(rw_lock.into_inner())

        }
        else
        {
            
            None

        }

    }

    pub fn strong_count(&self) -> usize
    {

        Arc::strong_count(&self.rw_lock)

    }

    pub fn weak_count(&self) -> usize
    {

        Arc::weak_count(&self.rw_lock)
        
    }

    pub fn get_shared_reader(&self) -> SharedReader<T>
    {

        SharedReader::new(self.rw_lock.clone())

    }

    pub fn downgrade(&self) -> WeakSharedWriter<T>
    {

        WeakSharedWriter::new(&self.rw_lock)

    }

}

impl<T> Clone for SharedWriter<T>
{

    fn clone(&self) -> Self
    {

        Self
        {
            
            rw_lock: self.rw_lock.clone()
        
        }

    }

}