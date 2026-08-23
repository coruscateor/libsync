use std::sync::Arc;

use std::task::Poll;

use inc_dec::IntIncDecSelf;

use crate::{PreferredMutexType, TryLockedReceiver};

use crate::multi_shot::{MultiShotError, MultiShotSharedDetails};

use super::Sender;

#[cfg(feature="use_std_sync")]
use crate::{get_mg, try_get_mg};

pub struct Receiver<T>
{

    shared_details: Arc<PreferredMutexType<MultiShotSharedDetails<T>>>,
    session_number: u32,
    used: bool

}

impl<T> Receiver<T>
{

    pub fn new(shared_details: Arc<PreferredMutexType<MultiShotSharedDetails<T>>>, session_number: u32) -> Self
    {

        Self
        {

            shared_details,
            session_number,
            used: false

        }

    }

    pub fn try_recv(&mut self) -> TryLockedReceiver<Result<Option<T>, MultiShotError>>
    {

        #[cfg(feature="use_std_sync")]
        let mut opt_mg = try_get_mg(&self.shared_details);

        #[cfg(any(feature="use_parking_lot_sync", feature="use_parking_lot_fair_sync"))]
        let mut opt_mg = self.shared_details.try_lock();

        if let Some(mg) = &mut opt_mg
        {

            if mg.session_number == self.session_number && !self.used
            {

                let opt_object = mg.opt_object.take();

                self.used = opt_object.is_some();

                if !self.used && mg.main_side_has_dropped
                {

                    TryLockedReceiver::Result(Err(MultiShotError::Closed))

                    //TryLockedReceiver::Result(Err(opt_object))

                }
                else
                {

                    TryLockedReceiver::Result(Ok(opt_object))

                }

                //TryLockedReceiver::Result(Ok(opt_object))

            }
            else
            {

                TryLockedReceiver::Result(Err(MultiShotError::Irrelevant))

                //TryLockedReceiver::Irrelevant
                
            }

        }
        else
        {

            TryLockedReceiver::WouldBlock

        }

    }

    pub fn recv<'a>(&'a mut self) -> RecvOrWait<'a, T> //Result<T, ()>
    {

        RecvOrWait::new( self)

    }

}

pub struct RecvOrWait<'a, T>
{

    receiver_ref: &'a mut Receiver<T>

    //shared_details_ref: &'a Arc<PreferredMutexType<MultiShotSharedDetails<T>>>,
    //used: bool

}

impl<'a, T> RecvOrWait<'a, T>
{

    pub fn new(receiver_ref: &'a mut Receiver<T>) -> Self //(shared_details_ref: &'a Arc<PreferredMutexType<MultiShotSharedDetails<T>>>) -> Self
    {

        Self
        {

            receiver_ref

            //shared_details_ref,
            //used: false

        }

    }

}

impl<'a, T> Future for RecvOrWait<'a, T>
{

    type Output = Result<T, MultiShotError>;

    fn poll(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<Self::Output>
    {

        let mut_self = self.get_mut();
        
        //mut_self.used = true;

        #[cfg(feature="use_std_sync")]
        let mut mg = get_mg(&mut_self.receiver_ref.shared_details); //mut_self.shared_details_ref);

        #[cfg(any(feature="use_parking_lot_sync", feature="use_parking_lot_fair_sync"))]
        let mut mg = self.receiver_ref.shared_details.lock();

        //If the Task wakes up spuriously, then its no big deal. Either the opt_object is occupied or it isn't. 

        if mg.session_number == mut_self.receiver_ref.session_number && !mut_self.receiver_ref.used
        {

            if let Some(object) = mg.opt_object.take()
            {

                mut_self.receiver_ref.used = true;

                Poll::Ready(Ok(object))

            }
            else
            {

                if mg.main_side_has_dropped //Arc::strong_count(&self.receiver_ref.shared_details) == 1 //mut_self.shared_details_ref) == 1
                {

                    mut_self.receiver_ref.used = true;

                    Poll::Ready(Err(MultiShotError::Closed))

                }
                else
                {

                    //Is being dropped?

                    mg.opt_waker = Some(cx.waker().clone());

                    //mg.should_be_awake = false;

                    Poll::Pending
                    
                }

            }

        }
        else
        {

            Poll::Ready(Err(MultiShotError::Irrelevant))
            
        }

    }

}

