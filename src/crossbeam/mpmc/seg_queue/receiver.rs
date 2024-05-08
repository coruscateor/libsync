use std::{sync::{atomic::{AtomicBool, AtomicUsize, Ordering}, Arc, Weak}, thread::{sleep, sleep_ms, Thread}, time::Duration};

use crossbeam::queue::SegQueue;

use crate::{ReceiveError, ReceiveResult, ReceiverTrait, SharedDetails, ScopedIncrementer};

use delegate::delegate;

pub struct Receiver<T, N = ()>
{

    shared_details: Arc<SharedDetails<SegQueue<T>, N>>, //SegQueue<T>, AtomicUsize, N)>,
    sender_count: Weak<()>

}

impl<T, N> Receiver<T, N>
{

    //pub fn new(shared_details: &Arc<(SegQueue<T>, AtomicUsize, N)>, sender_count: &Arc<()>) -> Self
    pub fn new(shared_details: &Arc<SharedDetails<SegQueue<T>, N>>, sender_count: &Arc<()>) -> Self
    {

        //Increment the active receiver counter.

        //shared_details.1.fetch_add(1, Ordering::SeqCst);

        shared_details.inc_active_receiver_count();

        Self
        {

            shared_details: shared_details.clone(),
            sender_count: Arc::downgrade(&sender_count)

        }

    }

    /*
    pub fn notifier(&self) -> &N
    {

        &self.shared_details.2

    }

    pub fn is_empty(&self) -> bool
    {

        self.shared_details.0.is_empty()

    }

    pub fn len(&self) -> usize
    {

        self.shared_details.0.len()

    }
    */

    delegate!
    {

        to self.shared_details
        {

            //pub fn senders_notifier(&self) -> &N;

            //pub fn senders_notifier_count(&self) -> &AtomicUsize;

            pub fn receivers_notifier(&self) -> &N;
        
            //pub fn receivers_notifier_count(&self) -> &AtomicUsize;

            pub fn current_active_receiver_count(&self) -> usize;

            #[cfg(feature="count_waiting_senders_and_receivers")]
            pub fn temp_inc_receivers_awiting_notification_count<'a>(&'a self) -> ScopedIncrementer<'a>;

            //#[cfg(feature="count_waiting_senders_and_receivers")]
            //pub fn temp_inc_senders_awiting_notification_count<'a>(&'a self) -> ScopedIncrementer<'a>;

        }

    }

    delegate!
    {

        to self.shared_details.queue()
        {
        
            //pub fn capacity(&self) -> usize;
        
            pub fn is_empty(&self) -> bool;
        
            //pub fn is_full(&self) -> bool;

            pub fn len(&self) -> usize;

        }

    }

}

impl<T, N> ReceiverTrait<T> for Receiver<T, N>
{

    fn recv(&self) -> ReceiveResult<T>
    {

        if let Some(res) = self.shared_details.queue().pop() //0.pop()
        {

            return Ok(res);

        }

        if self.sender_count.strong_count() == 0
        {

            return Err(ReceiveError::NoSenders);

        }

        Err(ReceiveError::Empty)

    }

}

impl<T> Clone for Receiver<T>
{

    fn clone(&self) -> Self
    {

        //Increment the active receiver counter.

        self.shared_details.inc_active_receiver_count(); //.1.fetch_add(1, Ordering::SeqCst);

        Self
        { 
            
            shared_details: self.shared_details.clone(),
            sender_count: self.sender_count.clone()
        
        }

    }

}

impl<T, N> Drop for Receiver<T, N>
{

    fn drop(&mut self)
    {

        let remaining_receivers_plus_one = self.shared_details.dec_active_receiver_count(); //.1.fetch_sub(1, Ordering::SeqCst);
        
        if remaining_receivers_plus_one == 1
        {

            //Pop'n'drop values until empty.

            loop 
            {
    
                if let None = self.shared_details.queue().pop() //.0.pop()
                {
    
                    break;
    
                }
    
            }

            //Sleep for a quarter of a second and try and clear the queue again to catch any "stragglers" that might have been missed earlier.

            //Is this necessary?

            sleep(Duration::from_millis(250));

            loop 
            {
    
                if let None = self.shared_details.queue().pop() //.0.pop()
                {
    
                    break;
    
                }
    
            }

        }
        
    }

}


