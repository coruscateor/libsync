use crate::{ItemUpdater, PreferredRwLockType, WakerQueueWithUpdatedItem};


pub struct NotifyingSharedInternals<T, I, U>
    where U: ItemUpdater<I>, 
          I: Clone
{

    pub rw_lock: PreferredRwLockType<T>,
    pub notifier: WakerQueueWithUpdatedItem<I, U>

}

impl<T, I, U> NotifyingSharedInternals<T, I, U>
    where U: ItemUpdater<I>, 
          I: Clone
{

    pub fn new(rw_lock: PreferredRwLockType<T>) -> Self
    {

        Self
        {

            rw_lock,
            notifier: WakerQueueWithUpdatedItem::new()

        }

    }

}