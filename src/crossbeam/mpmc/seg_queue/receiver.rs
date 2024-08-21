use std::{sync::{atomic::{AtomicBool, AtomicUsize, Ordering}, Arc, Weak}, thread::{sleep, sleep_ms, Thread}, time::Duration};

use crossbeam::queue::SegQueue;

use crate::{ReceiveError, ReceiveResult, SharedDetails}; //, ScopedIncrementer};

use delegate::delegate;

pub struct Receiver<T, N = ()>
{

    shared_details: Arc<SharedDetails<SegQueue<T>, N>>,
    sender_count: Weak<()>,
    receiver_count: Arc<()>

}

impl<T, N> Receiver<T, N>
{

    pub fn new(shared_details: Arc<SharedDetails<SegQueue<T>, N>>, sender_count: Weak<()>, receiver_count: Arc<()>) -> Self
    {

        //Increment the active receiver counter.

        //shared_details.inc_active_receiver_count();

        Self
        {

            shared_details, //: shared_details.clone(),
            sender_count, //: Arc::downgrade(&sender_count)
            receiver_count

        }

    }

    pub fn try_recv(&self) -> ReceiveResult<T>
    {

        if let Some(res) = self.shared_details.queue().pop()
        {

            return Ok(res);

        }

        if self.sender_count.strong_count() == 0
        {

            return Err(ReceiveError::NoSenders);

        }

        Err(ReceiveError::Empty)

    }

    /*
    pub fn recv(&self) -> Option<T>
    {

        self.shared_details.queue().pop()

    }
    */

    delegate!
    {

        to self.shared_details
        {

            //pub fn senders_notifier(&self) -> &N;

            //pub fn senders_notifier_count(&self) -> &AtomicUsize;

            pub fn receivers_notifier(&self) -> &N;

            /*
            pub fn receivers_do_not_wait(&self) -> bool;

            pub fn receivers_do_not_wait_t(&self);
        
            //pub fn receivers_notifier_count(&self) -> &AtomicUsize;

            //pub fn current_active_receiver_count(&self) -> usize;

            #[cfg(feature="count_waiting_senders_and_receivers")]
            pub fn temp_inc_receivers_awaiting_notification_count<'a>(&'a self) -> ScopedIncrementer<'a>;

            //#[cfg(feature="count_waiting_senders_and_receivers")]
            //pub fn temp_inc_senders_awiting_notification_count<'a>(&'a self) -> ScopedIncrementer<'a>;
            */

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

    delegate!
    {

        to self.sender_count
        {

            #[call(strong_count)]
            pub fn sender_strong_count(&self) -> usize;

            #[call(weak_count)]
            pub fn sender_weak_count(&self) -> usize;

        }

    }

    pub fn receiver_strong_count(&self) -> usize
    {

        Arc::strong_count(&self.receiver_count)

    }

    pub fn receiver_weak_count(&self) -> usize
    {

        Arc::weak_count(&self.receiver_count)
        
    }

}

impl<T> Clone for Receiver<T>
{

    fn clone(&self) -> Self
    {

        //Increment the active receiver counter.

        //self.shared_details.inc_active_receiver_count();

        Self
        { 
            
            shared_details: self.shared_details.clone(),
            sender_count: self.sender_count.clone(),
            receiver_count: self.receiver_count.clone()
        
        }

    }

}

impl<T, N> Drop for Receiver<T, N>
{

    fn drop(&mut self)
    {

        //let _remaining_receivers_plus_one = self.shared_details.dec_active_receiver_count();
        
        /*
        if remaining_receivers_plus_one == 1
        {

            //Pop'n'drop values until empty.

            loop 
            {
    
                if let None = self.shared_details.queue().pop()
                {
    
                    break;
    
                }
    
            }

            //Sleep for a quarter of a second and try and clear the queue again to catch any "stragglers" that might have been missed earlier.

            //Is this necessary?

            //Probably not

            /*
            sleep(Duration::from_millis(250));

            loop 
            {
    
                if let None = self.shared_details.queue().pop()
                {
    
                    break;
    
                }
    
            }
            */

        }
        */
        
    }

}


