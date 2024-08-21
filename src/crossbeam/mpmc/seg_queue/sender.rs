use std::sync::{atomic::{AtomicBool, AtomicUsize, Ordering}, Arc, Weak};

use crossbeam::queue::SegQueue;

use crate::{SendResult, SharedDetails}; //, ScopedIncrementer};

use delegate::delegate;

pub struct Sender<T, N = ()>
{

    //2: Active Receiver Count

    //shared_details: Arc<(SegQueue<T>, AtomicUsize, N)>,
    shared_details: Arc<SharedDetails<SegQueue<T>, N>>,
    sender_count: Arc<()>,
    receiver_count: Weak<()>

}

impl<T, N> Sender<T, N>
{

    pub fn new(shared_details: &Arc<SharedDetails<SegQueue<T>, N>>, sender_count: Arc<()>, receiver_count: &Arc<()>) -> Self
    {

        Self
        {

            shared_details: shared_details.clone(),
            sender_count: sender_count.clone(),
            receiver_count: Arc::downgrade(receiver_count)

        }

    }

    pub fn send(&self, value: T) -> SendResult<T>
    {

        //Is being dropped?

        //let active_receiver_count = self.shared_details.current_active_receiver_count();

        if self.receiver_count.strong_count() > 0 //active_receiver_count > 0
        {

            self.shared_details.queue().push(value);

            return Ok(());

        }

        Err(value)

    }
    
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

impl<T, N> Clone for Sender<T, N>
{

    fn clone(&self) -> Self
    {

        Self
        { 
            
            shared_details: self.shared_details.clone(),
            sender_count: self.sender_count.clone(),
            receiver_count: self.receiver_count.clone()
        
        }

    }

}

