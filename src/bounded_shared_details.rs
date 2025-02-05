use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

use crate::SharedDetails;

use delegate::delegate;

use std::fmt::Debug;

//#[cfg(feature="count_waiting_senders_and_receivers")]
//use crate::ScopedIncrementer;

pub struct BoundedSharedDetails<Q, N = ()>
{

    shared_details: SharedDetails<Q, N>,
    senders_notifier: N,
    //#[cfg(feature="count_waiting_senders_and_receivers")]
    //senders_awaiting_notification_count: AtomicUsize,
    //senders_do_not_wait: AtomicBool

}

impl<Q, N> BoundedSharedDetails<Q, N>
{

    pub fn new(queue: Q, receivers_notifier: N, senders_notifier: N) -> Self
    {

        Self
        {

            shared_details: SharedDetails::new(queue, receivers_notifier),
            senders_notifier,
            //#[cfg(feature="count_waiting_senders_and_receivers")]
            //senders_awaiting_notification_count: AtomicUsize::new(0),
            //senders_do_not_wait: AtomicBool::new(true)

        }

    }

    delegate!
    {

        to self.shared_details
        {

            pub fn queue_ref(&self) -> &Q;
        
            //active_receiver_count
        
            /*
            pub fn inc_active_receiver_count(&self) -> usize;
        
            pub fn dec_active_receiver_count(&self) -> usize;
        
            pub fn current_active_receiver_count(&self) -> usize;
            */

            //
        
            pub fn receivers_notifier_ref(&self) -> &N;

            /*
            pub fn receivers_do_not_wait(&self) -> bool;

            pub fn receivers_do_not_wait_t(&self);

            #[cfg(feature="count_waiting_senders_and_receivers")]
            pub fn temp_inc_receivers_awaiting_notification_count<'a>(&'a self) -> ScopedIncrementer<'a>;
            */

        }

    }
    
    pub fn senders_notifier_ref(&self) -> &N
    {

        &self.senders_notifier

    }

    /*
    pub fn senders_do_not_wait(&self) -> bool
    {

        self.senders_do_not_wait.load(Ordering::Acquire)

    }
    
    pub fn senders_do_not_wait_t(&self)
    {

        self.senders_do_not_wait.store(true, Ordering::Release);

    }

    #[cfg(feature="count_waiting_senders_and_receivers")]
    pub fn temp_inc_senders_awaiting_notification_count<'a>(&'a self) -> ScopedIncrementer<'a>
    {

        ScopedIncrementer::new(&self.senders_awaiting_notification_count) 

    }
    */

}

impl<Q, N> Debug for BoundedSharedDetails<Q, N>
    where Q: Debug,
          N: Debug
{

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("BoundedSharedDetails").field("shared_details", &self.shared_details).field("senders_notifier", &self.senders_notifier).finish()
    }
    
}