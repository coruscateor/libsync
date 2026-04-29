
use std::sync::{Arc, Weak};

use crate::{PreferredRwLockType, shared_reading_and_writing::SharedWriter};

pub struct WeakSharedWriter<T>
{

    weak_rw_lock: Weak<PreferredRwLockType<T>>

}

impl<T> WeakSharedWriter<T>
{

    pub fn new(rw_lock: &Arc<PreferredRwLockType<T>>) -> Self
    {

        Self
        {

            weak_rw_lock: Arc::downgrade(rw_lock)

        }

    }

    pub fn upgrade(&self) -> Option<SharedWriter<T>>
    {

        if let Some(rw_lock) = self.weak_rw_lock.upgrade()
        {

            Some(SharedWriter::from_rw_lock(rw_lock))

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
