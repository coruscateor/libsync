use std::sync::Arc;

use crate::PreferredRwLockType;

use super::{Reader, WeakSharedReader};

#[cfg(not(feature="use_faux_rwlock"))]
#[cfg(feature="use_std_sync")]
use crate::sync::read_get_rg;

pub struct SharedReader<T>
{

    rw_lock: Arc<PreferredRwLockType<T>>

}

impl<T> SharedReader<T>
{

    pub fn new(rw_lock: Arc<PreferredRwLockType<T>>) -> Self
    {

        Self
        {

            rw_lock

        }

    }

    #[cfg(not(feature="use_faux_rwlock"))]
    #[cfg(feature="use_std_sync")]
    pub fn read(&self) -> Reader<'_, T>
    {
        
        Reader::new(read_get_rg(&self.rw_lock))

    }

    #[cfg(any(feature="use_parking_lot_sync", feature="use_parking_lot_fair_sync", feature="use_faux_rwlock"))]
    pub fn read(&self) -> Reader<'_, T>
    {

        Reader::new(self.rw_lock.read())

    }

    pub fn read_clone(&self) -> T
        where T: Clone
    {

        (*self.read()).clone()

    }

    pub fn strong_count(&self) -> usize
    {

        Arc::strong_count(&self.rw_lock)

    }

    pub fn weak_count(&self) -> usize
    {

        Arc::weak_count(&self.rw_lock)
        
    }

    pub fn downgrade(&self) -> WeakSharedReader<T>
    {

        WeakSharedReader::new(&self.rw_lock)

    }

}
