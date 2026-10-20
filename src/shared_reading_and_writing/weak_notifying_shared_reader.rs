use std::sync::{Arc, Weak};

use crate::{ItemUpdater, PreferredRwLockType};

use super::{SharedReader, NotifyingSharedInternals, NotifyingSharedReader};

pub struct WeakNotifyingSharedReader<T, I, U>
    where U: ItemUpdater<I>, 
          I: Clone + PartialEq + Unpin
{

    weak_internals: Weak<NotifyingSharedInternals<T, I, U>>

}

impl<T, I, U> WeakNotifyingSharedReader<T, I, U>
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

    pub fn upgrade(&self) -> Option<NotifyingSharedReader<T, I, U>>
    {

        if let Some(internals) = self.weak_internals.upgrade()
        {

            Some(NotifyingSharedReader::new_current_item_from_notifier(internals))

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
