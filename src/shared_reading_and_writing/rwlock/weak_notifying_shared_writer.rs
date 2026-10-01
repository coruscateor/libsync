use std::sync::{Arc, Weak};

#[cfg(feature="use_std_sync")]
use std::sync::{ RwLockReadGuard, RwLockWriteGuard, TryLockError };

#[cfg(any(feature="use_parking_lot_sync", feature="use_parking_lot_fair_sync"))]
use parking_lot::{ RwLockReadGuard, RwLockWriteGuard };

use crate::{ItemUpdater, PreferredRwLockType};

use super::{SharedWriter, NotifyingSharedInternals, NotifyingSharedWriter};

pub struct WeakNotifyingSharedWriter<T, I, U>
    where U: ItemUpdater<I>, 
          I: Clone + PartialEq + Unpin
{

    weak_internals: Weak<NotifyingSharedInternals<T, I, U>>

}

impl<T, I, U> WeakNotifyingSharedWriter<T, I, U>
    where U: ItemUpdater<I>, 
          I: Clone + PartialEq + Unpin
{

    pub fn new(internals: &Arc<NotifyingSharedInternals<T, I, U>>) -> Self
    {

        Self
        {

            weak_internals: Arc::downgrade(internals)

        }

    }

    pub fn upgrade(&self) -> Option<NotifyingSharedWriter<T, I, U>>
    {

        if let Some(internals) = self.weak_internals.upgrade()
        {

            Some(NotifyingSharedWriter::from_internals(internals))

        }
        else
        {

            None
            
        }

    }
    
    pub fn strong_count(&self) -> usize
    {

        self.weak_internals.strong_count()

    }

    pub fn weak_count(&self) -> usize
    {

        self.weak_internals.weak_count()
        
    }

}
