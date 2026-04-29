use std::sync::Arc;

#[cfg(feature="use_std_sync")]
use std::sync::{ RwLockReadGuard, RwLockWriteGuard, TryLockError };

#[cfg(any(feature="use_parking_lot_sync", feature="use_parking_lot_fair_sync"))]
use parking_lot::{ RwLockReadGuard, RwLockWriteGuard };

use crate::PreferredRwLockType;

use super::SharedReader;

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
    fn read_get_rg(&self) -> RwLockReadGuard<'_, T>
    {

        let lock_result = self.rw_lock.read();

        match lock_result
        {

            Ok(mg) =>
            {

                mg

            }
            Err(err) =>
            {

                self.rw_lock.clear_poison();

                err.into_inner()

            }

        }

    }

    #[cfg(feature="use_std_sync")]
    pub fn read(&self) -> RwLockReadGuard<'_, T>
    {
        
        self.read_get_rg()

    }

    #[cfg(any(feature="use_parking_lot_sync", feature="use_parking_lot_fair_sync"))]
    pub fn read(&self) -> RwLockReadGuard<'_, T>
    {

        self.rw_lock.read()

    }

    pub fn read_clone(&self) -> T
        where T: Clone
    {

        (*self.read()).clone()

    }

    #[cfg(feature="use_std_sync")]
    fn write_get_wg(&self) -> RwLockWriteGuard<'_, T>
    {

        let lock_result = self.rw_lock.write();

        match lock_result
        {

            Ok(wg) =>
            {

                wg

            }
            Err(err) =>
            {

                self.rw_lock.clear_poison();

                err.into_inner()

            }

        }

    }
    
    #[cfg(feature="use_std_sync")]
    pub fn write(&self) -> RwLockWriteGuard<'_, T>
    {

        self.write_get_wg()

    }

    #[cfg(any(feature="use_parking_lot_sync", feature="use_parking_lot_fair_sync"))]
    pub fn write(&self) -> RwLockWriteGuard<'_, T>
    {

        self.rw_lock.write()

    }

    #[cfg(feature="use_std_sync")]
    fn try_read_get_rg(&self) -> Option<RwLockReadGuard<'_, T>>
    {

        let lock_result = self.rw_lock.try_read();

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

                        self.rw_lock.clear_poison();

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
    pub fn try_read(&self) -> Option<RwLockReadGuard<'_, T>>
    {

        self.try_read_get_rg()

    }

    #[cfg(any(feature="use_parking_lot_sync", feature="use_parking_lot_fair_sync"))]
    pub fn try_read(&self) -> Option<RwLockReadGuard<'_, T>>
    {

        self.rw_lock.try_read()

    }

    #[cfg(feature="use_std_sync")]
    fn try_write_get_wg(&self) -> Option<RwLockWriteGuard<'_, T>>
    {

        let lock_result = self.rw_lock.try_write();

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

                        self.rw_lock.clear_poison();

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
    pub fn try_write(&self) -> Option<RwLockWriteGuard<'_, T>>
    {

        self.try_write_get_wg()

    }

    #[cfg(any(feature="use_parking_lot_sync", feature="use_parking_lot_fair_sync"))]
    pub fn try_write(&self) -> Option<RwLockWriteGuard<'_, T>>
    {

        self.rw_lock.try_write()

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

        SharedReader::from_rw_lock(self.rw_lock.clone())

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