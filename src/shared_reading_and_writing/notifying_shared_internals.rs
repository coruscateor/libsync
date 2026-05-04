use crate::{ItemUpdater, PreferredRwLockType, WakerQueueWithUpdatedItem};


pub struct NotifyingSharedInternals<T, I, U>
    where U: ItemUpdater<I>, 
          I: Clone + PartialEq + Unpin
{

    pub rw_lock: PreferredRwLockType<T>,
    pub notifier: WakerQueueWithUpdatedItem<I, U>

}

impl<T, I, U> NotifyingSharedInternals<T, I, U>
    where U: ItemUpdater<I>, 
          I: Clone + PartialEq + Unpin
{

    pub fn new(rw_lock: PreferredRwLockType<T>) -> Self
    {

        Self
        {

            rw_lock,
            notifier: WakerQueueWithUpdatedItem::new()

        }

    }

    pub fn into_inner(self) -> Option<T>
    {

        #[cfg(feature="use_std_sync")]
        match self.rw_lock.into_inner()
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
        Some(self.rw_lock.into_inner())


    }

}