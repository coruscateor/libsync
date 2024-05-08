use std::sync::{atomic::{AtomicBool, AtomicUsize, Ordering}, Arc, Weak};

use crossbeam::queue::SegQueue;

use crate::{SendResult, SenderTrait, SharedDetails, ScopedIncrementer};

use delegate::delegate;

pub struct Sender<T, N = ()>
{

    //2: Active Receiver Count

    //shared_details: Arc<(SegQueue<T>, AtomicUsize, N)>,
    shared_details: Arc<SharedDetails<SegQueue<T>, N>>,
    sender_count: Arc<()>,

}

impl<T, N> Sender<T, N>
{

    //pub fn new(shared_details: &Arc<(SegQueue<T>, AtomicUsize, N)>, sender_count: &Arc<()>) -> Self //, reciver_count: &Weak<()>
    pub fn new(shared_details: &Arc<SharedDetails<SegQueue<T>, N>>, sender_count: &Arc<()>) -> Self
    {

        Self
        {

            shared_details: shared_details.clone(),
            sender_count: sender_count.clone(),

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

impl<T, N> SenderTrait<T> for Sender<T, N>
{

    fn send(&self, value: T) -> SendResult<T>
    {

        //Is being dropped?

        let active_receiver_count = self.shared_details.current_active_receiver_count(); //.1.load(Ordering::Acquire);

        if active_receiver_count > 0
        {

            self.shared_details.queue().push(value); //.0.push(value);

            //let res = 

            /*
            if let Err(val) = res
            {

                return Err(SendError::Full(val));

            }
            */

            return Ok(());

        }

        Err(value)

    }

}

impl<T, N> Clone for Sender<T, N>
{

    fn clone(&self) -> Self
    {

        Self
        { 
            
            shared_details: self.shared_details.clone(),
            sender_count: self.sender_count.clone(),
        
        }

    }

}

//If all the senders drop then all the receivers should be able to receive the messages in the buffer.

/*
impl<T, N> Drop for Sender<T, N>
{

    fn drop(&mut self)
    {

        //SegQueue does apparently drop all of its used slots when it's being dropped, however this is a case of "sooner is better than later".   

        if Arc::strong_count(&self.sender_count) == 1
        {

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
*/
