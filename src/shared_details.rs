use std::sync::atomic::{AtomicUsize, Ordering};

#[cfg(feature="count_waiting_senders_and_receivers")]
use crate::ScopedIncrementer;

pub struct SharedDetails<Q, N = ()>
{

    queue: Q,
    active_receiver_count: AtomicUsize,
    receivers_notifier: N,
    #[cfg(feature="count_waiting_senders_and_receivers")]
    receivers_awiting_notification_count: AtomicUsize

}

impl<Q, N> SharedDetails<Q, N>
{

    pub fn new(queue: Q, receivers_notifier: N) -> Self
    {

        Self
        {

            queue,
            active_receiver_count: AtomicUsize::new(0),
            receivers_notifier,
            #[cfg(feature="count_waiting_senders_and_receivers")]
            receivers_awiting_notification_count: AtomicUsize::new(0)

        }

    }

    pub fn queue(&self) -> &Q
    {

        &self.queue

    }

    //active_receiver_count

    pub fn inc_active_receiver_count(&self) -> usize
    {

        self.active_receiver_count.fetch_add(1, Ordering::SeqCst)

    }

    pub fn dec_active_receiver_count(&self) -> usize
    {

        self.active_receiver_count.fetch_sub(1, Ordering::SeqCst)

    }

    pub fn current_active_receiver_count(&self) -> usize
    {

        self.active_receiver_count.load(Ordering::Acquire)

    }

    //

    pub fn receivers_notifier(&self) -> &N
    {

        &self.receivers_notifier

    }

    #[cfg(feature="count_waiting_senders_and_receivers")]
    pub fn temp_inc_receivers_awiting_notification_count<'a>(&'a self) -> ScopedIncrementer<'a>
    {

        ScopedIncrementer::new(&self.receivers_awiting_notification_count) 

    }


}