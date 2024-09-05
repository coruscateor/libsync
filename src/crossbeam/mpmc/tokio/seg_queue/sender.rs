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

//#[derive(Clone)]
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
    pub fn send(&self, value: T) -> SendResult<T>
    {

        let res = self.base.send(value);

        if res.is_ok()
        {

            //Add a permit to the receivers_notifier if a value has successfully been sent.

            self.base.receivers_notifier().add_permit();

        }

        res

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

/* */
impl<T> Drop for Sender<T>
{

    fn drop(&mut self)
    {

        if self.base.sender_strong_count() == 1
        {

            //Engage free-for-all mode.

            self.base.receivers_notifier().close();

        }
    
    }

}


