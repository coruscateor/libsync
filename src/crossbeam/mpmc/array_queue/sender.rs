use std::sync::{atomic::{AtomicBool, AtomicUsize, Ordering}, Arc, Weak};

use crossbeam::queue::ArrayQueue;

use crate::{BoundedSendError, BoundedSendResult, BoundedSharedDetails}; //, ScopedIncrementer};

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
    receiver_count: Weak<()>

}

impl<T, N> Sender<T, N>
{

    pub fn new(shared_details: &Arc<BoundedSharedDetails<ArrayQueue<T>, N>>, sender_count: Arc<()>, receiver_count: &Arc<()>) -> Self
    {

        Self
        {

            shared_details: shared_details.clone(),
            sender_count,
            receiver_count: Arc::downgrade(receiver_count)

        }

    }

    ///
    /// Try sending a value, returning a SendError<T> if this could not be completed.
    /// 
    pub fn try_send(&self, value: T) -> BoundedSendResult<T>
    {

        //let active_receiver_count = self.shared_details.current_active_receiver_count();

        //Is being dropped?

        if self.receiver_count.strong_count() > 0 //active_receiver_count > 0
        {

            let res = self.shared_details.queue().push(value);

            if let Err(val) = res
            {

                return Err(BoundedSendError::Full(val));

            }

            return Ok(());

        }

        Err(BoundedSendError::NoReceivers(value))

    }

    delegate!
    {

        to self.shared_details
        {

            pub fn senders_notifier(&self) -> &N;

            //pub fn senders_notifier_count(&self) -> &AtomicUsize;

            pub fn receivers_notifier(&self) -> &N;

            /*
            pub fn receivers_do_not_wait(&self) -> bool;

            pub fn receivers_do_not_wait_t(&self);

            pub fn senders_do_not_wait(&self) -> bool;
    
            //pub fn senders_do_not_wait_t(&self);
        
            //pub fn receivers_notifier_count(&self) -> &AtomicUsize;

            //pub fn current_active_receiver_count(&self) -> usize;

            #[cfg(feature="count_waiting_senders_and_receivers")]
            pub fn temp_inc_receivers_awaiting_notification_count<'a>(&'a self) -> crate::ScopedIncrementer<'a>;

            #[cfg(feature="count_waiting_senders_and_receivers")]
            pub fn temp_inc_senders_awaiting_notification_count<'a>(&'a self) -> ScopedIncrementer<'a>;
            */

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

    pub fn sender_strong_count(&self) -> usize
    {

        Arc::strong_count(&self.sender_count)

    }

    pub fn sender_weak_count(&self) -> usize
    {

        Arc::weak_count(&self.sender_count)
        
    }

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
            receiver_count: self.receiver_count.clone()
        
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


