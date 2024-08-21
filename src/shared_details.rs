use std::sync::{atomic::{AtomicBool, AtomicUsize, Ordering}, Arc, Weak};

//#[cfg(feature="count_waiting_senders_and_receivers")]
//use crate::ScopedIncrementer;

pub struct SharedDetails<Q, N = ()>
{

    queue: Q,
    //sender_count: Weak<()>,
    //receiver_count: Weak<()>,
    //active_receiver_count: AtomicUsize,
    receivers_notifier: N,
    //#[cfg(feature="count_waiting_senders_and_receivers")]
    //receivers_awiting_notification_count: AtomicUsize,
    //receivers_do_not_wait: AtomicBool

}

impl<Q, N> SharedDetails<Q, N>
{

    pub fn new(queue: Q, receivers_notifier: N) -> Self //, sender_count: &Arc<()>, receiver_count: &Arc<()>) -> Self
    {

        Self
        {

            queue,
            //sender_count: Arc::downgrade(sender_count),
            //receiver_count: Arc::downgrade(receiver_count),
            //active_receiver_count: AtomicUsize::new(0),
            receivers_notifier,
            //#[cfg(feature="count_waiting_senders_and_receivers")]
            //receivers_awiting_notification_count: AtomicUsize::new(0),
            //receivers_do_not_wait: AtomicBool::new(false)

        }

    }

    pub fn queue(&self) -> &Q
    {

        &self.queue

    }

    //active_receiver_count

    /*
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
    */

    //

    pub fn receivers_notifier(&self) -> &N
    {

        &self.receivers_notifier

    }

    /*
    pub fn receivers_do_not_wait(&self) -> bool
    {

        self.receivers_do_not_wait.load(Ordering::Acquire)

    }
    
    pub fn receivers_do_not_wait_t(&self)
    {

        self.receivers_do_not_wait.store(true, Ordering::Release);

    }

    #[cfg(feature="count_waiting_senders_and_receivers")]
    pub fn temp_inc_receivers_awaiting_notification_count<'a>(&'a self) -> ScopedIncrementer<'a>
    {

        ScopedIncrementer::new(&self.receivers_awiting_notification_count) 

    }
    */
    
}