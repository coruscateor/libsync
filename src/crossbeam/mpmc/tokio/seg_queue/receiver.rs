use std::{sync::{atomic::{AtomicBool, AtomicUsize, Ordering}, Arc, Weak}, thread::{sleep, sleep_ms, Thread}, time::Duration};

use crossbeam::queue::SegQueue;

//use futures::executor::block_on;

use tokio::{sync::Notify, time::timeout};

use crate::{crossbeam::mpmc::tokio::ChannelSemaphore, BoundedSharedDetails, ReceiveError, ReceiveResult, SharedDetails, TimeoutReceiveError};

use crate::crossbeam::mpmc::seg_queue::{Sender, Receiver as BaseReceiver};

//use crate::crossbeam::mpmc::array_queue::

use delegate::delegate;

use std::clone::Clone;

//#[derive(Clone)]
pub struct Receiver<T>
{

    base: BaseReceiver<T, ChannelSemaphore> //Notify>

}

//The Recever notifies because the queue is full...

impl<T> Receiver<T>
{

    pub fn new(shared_details: Arc<SharedDetails<SegQueue<T>, ChannelSemaphore>>, sender_count: Weak<()>, receiver_count: Arc<()>,) -> Self //Notify>>, sender_count: Weak<()>, receiver_count: Arc<()>,) -> Self
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

        to self.base.receivers_notifier()
        {

            pub fn is_closed(&self) -> bool;

        }

    }

    //

    pub fn try_recv(&self) -> ReceiveResult<T>
    {

        if self.base.receivers_notifier().has_waiters()
        {

            return ReceiveResult::Err(ReceiveError::Empty);

        }

        let res = self.base.try_recv();

        res

    }

    pub async fn recv(&self) -> ReceiveResult<T>
    {

        loop
        {

            let acquired_or_not = self.base.receivers_notifier().try_acquire().await;

            let recvd;

            match acquired_or_not
            {

                Some(res) =>
                {

                    match res
                    {

                        Ok(permit) =>
                        {

                            permit.forget();

                            recvd = self.base.try_recv();

                        }
                        Err(_err) =>
                        {

                            return self.base.try_recv();

                        }

                    }

                }
                None =>
                {

                    recvd = self.base.try_recv();

                }

            }

            //if recvd is empty then an error has occured, go back and wait again.

            match recvd
            {

                Ok(res) =>
                {

                    return Ok(res);

                }
                Err(err) =>
                {

                    match err
                    {

                        ReceiveError::Empty => {},
                        ReceiveError::NoSenders => return Err(err)

                    }

                }

            }

        }

            /*
            let pop_res = self.base.try_recv(); //self.try_recv();

            match pop_res
            {

                Ok(val) =>
                {

                    //self.base.senders_notifier().notify_one();

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
            */
            
        //}

    }

    pub async fn recv_or_timeout(&self, duration: Duration) -> Result<T, TimeoutReceiveError>
    {

        let recvd;

        let mut acquired_or_not= self.base.receivers_notifier().try_acquire_timeout(duration).await;

        loop
        {

            match acquired_or_not
            {
    
                Some(res) =>
                {
    
                    match res
                    {
    
                        Ok(permit_res) =>
                        {
    
                            match permit_res
                            {
    
                                Ok(permit) =>
                                {
    
                                    permit.forget();
    
                                    recvd = self.base.try_recv();

                                    break;
    
                                }
                                Err(_err) =>
                                {
    
                                    recvd = self.base.try_recv();

                                    break;
    
                                }
    
                            }
    
                        }
                        Err(_err) =>
                        {
    
                            return Err(TimeoutReceiveError::TimedOut);
    
                            /*
                            let could_be_notified = self.base.receivers_notifier().try_acquire().await;
    
                            match could_be_notified
                            {
                                Some(notified) =>
                                {
    
                                    res = timeout(timeout_time, notified).await?;
    
                                }
                                None =>
                                {
    
    
    
                                }
    
                            }
                            */
    
                        }
    
                    }
    
                }
                None =>
                {
    
                    let recvd = self.base.try_recv();

                    match recvd
                    {
            
                        Ok(res) =>
                        {
            
                            return Ok(res);
            
                        }
                        Err(err) =>
                        {

                            match err
                            {

                                ReceiveError::Empty =>
                                {

                                    //A value is supposed to have been poped, wait here.

                                    acquired_or_not = Some(self.base.receivers_notifier().acquire_timeout(duration).await);

                                }
                                ReceiveError::NoSenders =>
                                {

                                    return Err(TimeoutReceiveError::NotTimedOut(err))

                                }

                            }
            
                        }
            
                    }
    
                }
    
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

        /*
        let recv_res = self.try_recv();
        
        match recv_res
        {

            Ok(val) =>
            {

                //self.base.senders_notifier().notify_one();

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

                            //#[cfg(feature="count_waiting_senders_and_receivers")]
                            //let _sc_inc = self.base.temp_inc_receivers_awaiting_notification_count();

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

/*
impl<T> Drop for Receiver<T>
{

    fn drop(&mut self)
    {

        /*
        if self.base.sender_strong_count() == 1
        {

            self.base.senders_notifier().notify_waiters();

        }
        */
    
    }

}
*/




