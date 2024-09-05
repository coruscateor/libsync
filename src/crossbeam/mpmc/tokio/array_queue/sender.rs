use std::{sync::{atomic::{AtomicBool, AtomicUsize, Ordering}, Arc, Weak}, time::Duration};

use crossbeam::queue::ArrayQueue;

use tokio::sync::{Notify, Semaphore};

use crate::{BoundedSendError, BoundedSendResult, BoundedSharedDetails, SendResult, TimeoutBoundedSendError};

use crate::crossbeam::mpmc::array_queue::{Sender as BaseSender, Receiver};

use delegate::delegate;

use std::clone::Clone;

use tokio::time::timeout;

use crate::tokio_helpers::SemaphoreController;

//use futures::executor::block_on;

//https://docs.rs/crossbeam/0.8.4/crossbeam/queue/struct.ArrayQueue.html

//https://docs.rs/tokio/1.37.0/tokio/sync/struct.Notify.html

//https://docs.rs/tokio/1.37.0/tokio/time/fn.timeout.html

//#[derive(Clone)]
pub struct Sender<T>
{

    base: BaseSender<T, SemaphoreController> //Semaphore> //Notify>

}

//The Sender notifies because the queue is empty...

//Awaiting senders, notifying recevers. 

impl<T> Sender<T>
{

    pub fn new(shared_details: &Arc<BoundedSharedDetails<ArrayQueue<T>, SemaphoreController>>, sender_count: Arc<()>, receiver_count: &Arc<()>) -> Self //Semaphore //&Arc<BoundedSharedDetails<ArrayQueue<T>, Notify>>, sender_count: Arc<()>, receiver_count: &Arc<()>) -> Self
    {

        Self
        {

            base: BaseSender::new(shared_details, sender_count, receiver_count)

        }

    }

    delegate!
    {

        to self.base
        {

            pub fn capacity(&self) -> usize;
        
            pub fn is_empty(&self) -> bool;
        
            pub fn is_full(&self) -> bool;
        
            pub fn len(&self) -> usize;
        
            pub fn len_capacity(&self) -> (usize, usize);
        
            pub fn remaining_capacity(&self) -> usize;

        }

    }

    ///
    /// Attempts to send a value, calls notify_one on the notifier if this was successful.
    /// 
    pub fn try_send(&self, value: T) -> Result<(), BoundedSendError<T>>
    {

        let res = self.base.try_send(value);

        if res.is_ok()
        {

            //Add a permit to the receivers_notifier if a value has successfully been sent.

            self.base.receivers_notifier().add_permit();

            //Remove an avalible slot.

            self.base.senders_notifier().forget_permit();

        }

        res

    }
    
    ///
    /// Attempts to send a value, waiting until signalled if the queue is full. Returns BoundedSendError<T>::NoReceiver if there are no receivers on the other end.
    /// 
    pub async fn send(&self, value: T) -> Result<(), BoundedSendError<T>>
    {

        let mut item = value;

        //Loop until you send something or there are no more senders.

        loop
        {

            let acquired_or_not = self.base.senders_notifier().acquire().await;
    
            match acquired_or_not
            {
    
                Ok(permit) =>
                {
    
                    let sent_res = self.base.try_send(item);

                    permit.forget();

                    match sent_res
                    {
            
                        Ok(res) =>
                        {
        
                            //Add a permit for an item to be received.
        
                            self.base.receivers_notifier().add_permit();
            
                            return Ok(res);
            
                        }
                        Err(err) =>
                        {
            
                            match err
                            {
            
                                BoundedSendError::Full(value) =>
                                {

                                    //Try again
                                    
                                    item = value
                                
                                }
                                BoundedSendError::NoReceivers(_) => return Err(err)
            
                            }
            
                        }
            
                    }
    
                }
                Err(_err) =>
                {
    
                    return self.base.try_send(item);
    
                }
    
            }

        }

    }

    //Timeouts

    pub async fn send_or_timeout(&self, value: T, duration: Duration) -> Result<(), TimeoutBoundedSendError<T>>
    {

        let acquired_or_not= self.base.senders_notifier().acquire_timeout(duration).await;

        let sent;

        //let mut is_errord = false;

        match acquired_or_not
        {

            Ok(res) =>
            {

                match res
                {

                    Ok(permit) =>
                    {

                        sent = self.base.try_send(value);

                        permit.forget();

                    }
                    Err(_err) =>
                    {

                        //is_errord = true;

                        sent = self.base.try_send(value);

                    }

                }

            }
            Err(_err) =>
            {

                return Err(TimeoutBoundedSendError::TimedOut(value));

            }

        }

        match sent
        {

            Ok(res) =>
            {

                //Add a permit for an item to be received.

                self.base.receivers_notifier().add_permit();

                Ok(res)

            }
            Err(err) =>
            {

                Err(TimeoutBoundedSendError::NotTimedOut(err))

            }

        }

    }

    //Blocking

    /*
    pub fn blocking_send(&self, value: T) -> Result<(), BoundedSendError<T>>
    {

        block_on(self.send(value))

    }

    pub fn blocking_send_or_timeout(&self, value: T, timeout_time: Duration) -> Result<(), TimeoutBoundedSendError<T>>
    {

        block_on(self.send_or_timeout(value, timeout_time))

    }
    */
    
}

impl<T> Clone for Sender<T>
{

    fn clone(&self) -> Self
    {

        Self
        { 
            
            base: self.base.clone()

        }

    }

}

impl<T> Drop for Sender<T> //Receiver<T>
{

    fn drop(&mut self)
    {

        if self.base.sender_strong_count() == 1
        {

            //Engage free-for-all mode.

            self.base.senders_notifier().close();

            self.base.receivers_notifier().close();

        }
    
    }

}

