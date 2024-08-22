use std::{sync::{atomic::{AtomicBool, AtomicUsize, Ordering}, Arc, Weak}, time::Duration};

use crossbeam::queue::SegQueue;

use tokio::sync::{Notify, Semaphore};

use crate::{BoundedSendError, BoundedSendResult, BoundedSharedDetails, SendResult, SharedDetails, TimeoutBoundedSendError}; //crossbeam::mpmc::tokio::ChannelSemaphore, 

use crate::crossbeam::mpmc::seg_queue::{Sender as BaseSender, Receiver};

use delegate::delegate;

use std::clone::Clone;

use tokio::time::timeout;

use crate::tokio_helpers::SemaphoreController;

//use futures::executor::block_on;

//https://docs.rs/crossbeam/0.8.4/crossbeam/queue/struct.SegQueue.html

//https://docs.rs/tokio/1.37.0/tokio/sync/struct.Notify.html

//https://docs.rs/tokio/1.37.0/tokio/time/fn.timeout.html

#[derive(Clone)]
pub struct Sender<T>
{

    base: BaseSender<T, SemaphoreController> //ChannelSemaphore> //Notify>

}

//The Sender notifies because the queue is empty...

//Awaiting senders, notifying recevers. 

impl<T> Sender<T>
{

    pub fn new(shared_details: &Arc<SharedDetails<SegQueue<T>, SemaphoreController>>, sender_count: Arc<()>, receiver_count: &Arc<()>) -> Self //ChannelSemaphore //Notify>>, sender_count: Arc<()>, receiver_count: &Arc<()>) -> Self
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

        to self.base.receivers_notifier()
        {

            pub fn is_closed(&self) -> bool;

        }

    }

    ///
    /// Attempts to send a value, calls notify_one on the notifier if this was successful.
    /// 
    pub fn send(&self, value: T) -> SendResult<T> //Result<(), SendError<T>>
    {

        let res = self.base.send(value);

        if res.is_ok()
        {

            //Add a permit to the receivers_notifier if a value has successfully been sent.

            self.base.receivers_notifier().add_permit(); //.try_add_permit();

        }

        res

        /*
        let rn = self.base.receivers_notifier();

        if rn.has_permits()
        {

            return SendResult::Err(());

        }
        */

        /*
        let res = self.base.send(value);
        
        if res.is_ok()
        {

            self.base.receivers_notifier().notify_one();

        }

        res
        */

    }

    //
    // Attempts to send a value, waiting until signalled if the queue is full. Returns BoundedSendError<T>::NoReceiver if there are no receivers on the other end.
    //
    
    /*
    pub async fn send(&self, value: T) -> Result<(), BoundedSendError<T>>
    {

        let mut send_res = self.base.try_send(value);

        loop
        {

            match send_res
            {

                Ok(_val) =>
                {

                    //self.base.receivers_notifier().notify_one();

                    return Ok(());

                }
                Err(err) =>
                {

                    if let BoundedSendError::Full(val) = err
                    {

                        {

                            #[cfg(feature="count_waiting_senders_and_receivers")]
                            let _sc_inc = self.base.temp_inc_senders_awaiting_notification_count();

                            self.base.senders_notifier().notified().await;

                        }

                        //Try sending again

                        send_res = self.try_send(val);

                    }
                    else
                    {

                        return Err(err);
                        
                    }

                }
                
            }
            
        }

    }
    */

    //Timeouts

    /*
    pub async fn send_or_timeout(&self, value: T, timeout_time: Duration) -> Result<(), TimeoutBoundedSendError<T>>
    {

        let send_res = self.try_send(value);
        
        match send_res
        {

            Ok(_val) =>
            {

                return Ok(());

            }
            Err(err) =>
            {

                match err
                {

                    BoundedSendError::Full(val) =>
                    {

                        let res;

                        {

                            #[cfg(feature="count_waiting_senders_and_receivers")]
                            let _sc_inc = self.base.temp_inc_senders_awaiting_notification_count();

                            let notified = self.base.senders_notifier().notified();

                            res = timeout(timeout_time, notified).await;

                        }

                        match res
                        {

                            Ok(_) =>
                            {

                                //Try sending again if the task has not been timed out.

                                let res = self.try_send(val);

                                match res
                                {

                                    Ok(_) =>
                                    {

                                        return Ok(());

                                    },
                                    Err(err) =>
                                    {

                                        return Err(TimeoutBoundedSendError::NotTimedOut(err));

                                    }

                                }

                            },
                            Err(_err) =>
                            {

                                return Err(TimeoutBoundedSendError::TimedOut(val));

                            }

                        }

                    }
                    BoundedSendError::NoReceivers(_) =>
                    {

                        Err(TimeoutBoundedSendError::NotTimedOut(err))
                        
                    }

                }
            
            }

        }

    }
    */

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

impl<T> Drop for Sender<T>
{

    fn drop(&mut self)
    {

        if self.base.receiver_strong_count() == 1
        {

            self.base.receivers_notifier().close();

            /*
            self.base.receivers_do_not_wait_t();

            let mut len = self.base.len();

            while len > 0
            {

                self.base.receivers_notifier().notify_one();

                len -= 1;    
                
            }

            self.base.receivers_notifier().notify_waiters();
            */

        }
    
    }

}
