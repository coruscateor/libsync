use std::{sync::{atomic::{AtomicBool, AtomicUsize, Ordering}, Arc, Weak}, thread::{sleep, sleep_ms, Thread}, time::Duration};

use crossbeam::queue::ArrayQueue;

use crate::{ReceiveError, ReceiveResult, ReceiverTrait, BoundedSharedDetails, ScopedIncrementer};

use delegate::delegate;

//#[derive(Clone)]
pub struct Receiver<T, N = ()>
{

    //1: Queue
    //2: Active reciver count
    //3: Notify - Senders
    //4: Notify - Senders count
    //3: Notify - Receivers
    //4: Notify - Receivers count

    //queue: Arc<ArrayQueue<T>>,
    //shared_details: Arc<(ArrayQueue<T>, AtomicUsize, N, AtomicUsize, N, AtomicUsize)>,
    shared_details: Arc<BoundedSharedDetails<ArrayQueue<T>, N>>,
    //reciver_count: Arc<()>,
    sender_count: Weak<()>

}

impl<T, N> Receiver<T, N>
{

    //pub fn new(shared_details: &Arc<(ArrayQueue<T>, AtomicUsize, N, AtomicUsize, N, AtomicUsize)>, sender_count: &Arc<()>) -> Self //reciver_count: &Arc<()>,
    pub fn new(shared_details: &Arc<BoundedSharedDetails<ArrayQueue<T>, N>>, sender_count: &Arc<()>) -> Self
    {

        //Increment the active receiver counter.

        //shared_details.1.fetch_add(1, Ordering::SeqCst);

        shared_details.inc_active_receiver_count();

        Self
        {

            //queue: queue.clone(),
            shared_details: shared_details.clone(),
            //reciver_count: reciver_count.clone(),
            sender_count: Arc::downgrade(&sender_count)

        }

    }

    /*
    pub fn senders_notifier(&self) -> &N
    {

        //&self.shared_details.2

        self.shared_details.senders_notifier()

    }
    pub fn senders_notifier_count(&self) -> &AtomicUsize
    {

        &self.shared_details.3

    }

    pub fn receivers_notifier(&self) -> &N
    {

        &self.shared_details.4

    }

    pub fn receivers_notifier_count(&self) -> &AtomicUsize
    {

        &self.shared_details.5

    }

    pub fn capacity(&self) -> usize
    {

        self.shared_details.0.capacity()

    }

    pub fn is_empty(&self) -> bool
    {

        self.shared_details.0.is_empty()

    }

    pub fn is_full(&self) -> bool
    {

        self.shared_details.0.is_full()

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

            pub fn senders_notifier(&self) -> &N;

            //pub fn senders_notifier_count(&self) -> &AtomicUsize;

            pub fn receivers_notifier(&self) -> &N;
        
            //pub fn receivers_notifier_count(&self) -> &AtomicUsize;

            pub fn current_active_receiver_count(&self) -> usize;

            #[cfg(feature="count_waiting_senders_and_receivers")]
            pub fn temp_inc_receivers_awiting_notification_count<'a>(&'a self) -> ScopedIncrementer<'a>;

            #[cfg(feature="count_waiting_senders_and_receivers")]
            pub fn temp_inc_senders_awiting_notification_count<'a>(&'a self) -> ScopedIncrementer<'a>;

        }

    }

    delegate!
    {

        to self.shared_details.queue()
        {
        
            pub fn capacity(&self) -> usize;
        
            pub fn is_empty(&self) -> bool;
        
            pub fn is_full(&self) -> bool;

            pub fn len(&self) -> usize;

        }

    }

    pub fn len_capacity(&self) -> (usize, usize)
    {

        (self.len(), self.capacity())

    }

    pub fn remaining_capacity(&self) -> usize
    {

        self.capacity() - self.len()

    }

    //

    /*
    pub fn current_senders_notifier_count(&self) -> usize
    {

        self.shared_details.3.load(Ordering::Acquire)

    }

    pub fn current_receivers_notifier_count(&self) -> usize
    {

        self.shared_details.5.load(Ordering::Acquire)

    }

    //

    pub fn inc_senders_notifier_count(&self) -> usize
    {

        self.shared_details.3.fetch_add(1, Ordering::SeqCst)

    }

    pub fn dec_senders_notifier_count(&self) -> usize
    {

        self.shared_details.3.fetch_sub(1, Ordering::SeqCst)

    }

    pub fn inc_receivers_notifier_count(&self) -> usize
    {

        self.shared_details.5.fetch_add(1, Ordering::SeqCst)

    }

    pub fn dec_receivers_notifier_count(&self) -> usize
    {

        self.shared_details.5.fetch_sub(1, Ordering::SeqCst)

    }
    */


    /*
    pub fn not_clone(&self) -> Self
    {

        self.clone()

    }
    */

}

impl<T, N> ReceiverTrait<T> for Receiver<T, N>
{

    ///
    /// Try to receive a value immediately.
    /// 
    fn recv(&self) -> ReceiveResult<T>
    {

        if let Some(res) = self.shared_details.queue().pop() //.0.pop()
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

impl<T, N> Clone for Receiver<T, N>
{

    fn clone(&self) -> Self
    {

        //Increment the active receiver counter.

        //self.shared_details.1.fetch_add(1, Ordering::SeqCst);

        self.shared_details.inc_active_receiver_count();

        Self
        { 
            
            shared_details: self.shared_details.clone(),
            //reciver_count: self.reciver_count.clone(),
            sender_count: self.sender_count.clone()
        
        }

    }

}

/*

Above:

!=

impl<T> Clone for Receiver<T>
{

    fn clone(&self) -> Self
    {

        //Increment the active receiver counter.

        self.shared_details.1.fetch_add(1, Ordering::SeqCst);

        Self
        { 
            
            shared_details: self.shared_details.clone(),
            //reciver_count: self.reciver_count.clone(),
            sender_count: self.sender_count.clone()
        
        }

    }

}

 */

impl<T, N> Drop for Receiver<T, N>
{

    fn drop(&mut self)
    {

        //let sc = Arc::strong_count(&self.reciver_count);

        let remaining_receivers_plus_one = self.shared_details.dec_active_receiver_count(); //self.shared_details.1.fetch_sub(1, Ordering::SeqCst);
        
        //let actual_remaining_receivers = remaining_receivers - 1;

        //actual_

        if remaining_receivers_plus_one == 1 //sc == 1
        {

            //self.shared_details.1.store(true, Ordering::SeqCst);

            //Set is dropping???

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

    /*
    fn drop(&mut self)
    {

        let sc = Arc::strong_count(&self.reciver_count);

        if sc == 1
        {*/

            //Set is dropping???

            //Pop'n'drop values until empty. 

            /*
            loop 
            {
    
                if let None = self.queue.pop()
                {
    
                    break;
    
                }
    
            }

        }
    
    }*/

}

