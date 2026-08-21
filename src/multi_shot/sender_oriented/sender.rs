use std::sync::Arc;

use inc_dec::IntIncDecSelf;

use crate::PreferredMutexType;

use crate::multi_shot::multi_shot_shared_details::MultiShotSharedDetails;

#[cfg(feature="use_std_sync")]
use crate::get_mg;

use super::Receiver;

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

    pub fn new_recevier(&self) -> Receiver<T>
    {

        let new_session_number;

        {

            #[cfg(feature="use_std_sync")]
            let mut mg = get_mg(&self.shared_details);

            #[cfg(any(feature="use_parking_lot_sync", feature="use_parking_lot_fair_sync"))]
            let mut mg = self.waker_queue_internals.lock();

            new_session_number = mg.session_number.wpp();

        }

        Receiver::new(self.shared_details.clone(), new_session_number)

    }

    pub fn send(&mut self, object: T)
    {

        self.used = true;

        #[cfg(feature="use_std_sync")]
        let mut mg = get_mg(&self.shared_details);

        #[cfg(any(feature="use_parking_lot_sync", feature="use_parking_lot_fair_sync"))]
        let mut mg = self.waker_queue_internals.lock();

        mg.opt_object = Some(object);

        if let Some(waker) = mg.opt_waker.take()
        {

            waker.wake();
            
        }

    }

    pub fn cancel(&self)
    {

        #[cfg(feature="use_std_sync")]
        let mut mg = get_mg(&self.shared_details);

        #[cfg(any(feature="use_parking_lot_sync", feature="use_parking_lot_fair_sync"))]
        let mut mg = self.waker_queue_internals.lock();

        mg.session_number.wpp();        

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

            if let Some(waker) = mg.opt_waker.take()
            {

                waker.wake();

            }

        }

        // SAFETY: `self` is pinned till after dropped.
        //unsafe { Drop::pin_drop(std::pin::Pin::new_unchecked(self)) }

    }

}
