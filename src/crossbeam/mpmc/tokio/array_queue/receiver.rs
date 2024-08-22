use std::{sync::{atomic::{AtomicBool, AtomicUsize, Ordering}, Arc, Weak}, thread::{sleep, sleep_ms, Thread}, time::Duration};

use crossbeam::queue::ArrayQueue;

//use futures::executor::block_on;

use tokio::{sync::Notify, time::timeout};

use crate::{BoundedSharedDetails, ReceiveError, ReceiveResult, TimeoutReceiveError};

use crate::crossbeam::mpmc::array_queue::{Sender, Receiver as BaseReceiver};

//use crate::crossbeam::mpmc::array_queue::

use delegate::delegate;

use std::clone::Clone;

use crate::tokio_helpers::SemaphoreController;

#[derive(Clone)]
pub struct Receiver<T>
{

    base: BaseReceiver<T, SemaphoreController> //Notify>

}

//The Recever notifies because the queue is full...

impl<T> Receiver<T>
{

    pub fn new(shared_details: Arc<BoundedSharedDetails<ArrayQueue<T>, SemaphoreController>>, sender_count: Weak<()>, receiver_count: Arc<()>,) -> Self //Notify
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

            pub fn capacity(&self) -> usize;
        
            pub fn is_empty(&self) -> bool;
        
            pub fn is_full(&self) -> bool;
        
            pub fn len(&self) -> usize;
        
            pub fn len_capacity(&self) -> (usize, usize);
        
            pub fn remaining_capacity(&self) -> usize;

        }

    }

    //

    pub fn try_recv(&self) -> ReceiveResult<T>
    {

        let res = self.base.try_recv();

        if res.is_ok()
        {

            //Remove an avalible permit from the senders side.

            self.base.receivers_notifier().forget_permit();

            //Add an avalible permit to the senders side.

            self.base.senders_notifier().add_permit();

            //self.base.senders_notifier().notify_one();

        }

        res

    }

    pub async fn recv(&self) -> ReceiveResult<T>
    {

        //Loop until you send something or there are no more senders.

        loop
        {

            let can_receive_or_not = self.base.receivers_notifier().acquire().await;
    
            match can_receive_or_not
            {
    
                Ok(permit) =>
                {
    
                    let sent_res = self.base.try_recv();

                    permit.forget();

                    match sent_res
                    {
            
                        Ok(res) =>
                        {
        
                            //Add a permit for an item to be sent.
        
                            self.base.senders_notifier().add_permit();
            
                            return Ok(res);
            
                        }
                        Err(err) =>
                        {
            
                            match err
                            {

                                ReceiveError::Empty => { /* Try again */ },
                                ReceiveError::NoSenders =>
                                {

                                    return Err(err);

                                }

                            }
            
                        }
            
                    }
    
                }
                Err(_err) =>
                {
    
                    return self.base.try_recv();
    
                }
    
            }

        }        

        /*
        loop
        {

            let pop_res = self.base.try_recv();

            match pop_res
            {

                Ok(val) =>
                {

                    self.base.senders_notifier().notify_one();

                    return Ok(val);

                }
                Err(err) =>
                {

                    if let ReceiveError::Empty = err
                    {
                        
                        if self.base.receivers_do_not_wait()
                        {

                            return Err(ReceiveError::NoSenders);

                        }

                        #[cfg(feature="count_waiting_senders_and_receivers")]
                        let _sc_inc = self.base.temp_inc_receivers_awaiting_notification_count();

                        self.base.receivers_notifier().notified().await;

                    }
                    else
                    {

                        return Err(err);
                        
                    }

                }
                
            }
            
        }
        */

    }

    pub async fn recv_or_timeout(&self, timeout_time: Duration) -> Result<T, TimeoutReceiveError>
    {

        let can_receive_or_not = self.base.receivers_notifier().acquire_timeout(timeout_time).await;
    
        let recvd;

        match can_receive_or_not
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

                //Add a permit for an item to be sent.

                self.base.senders_notifier().add_permit();

                Ok(res)

            }
            Err(err) =>
            {

                Err(TimeoutReceiveError::NotTimedOut(err))

            }

        }

        /*
        let recv_res = self.base.try_recv();
        
        match recv_res
        {

            Ok(val) =>
            {

                self.base.senders_notifier().notify_one();

                return Ok(val);

            }
            Err(err) =>
            {

                match err
                {

                    ReceiveError::Empty =>
                    {

                        let res;

                        {

                            if self.base.receivers_do_not_wait()
                            {

                                return Err(TimeoutReceiveError::NotTimedOut(ReceiveError::NoSenders));

                            }

                            #[cfg(feature="count_waiting_senders_and_receivers")]
                            let _sc_inc = self.base.temp_inc_receivers_awaiting_notification_count();

                            let notified = self.base.receivers_notifier().notified();

                            res = timeout(timeout_time, notified).await;

                        }

                        match res
                        {

                            Ok(_) =>
                            {

                                //Try sending again if the task has not been timed out.

                                let res = self.try_recv();

                                match res
                                {

                                    Ok(res) =>
                                    {

                                        return Ok(res);

                                    },
                                    Err(err) =>
                                    {

                                        return Err(TimeoutReceiveError::NotTimedOut(err));

                                    }

                                }

                            },
                            Err(_err) =>
                            {

                                return Err(TimeoutReceiveError::TimedOut);

                            }

                        }

                    }
                    ReceiveError::NoSenders =>
                    {

                        Err(TimeoutReceiveError::NotTimedOut(err))
                        
                    }

                }
            
            }

        }
        */

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

impl<T> Drop for Receiver<T>
{

    fn drop(&mut self)
    {

        if self.base.sender_strong_count() == 1
        {

            self.base.senders_notifier().close();

            /*
            self.base.senders_do_not_wait_t();

            let mut len = self.base.len();

            while len > 0
            {

                self.base.senders_notifier().notify_one();

                len -= 1;    
                
            }

            self.base.senders_notifier().notify_waiters();

            */

        }
    
    }

}





