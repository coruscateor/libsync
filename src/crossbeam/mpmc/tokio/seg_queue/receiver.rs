use std::{sync::{atomic::{AtomicBool, AtomicUsize, Ordering}, Arc, Weak}, thread::{sleep, sleep_ms, Thread}, time::Duration};

use crossbeam::queue::SegQueue;

//use futures::executor::block_on;

use tokio::{sync::Notify, time::timeout};

use crate::{BoundedSharedDetails, ReceiveError, ReceiveResult, SharedDetails, TimeoutReceiveError}; //crossbeam::mpmc::tokio::ChannelSemaphore, 

use crate::crossbeam::mpmc::base::seg_queue::{Sender, Receiver as BaseReceiver};

//use crate::crossbeam::mpmc::array_queue::

use delegate::delegate;

use std::clone::Clone;

use crate::tokio_helpers::SemaphoreController;

use std::fmt::Debug;

//#[derive(Clone)]
pub struct Receiver<T>
{

    base: BaseReceiver<T, SemaphoreController>

}

impl<T> Receiver<T>
{

    pub fn new(shared_details: Arc<SharedDetails<SegQueue<T>, SemaphoreController>>, sender_count: Weak<()>, receiver_count: Arc<()>,) -> Self
    {

        Self
        {

            base: BaseReceiver::new(shared_details, sender_count, receiver_count)

        }

    }

    delegate!
    {

        to self.base
        {

            //pub fn try_recv(&self) -> ReceiveResult<T>;

            //pub fn notifier(&self) -> &Notify;

            //pub fn capacity(&self) -> usize;
        
            pub fn is_empty(&self) -> bool;
        
            //pub fn is_full(&self) -> bool;
        
            pub fn len(&self) -> usize;
        
            //pub fn len_capacity(&self) -> (usize, usize);
        
            //pub fn remaining_capacity(&self) -> usize;

        }

    }

    delegate!
    {

        to self.base.receivers_notifier_ref()
        {

            pub fn is_closed(&self) -> bool;

        }

    }

    //

    pub fn try_recv(&self) -> ReceiveResult<T>
    {

        let res = self.base.try_recv();

        //There must be one permit in the receivers_notifier for every item in the queue. 

        if res.is_ok()
        {

            //Only forget a permit if we know one existed.

            self.base.receivers_notifier_ref().forget_permit();

        }

        res

    }
    
    pub async fn recv(&self) -> Option<T> //ReceiveResult<T>
    {

        //Loop until you receive something or there are no more senders.

        loop
        {

            let acquired_or_not = self.base.receivers_notifier_ref().acquire().await;
    
            match acquired_or_not
            {
    
                Ok(permit) =>
                {
    
                    let recvd = self.base.try_recv();

                    permit.forget();

                    match recvd
                    {
                        
                        Ok(res) =>
                        {
            
                            //return Ok(res);

                            return Some(res);
            
                        }
                        Err(err) =>
                        {
            
                            match err
                            {
            
                                ReceiveError::Empty => { /* Try again */ }
                                ReceiveError::NoSenders => //return Err(err)
                                {

                                    return None;

                                }

                            }
            
                        }
            
                    }
    
                }
                Err(_err) =>
                {
    
                    //return self.base.try_recv();
                    
                    if let Ok(res) = self.base.try_recv()
                    {

                        return Some(res);

                    }

                    return None;

                }
    
            }

        }

    }

    pub async fn recv_or_timeout(&self, duration: Duration) -> Result<T, TimeoutReceiveError>
    {

        let acquired_or_not= self.base.receivers_notifier_ref().acquire_timeout(duration).await;

        let recvd;

        match acquired_or_not
        {

            Ok(res) =>
            {

                match res
                {

                    Ok(permit) =>
                    {

                        recvd = self.base.try_recv();

                        permit.forget();

                    }
                    Err(_err) =>
                    {

                        recvd = self.base.try_recv();

                    }

                }

            }
            Err(_err) =>
            {

                return Err(TimeoutReceiveError::TimedOut);

            }

        }

        match recvd
        {

            Ok(res) =>
            {

                Ok(res)

            }
            Err(err) =>
            {

                Err(TimeoutReceiveError::NotTimedOut(err))

            }

        }

    }

    //Blocking

    /*
    pub fn blocking_recv(&self) -> ReceiveResult<T>
    {

        block_on(self.recv())

    }

    pub fn blocking_recv_or_timeout(&self, timeout_time: Duration) -> Result<T, TimeoutReceiveError>
    {

        block_on(self.recv_or_timeout(timeout_time))

    }
    */

}

impl<T> Clone for Receiver<T>
{

    fn clone(&self) -> Self
    {

        Self
        {
            
            base: self.base.clone()
        
        }

    }

}

impl<T> Debug for Receiver<T>
    where T: Debug
{

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Receiver").field("base", &self.base).finish()
    }
    
}

impl<T> Drop for Receiver<T>
{

    fn drop(&mut self)
    {

        if self.base.receiver_strong_count() == 1
        {

            //Engage free-for-all mode.

            self.base.receivers_notifier_ref().close();

        }
    
    }

}





