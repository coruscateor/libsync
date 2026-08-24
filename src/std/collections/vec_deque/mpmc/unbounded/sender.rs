use std::sync::{Arc, Weak};

#[cfg(feature="use_std_sync")]
use crate::get_mg;

use super::ChannelSharedDetails;

use crate::{BoundedSendError, PreferredMutexType};

use delegate::delegate;

use std::fmt::Debug;

pub struct Sender<T>
{

    shared_details: Arc<PreferredMutexType<ChannelSharedDetails<T>>>,
    senders_count: Arc<()>,
    receivers_count: Weak<()>

}

impl<T> Sender<T>
{

    pub fn new(shared_details: Arc<PreferredMutexType<ChannelSharedDetails<T>>>, senders_count: Arc<()>, receivers_count: Weak<()>) -> Self
    {

        Self
        {

            shared_details,
            senders_count,
            receivers_count
            
        }

    }

    pub fn send(&self, value: T) -> Result<(), T>
    {

        let opt_waker;

        {

            #[cfg(feature="use_std_sync")]
            let mut mg = get_mg(&self.shared_details);

            #[cfg(any(feature="use_parking_lot_sync", feature="use_parking_lot_fair_sync"))]
            let mut mg = self.shared_details.lock();

            if mg.is_closed
            {

                return Err(value);

            }

            mg.message_queue.push_back(value);

            opt_waker = mg.waker_queue.pop_front();

            if let Some(waker) = opt_waker
            {

                let opt_entry = mg.active_ids.get_mut(&waker.id());
                
                if let Some(entry) = opt_entry
                {

                    *entry = true;

                }

                waker.wake();

            }

        }

        /*
        if let Some(waker) = opt_waker
        {

            waker.wake();
            
        }
        */

        Ok(())

    }

    pub fn is_empty(&self) -> bool
    {

        #[cfg(feature="use_std_sync")]
        let mg = get_mg(&self.shared_details);

        #[cfg(any(feature="use_parking_lot_sync", feature="use_parking_lot_fair_sync"))]
        let mg = self.shared_details.lock();

        mg.message_queue.is_empty()

    }

    pub fn len(&self) -> usize
    {

        #[cfg(feature="use_std_sync")]
        let mg = get_mg(&self.shared_details);

        #[cfg(any(feature="use_parking_lot_sync", feature="use_parking_lot_fair_sync"))]
        let mg = self.shared_details.lock();

        mg.message_queue.len()

    }

    ///
    /// The total number of Sender instances.
    /// 
    pub fn strong_count(&self) -> usize
    {

        Arc::strong_count(&self.senders_count)

    }

    ///
    /// The total number of potential Sender instances.
    /// 
    pub fn weak_count(&self) -> usize
    {

        Arc::weak_count(&self.senders_count)
        
    }

    delegate!
    {

        to self.receivers_count
        {

            ///
            /// The total number of Receiver instances.
            /// 
            #[call(strong_count)]
            pub fn receivers_strong_count(&self) -> usize;

            ///
            /// The total number of potential Receiver instances.
            /// 
            #[call(weak_count)]
            pub fn receivers_weak_count(&self) -> usize;

        }

    }

    pub fn is_closed(&self) -> bool
    {

        #[cfg(feature="use_std_sync")]
        let mg = get_mg(&self.shared_details);

        #[cfg(any(feature="use_parking_lot_sync", feature="use_parking_lot_fair_sync"))]
        let mg = self.shared_details.lock();

        mg.is_closed

    }

    //pub fn downgrade(&self) -> WeakSender<T>

    pub fn same_channel(&self, other: &Self) -> bool
    {

        Arc::ptr_eq(&self.shared_details, &other.shared_details) 

    }

    pub fn shared_details_ptr_addr(&self) -> usize
    {

        Arc::as_ptr(&self.shared_details).addr()

    }

    /*
    pub fn same_channel_receiver(&self, other: &super::Receiver<T>) -> bool
    {

        self.shared_details_ptr_addr() == other.shared_details_ptr_addr()

    }
    */

}

impl<T> Clone for Sender<T>
{

    fn clone(&self) -> Self
    {

        Self
        { 
            
            shared_details: self.shared_details.clone(),
            senders_count: self.senders_count.clone(),
            receivers_count: self.receivers_count.clone()
        
        }

    }

}

impl<T> Debug for Sender<T>
    where T: Debug
{

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Sender").field("shared_details", &self.shared_details).field("senders_count", &self.senders_count).field("receivers_count", &self.receivers_count).finish()
    }

}

impl<T> Drop for Sender<T>
{

    fn drop(&mut self)
    {

        if self.strong_count() == 1
        {

            #[cfg(feature="use_std_sync")]
            let mut mg = get_mg(&self.shared_details);

            #[cfg(any(feature="use_parking_lot_sync", feature="use_parking_lot_fair_sync"))]
            let mut mg = self.shared_details.lock();

            mg.is_closed = true;

            //Engage free-for-all mode.

            for waker in mg.waker_queue.drain(..)
            {

                waker.wake();

            }

        }
    
    }

}
