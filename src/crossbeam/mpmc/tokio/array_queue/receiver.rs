use std::{sync::{atomic::{AtomicBool, AtomicUsize, Ordering}, Arc, Weak}, thread::{sleep, sleep_ms, Thread}, time::Duration};

use crossbeam::queue::ArrayQueue;

use tokio::{sync::Notify, time::timeout};

use crate::{BoundedSharedDetails, ReceiveError, ReceiveResult, ReceiverTrait, TimeoutReceiveError};

use crate::crossbeam::mpmc::array_queue::{Sender, Receiver as BaseReceiver};

//use crate::crossbeam::mpmc::array_queue::

use delegate::delegate;

use std::clone::Clone;

#[derive(Clone)]
pub struct Receiver<T>
{

    //base: BaseReceiver<T, ()> //Notify>
    base: BaseReceiver<T, Notify>

}

//The Recever notifies because the queue is full...

impl<T> Receiver<T>
{

    //pub fn new(shared_details: &Arc<(ArrayQueue<T>, AtomicUsize, ())>, sender_count: &Arc<()>) -> Self //Notify
    //pub fn new(shared_details: &Arc<(ArrayQueue<T>, AtomicUsize, Notify, AtomicUsize, Notify, AtomicUsize)>, sender_count: &Arc<()>) -> Self
    pub fn new(shared_details: &Arc<BoundedSharedDetails<ArrayQueue<T>, Notify>>, sender_count: &Arc<()>) -> Self
    {

        Self
        {

            base: BaseReceiver::new(shared_details, sender_count)

        }

    }

    delegate!
    {

        to self.base
        {

            //pub fn notifier(&self) -> &Notify;

            pub fn capacity(&self) -> usize;
        
            pub fn is_empty(&self) -> bool;
        
            pub fn is_full(&self) -> bool;
        
            pub fn len(&self) -> usize;
        
            pub fn len_capacity(&self) -> (usize, usize);
        
            pub fn remaining_capacity(&self) -> usize;

        }

    }

    /*
    pub fn senders_count(&self) -> usise
    {

        self.base.

    }
    */

    //

    pub fn recv_notify_one(&self) -> ReceiveResult<T>
    {

        let res = self.base.recv();

        if res.is_ok()
        {

            self.base.senders_notifier().notify_one();

        }

        res

    }

    pub async fn recv_or_wait(&self) -> ReceiveResult<T>
    {

        loop
        {

            let pop_res = self.base.recv();

            match pop_res
            {

                Ok(val) =>
                {

                    return Ok(val);

                }
                Err(err) =>
                {

                    if let ReceiveError::Empty = err
                    {

                        {

                            #[cfg(feature="count_waiting_senders_and_receivers")]
                            let _sc_inc = self.base.temp_inc_receivers_awiting_notification_count();

                            self.base.receivers_notifier().notified().await;

                        }

                        //self.base.senders_notifier_count().fetch_add(1, Ordering::SeqCst);

                        //self.base.senders_notifier_count().fetch_sub(1, Ordering::SeqCst);

                    }
                    else
                    {

                        return Err(err);
                        
                    }

                }
                
            }
            
        }

    }

    pub async fn recv_or_timeout(&self, timeout_time: Duration) -> Result<T, TimeoutReceiveError>
    {

        let recv_res = self.base.recv();
        
        match recv_res
        {

            Ok(val) =>
            {

                return Ok(val);

            }
            Err(err) =>
            {

                match err
                {

                    ReceiveError::Empty =>
                    {

                        //self.base.receivers_notifier_count().fetch_add(1, Ordering::SeqCst);

                        let res;

                        {

                            #[cfg(feature="count_waiting_senders_and_receivers")]
                            let _sc_inc = self.base.temp_inc_receivers_awiting_notification_count();

                            let notified = self.base.receivers_notifier().notified();

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

                                let res = self.base.recv();

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

    }

}

impl<T> ReceiverTrait<T> for Receiver<T>
{

    delegate!
    {

        to self.base
        {

            fn recv(&self) -> ReceiveResult<T>;

        }

    }

}

/*
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
*/

/*
no method named `clone` found for struct `mpmc::array_queue::receiver::Receiver` in the current scope
items from traits can only be used if the trait is implemented and in scope
the following trait defines an item `clone`, perhaps you need to implement it:
candidate #1: `Clone`rustcClick for full compiler diagnostic
receiver.rs(7, 1): method `clone` not found for this struct
 */



