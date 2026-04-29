use std::sync::Arc;

#[cfg(feature="use_std_sync")]
use std::sync::{ RwLockReadGuard, RwLockWriteGuard, TryLockError };

#[cfg(any(feature="use_parking_lot_sync", feature="use_parking_lot_fair_sync"))]
use parking_lot::{ RwLockReadGuard, RwLockWriteGuard };

use crate::PreferredRwLockType;

pub struct SharedReader<T>
{

    rw_lock: Arc<PreferredRwLockType<T>>

}

impl<T> SharedReader<T>
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


}
