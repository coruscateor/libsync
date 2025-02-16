use std::{sync::{atomic::{AtomicBool, AtomicUsize, Ordering}, Arc, Weak}, thread::{sleep, sleep_ms, Thread}, time::Duration};

use crossbeam::queue::ArrayQueue;

//use futures::executor::block_on;

use tokio::{sync::Notify, time::timeout};

use crate::{BoundedSharedDetails, ReceiveError, ReceiveResult, TimeoutReceiveError};

use crate::crossbeam::mpmc::base::array_queue::{Sender, Receiver as BaseReceiver};

//use crate::crossbeam::mpmc::array_queue::

use delegate::delegate;

use std::clone::Clone;

use crate::tokio_helpers::SemaphoreController;

use std::fmt::Debug;

//#[derive(Clone)]
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

            //Remove an avalible permit from the receivers side.

            self.base.receivers_notifier_ref().forget_permit();

            //Add an avalible permit to the senders side.

            self.base.senders_notifier_ref().add_permit();

            //self.base.senders_notifier().notify_one();

        }

        res

    }

    pub async fn recv(&self) -> ReceiveResult<T> //Option<T> 
    {

        //Loop until you send something or there are no more senders.

        loop
        {

            let can_receive_or_not = self.base.receivers_notifier_ref().acquire().await;
    
            let sent;

            match can_receive_or_not
            {
    
                Ok(permit) =>
                {
    
                    permit.forget();

                    sent = self.base.try_recv()?;

                    return Ok(sent);
                    
                }
                Err(_err) =>
                {
    
                    sent = self.base.try_recv()?;

                    return Ok(sent);

                }
    
            }

        }

        /*
        loop
        {

            let can_receive_or_not = self.base.receivers_notifier_ref().acquire().await;
    
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
        
                            //Add a permit for an item to be sent (a slot is now free).
        
                            self.base.senders_notifier_ref().add_permit();
            
                            //return Ok(res);
            
                            return Some(res);

                        }
                        Err(err) =>
                        {
            
                            match err
                            {

                                ReceiveError::Empty => { /* Try again */ },
                                ReceiveError::NoSenders =>
                                {

                                    //return Err(err);

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
        */

    }

    pub async fn recv_or_timeout(&self, timeout_time: Duration) -> Result<T, TimeoutReceiveError>
    {

        let can_receive_or_not = self.base.receivers_notifier_ref().acquire_timeout(timeout_time).await;
    
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

                //Add a permit for an item to be sent (a slot is now free).

                self.base.senders_notifier_ref().add_permit();

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

impl<T> Drop for Receiver<T> //Sender<T>
{

    fn drop(&mut self)
    {

        if self.base.receiver_strong_count() == 1
        {

            //Engage free-for-all mode.

            self.base.receivers_notifier_ref().close();

            self.base.senders_notifier_ref().close();

        }
    
    }

}



