use std::sync::{Arc, Weak};

#[cfg(feature="use_std_sync")]
use std::sync::{ RwLockReadGuard, RwLockWriteGuard, TryLockError };

use crate::PreferredRwLockType;

use super::SharedReader;

pub struct WeakSharedReader<T>
{

    weak_rw_lock: Weak<PreferredRwLockType<T>>

}

impl<T> WeakSharedReader<T>
{

    pub fn new(rw_lock: &Arc<PreferredRwLockType<T>>) -> Self
    {

        Self
        {

            weak_rw_lock: Arc::downgrade(rw_lock)

        }

    }

    pub fn upgrade(&self) -> Option<SharedReader<T>>
    {

        if let Some(rw_lock) = self.weak_rw_lock.upgrade()
        {

            Some(SharedReader::from_rw_lock(rw_lock))

        }
        else
        {

            None
            
        }

    }
    
    pub fn strong_count(&self) -> usize
    {

        self.weak_rw_lock.strong_count()

    }

    pub fn weak_count(&self) -> usize
    {

        self.weak_rw_lock.weak_count()
        
    }

}
