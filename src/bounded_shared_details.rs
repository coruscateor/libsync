use std::sync::atomic::{AtomicUsize, Ordering};

use crate::SharedDetails;

use delegate::delegate;

#[cfg(feature="count_waiting_senders_and_receivers")]
use crate::ScopedIncrementer;

pub struct BoundedSharedDetails<Q, N = ()>
{

    shared_details: SharedDetails<Q, N>,
    senders_notifier: N,
    #[cfg(feature="count_waiting_senders_and_receivers")]
    senders_awiting_notification_count: AtomicUsize

}

impl<Q, N> BoundedSharedDetails<Q, N>
{

    pub fn new(queue: Q, receivers_notifier: N, senders_notifier: N) -> Self
    {

        Self
        {

            shared_details: SharedDetails::new(queue, receivers_notifier),
            senders_notifier,
            #[cfg(feature="count_waiting_senders_and_receivers")]
            senders_awiting_notification_count: AtomicUsize::new(0)

        }

    }

    delegate!
    {

        to self.shared_details
        {

            pub fn queue(&self) -> &Q;
        
            //active_receiver_count
        
            pub fn inc_active_receiver_count(&self) -> usize;
        
            pub fn dec_active_receiver_count(&self) -> usize;
        
            pub fn current_active_receiver_count(&self) -> usize;
        
            //
        
            pub fn receivers_notifier(&self) -> &N;

            #[cfg(feature="count_waiting_senders_and_receivers")]
            pub fn temp_inc_receivers_awiting_notification_count<'a>(&'a self) -> ScopedIncrementer<'a>;


        }

    }

    pub fn senders_notifier(&self) -> &N
    {

        &self.senders_notifier

    }

    #[cfg(feature="count_waiting_senders_and_receivers")]
    pub fn temp_inc_senders_awiting_notification_count<'a>(&'a self) -> ScopedIncrementer<'a>
    {

        ScopedIncrementer::new(&self.senders_awiting_notification_count) 

    }

}