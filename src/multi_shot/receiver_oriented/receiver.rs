use std::sync::Arc;
use std::task::Poll;

use crate::PreferredMutexType;

use crate::multi_shot::multi_shot_shared_details::MultiShotSharedDetails;

use super::Sender;

#[cfg(feature="use_std_sync")]
use crate::get_mg;

pub struct Receiver<T>
{

    shared_details: Arc<PreferredMutexType<MultiShotSharedDetails<T>>>

}

impl<T> Receiver<T>
{

    pub fn new(shared_details: Arc<PreferredMutexType<MultiShotSharedDetails<T>>>) -> Self
    {

        Self
        {

            shared_details

        }

    }

    pub fn new_sender(&self) -> Result<Sender<T>, ()>
    {

        if Arc::strong_count(&self.shared_details) == 1
        {

            Ok(Sender::new(self.shared_details.clone()))

        }
        else
        {

            Err(())
            
        }

    }

    pub fn try_recv(&self) -> Option<T>
    {

        #[cfg(feature="use_std_sync")]
        let mut mg = get_mg(&self.shared_details);

        #[cfg(any(feature="use_parking_lot_sync", feature="use_parking_lot_fair_sync"))]
        let mut mg = self.waker_queue_internals.lock();

        mg.object.take()

    }

    pub fn recv<'a>(&'a self) -> RecvOrWait<'a, T> //Result<T, ()>
    {

        RecvOrWait::new(&self.shared_details)

    }

}

pub struct RecvOrWait<'a, T>
{

    shared_details_ref: &'a Arc<PreferredMutexType<MultiShotSharedDetails<T>>>,
    used: bool

}

impl<'a, T> RecvOrWait<'a, T>
{

    pub fn new(shared_details_ref: &'a Arc<PreferredMutexType<MultiShotSharedDetails<T>>>) -> Self
    {

        Self
        {

            shared_details_ref,
            used: false

        }

    }

}

impl<'a, T> Future for RecvOrWait<'a, T>
{

    type Output = Result<T, ()>;

    fn poll(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<Self::Output>
    {

        let mut_self = self.get_mut();
        
        mut_self.used = true;

        #[cfg(feature="use_std_sync")]
        let mut mg = get_mg(mut_self.shared_details_ref);

        #[cfg(any(feature="use_parking_lot_sync", feature="use_parking_lot_fair_sync"))]
        let mut mg = self.waker_queue_internals.lock();

        if !mg.should_be_awake
        {

            return Poll::Pending;

        }

        if let Some(res) = mg.object.take()
        {

            Poll::Ready(Ok(res))

        }
        else
        {

            if Arc::strong_count(mut_self.shared_details_ref) == 1
            {

                Poll::Ready(Err(()))

            }
            else
            {

                //Is being dropped?

                mg.waker = Some(cx.waker().clone());

                mg.should_be_awake = false;

                Poll::Pending
                
            }

        }

    }

}

impl<'a, T> Drop for RecvOrWait<'a, T>
{

    fn drop(&mut self)
    {

        if !self.used
        {

            

        }

        // SAFETY: `self` is pinned till after dropped.
        //unsafe { Drop::pin_drop(std::pin::Pin::new_unchecked(self)) }
    }
}
