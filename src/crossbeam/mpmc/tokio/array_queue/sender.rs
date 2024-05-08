use std::{sync::{atomic::{AtomicBool, AtomicUsize, Ordering}, Arc, Weak}, time::Duration};

use crossbeam::queue::ArrayQueue;

use tokio::sync::Notify;

use crate::{BoundedSendError, BoundedSendResult, BoundedSenderTrait, BoundedSharedDetails, SendResult, SenderTrait, TimeoutBoundedSendError};

use crate::crossbeam::mpmc::array_queue::{Sender as BaseSender, Receiver};

use delegate::delegate;

use std::clone::Clone;

use tokio::time::timeout;

use futures::executor::block_on;

//https://docs.rs/crossbeam/0.8.4/crossbeam/queue/struct.ArrayQueue.html

//https://docs.rs/tokio/1.37.0/tokio/sync/struct.Notify.html

//https://docs.rs/tokio/1.37.0/tokio/time/fn.timeout.html

#[derive(Clone)]
pub struct Sender<T>
{

    base: BaseSender<T, Notify>

}

//The Sender notifies because the queue is empty...

//Awaiting senders, notifying recevers. 

impl<T> Sender<T>
{

    //pub fn new(shared_details: &Arc<(ArrayQueue<T>, AtomicUsize, Notify, AtomicUsize, Notify, AtomicUsize)>, sender_count: &Arc<()>) -> Self
    pub fn new(shared_details: &Arc<BoundedSharedDetails<ArrayQueue<T>, Notify>>, sender_count: &Arc<()>) -> Self
    {

        Self
        {

            base: BaseSender::new(shared_details, sender_count)

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
    pub fn send_notify_one(&self, value: T) -> Result<(), BoundedSendError<T>>
    {

        let res = self.base.send(value);
        
        if res.is_ok()
        {

            self.base.receivers_notifier().notify_one();

        }

        res

    }

    ///
    /// Attempts to send a value, waiting until signalled if the queue is full. Returns BoundedSendError<T>::NoReceiver if there are no receivers on the other end.
    /// 
    pub async fn send_or_wait(&self, value: T) -> Result<(), BoundedSendError<T>>
    {

        let mut send_res = self.base.send(value);

        loop
        {

            match send_res
            {

                Ok(_val) =>
                {

                    return Ok(());

                }
                Err(err) =>
                {

                    if let BoundedSendError::Full(val) = err
                    {

                        //self.base.receivers_notifier_count().fetch_add(1, Ordering::SeqCst);

                        {

                            #[cfg(feature="count_waiting_senders_and_receivers")]
                            let _sc_inc = self.base.temp_inc_senders_awiting_notification_count();

                            self.base.senders_notifier().notified().await;

                        }

                        //self.base.receivers_notifier_count().fetch_sub(1, Ordering::SeqCst);

                        //Try sending again

                        send_res = self.base.send(val);

                    }
                    else
                    {

                        return Err(err);
                        
                    }

                }
                
            }
            
        }

    }

    pub async fn send_or_wait_notify_one(&self, value: T) -> Result<(), BoundedSendError<T>>
    {

        let res = self.send_or_wait(value).await;

        if res.is_ok()
        {

            self.base.receivers_notifier().notify_one();

        }

        res

    }

    //Timeouts

    pub async fn send_or_timeout(&self, value: T, timeout_time: Duration) -> Result<(), TimeoutBoundedSendError<T>>
    {

        let send_res = self.base.send(value);
        
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

                        //self.base.receivers_notifier_count().fetch_add(1, Ordering::SeqCst);

                        let res;

                        {

                            #[cfg(feature="count_waiting_senders_and_receivers")]
                            let _sc_inc = self.base.temp_inc_senders_awiting_notification_count();

                            let notified = self.base.senders_notifier().notified();

                            res = timeout(timeout_time, notified).await;

                        }

                        //self.base.receivers_notifier_count().fetch_sub(1, Ordering::SeqCst);

                        match res
                        {

                            Ok(_) =>
                            {

                                //return Ok(());

                                //continue;

                                //Try sending again if the task has not been timed out.

                                let res = self.base.send(val);

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

                        //Try sending again

                        /*
                        let res = self.base.send(val);

                        match res
                        {

                            Ok(_) =>
                            {

                                return Ok(());
                                
                            },
                            Err(err) =>
                            {

                                return Err(TimeoutSendError::NotTimedOut(err));

                            }

                        }
                        */

                    }
                    BoundedSendError::NoReceivers(_) =>
                    {

                        Err(TimeoutBoundedSendError::NotTimedOut(err))
                        
                    }

                }
            
            }

        }

    }

    pub async fn send_or_timeout_notify_one(&self, value: T, timeout_time: Duration) -> Result<(), TimeoutBoundedSendError<T>>
    {

        let res = self.send_or_timeout(value, timeout_time).await;

        if res.is_ok()
        {

            self.base.receivers_notifier().notify_one();

        }

        res

    }

    //Blocking

    pub fn blocking_send_or_wait(&self, value: T) -> Result<(), BoundedSendError<T>>
    {

        block_on(self.send_or_wait(value))

    }

    pub fn blocking_send_or_wait_notify_one(&self, value: T) -> Result<(), BoundedSendError<T>>
    {

        block_on(self.send_or_wait_notify_one(value))

    }

    pub fn blocking_send_or_timeout(&self, value: T, timeout_time: Duration) -> Result<(), TimeoutBoundedSendError<T>>
    {

        block_on(self.send_or_timeout(value, timeout_time))

    }

    pub async fn blocking_send_or_timeout_notify_one(&self, value: T, timeout_time: Duration) -> Result<(), TimeoutBoundedSendError<T>>
    {

        block_on(self.send_or_timeout_notify_one(value, timeout_time))

    }
    
}

impl<T> BoundedSenderTrait<T> for Sender<T>
{


    delegate!
    {

        to self.base
        {

            fn send(&self, value: T) -> BoundedSendResult<T>;

        }

    }

}
