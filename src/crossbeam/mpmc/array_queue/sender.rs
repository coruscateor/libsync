use std::sync::{atomic::{AtomicBool, AtomicUsize, Ordering}, Arc, Weak};

use crossbeam::queue::ArrayQueue;

use crate::{BoundedSendError, BoundedSendResult, BoundedSenderTrait, BoundedSharedDetails, ScopedIncrementer};

use delegate::delegate;

pub struct Sender<T, N = ()>
{

    //1: Queue
    //2: Active reciver count
    //3: Notify - Senders
    //4: Notify - Senders count
    //3: Notify - Receivers
    //4: Notify - Receivers count

    //shared_details: Arc<(ArrayQueue<T>, AtomicUsize, N, AtomicUsize, N, AtomicUsize)>,
    shared_details: Arc<BoundedSharedDetails<ArrayQueue<T>, N>>,
    sender_count: Arc<()>,
    //reciver_count: Weak<()>

}

impl<T, N> Sender<T, N>
{

    //pub fn new(shared_details: &Arc<(ArrayQueue<T>, AtomicUsize, N, AtomicUsize, N, AtomicUsize)>, sender_count: &Arc<()>) -> Self //, reciver_count: &Weak<()>
    pub fn new(shared_details: &Arc<BoundedSharedDetails<ArrayQueue<T>, N>>, sender_count: &Arc<()>) -> Self
    {

        Self
        {

            shared_details: shared_details.clone(),
            sender_count: sender_count.clone(),
            //reciver_count: reciver_count.clone()

        }

    }

    /*
    pub fn notifier(&self) -> &N
    {

        &self.shared_details.2

    }
    */

    /*
    pub fn senders_notifier(&self) -> &N
    {

        &self.shared_details.2

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

        &self.shared_details.3

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
            pub fn temp_inc_receivers_awiting_notification_count<'a>(&'a self) -> crate::ScopedIncrementer<'a>;

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
    
}

impl<T, N> BoundedSenderTrait<T> for Sender<T, N>
{

    ///
    /// Try sending a value, returning a SendError<T> if this could not be completed.
    /// 
    fn send(&self, value: T) -> BoundedSendResult<T>
    {

        let active_receiver_count = self.shared_details.current_active_receiver_count(); //.1.load(Ordering::Acquire);

        //Is being dropped?

        // !is_being_dropped

        if active_receiver_count > 0
        {

            let res = self.shared_details.queue().push(value); //.0.push(value);

            if let Err(val) = res
            {

                return Err(BoundedSendError::Full(val));

            }

            return Ok(());

        }

        Err(BoundedSendError::NoReceivers(value))

        /* 
        let mut sc = self.reciver_count.strong_count();

        if sc > 1
        {
            */
            //Is dropping???

            /* 
            let res = self.shared_details.0.push(value);

            if let Err(val) = res
            {

                return Err(SendError::Full(val));

            }
            */
            //Double check
            /* 
            sc = self.reciver_count.strong_count();

            if sc < 1
            {

                loop 
                {

                    if let None = self.queue.pop()
                    {

                        return Err(SendError::ValueIrrecoverable);
    
                    }

                }

            }

            return Ok(());

        }

        Err(SendError::NoReceiver(value))
        */
    }

}

//Make sure that the generic parameters match!!!

impl<T, N> Clone for Sender<T, N>
{

    fn clone(&self) -> Self
    {

        //Increment the active receiver counter.

        //self.shared_details.1.fetch_add(1, Ordering::SeqCst);

        Self
        { 
            
            shared_details: self.shared_details.clone(),
            sender_count: self.sender_count.clone(),
            //reciver_count: self.reciver_count.clone()
        
        }

    }

}

//If all the senders drop then all the receivers should be able to receive the messages in the buffer.

/*
impl<T, N> Drop for Sender<T, N>
{

    fn drop(&mut self)
    {

        //ArrayQueue does apparently drop all of its used slots when it's being dropped, however this is a case of "sooner is better than later".   

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


