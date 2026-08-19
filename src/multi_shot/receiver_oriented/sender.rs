use std::sync::Arc;

use crate::PreferredMutexType;

use crate::multi_shot::multi_shot_shared_details::MultiShotSharedDetails;

#[cfg(feature="use_std_sync")]
use crate::get_mg;

pub struct Sender<T>
{

    shared_details: Arc<PreferredMutexType<MultiShotSharedDetails<T>>>,
    used: bool

}

impl<T> Sender<T>
{

    pub fn new(shared_details: Arc<PreferredMutexType<MultiShotSharedDetails<T>>>) -> Self
    {

        Self
        {

            shared_details,
            used: false

        }

    }

    pub fn send(mut self, object: T)
    {

        self.used = true;

        #[cfg(feature="use_std_sync")]
        let mut mg = get_mg(&self.shared_details);

        #[cfg(any(feature="use_parking_lot_sync", feature="use_parking_lot_fair_sync"))]
        let mut mg = self.waker_queue_internals.lock();

        mg.object = Some(object);

        mg.should_be_awake = true;

        if let Some(waker) = mg.waker.take()
        {

            waker.wake();
            
        }

    }

}

impl<T> Drop for Sender<T>
{

    fn drop(&mut self)
    {

        if !self.used
        {

            #[cfg(feature="use_std_sync")]
            let mut mg = get_mg(&self.shared_details);

            #[cfg(any(feature="use_parking_lot_sync", feature="use_parking_lot_fair_sync"))]
            let mut mg = self.waker_queue_internals.lock();

            mg.should_be_awake = true;

            if let Some(waker) = mg.waker.take()
            {

                waker.wake();

            }

        }

        // SAFETY: `self` is pinned till after dropped.
        //unsafe { Drop::pin_drop(std::pin::Pin::new_unchecked(self)) }

    }

}