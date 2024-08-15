use std::{sync::{atomic::{AtomicBool, AtomicUsize, Ordering}, Arc, Weak}, thread::{sleep, sleep_ms, Thread}, time::Duration};

use crossbeam::queue::ArrayQueue;

use crate::{ReceiveError, ReceiveResult, BoundedSharedDetails, ScopedIncrementer};

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
    sender_count: Weak<()>,
    receiver_count: Arc<()>

}

impl<T, N> Receiver<T, N>
{

    pub fn new(shared_details: Arc<BoundedSharedDetails<ArrayQueue<T>, N>>, sender_count: Weak<()>, receiver_count: Arc<()>) -> Self
    {

        //Increment the active receiver counter.

        //shared_details.inc_active_receiver_count();

        Self
        {

            shared_details: shared_details.clone(),
            sender_count, //: Arc::downgrade(&sender_count)
            receiver_count

        }

    }

    ///
    /// Try to receive a value immediately.
    /// 
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

            pub fn senders_notifier(&self) -> &N;

            //pub fn senders_notifier_count(&self) -> &AtomicUsize;

            pub fn receivers_notifier(&self) -> &N;
        
            //pub fn receivers_notifier_count(&self) -> &AtomicUsize;

            //pub fn current_active_receiver_count(&self) -> usize;

            #[cfg(feature="count_waiting_senders_and_receivers")]
            pub fn temp_inc_receivers_awaiting_notification_count<'a>(&'a self) -> ScopedIncrementer<'a>;

            #[cfg(feature="count_waiting_senders_and_receivers")]
            pub fn temp_inc_senders_awaiting_notification_count<'a>(&'a self) -> ScopedIncrementer<'a>;

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

    /*
    delegate!
    {

        to self.receiver_count
        {

            #[call(strong_count)]
            pub fn receiver_strong_count(&self) -> usize;

            #[call(weak_count)]
            pub fn receiver_weak_count(&self) -> usize;

        }

    }
    */

    pub fn len_capacity(&self) -> (usize, usize)
    {

        (self.len(), self.capacity())

    }

    pub fn remaining_capacity(&self) -> usize
    {

        self.capacity() - self.len()

    }

}

impl<T, N> Clone for Receiver<T, N>
{

    fn clone(&self) -> Self
    {

        //Increment the active receiver counter.

        //self.shared_details.1.fetch_add(1, Ordering::SeqCst);

        //self.shared_details.inc_active_receiver_count();

        Self
        { 
            
            shared_details: self.shared_details.clone(),
            receiver_count: self.receiver_count.clone(),
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

        //let _remaining_receivers_plus_one = self.shared_details.dec_active_receiver_count(); //self.shared_details.1.fetch_sub(1, Ordering::SeqCst);
        
        //self.shared_details.senders_notifier().

        //let actual_remaining_receivers = remaining_receivers - 1;

        //actual_

        /*
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

            /*
            sleep(Duration::from_millis(250));

            loop 
            {
    
                if let None = self.shared_details.queue().pop() //.0.pop()
                {
    
                    break;
    
                }
    
            }
            */

        }
        */
        
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


