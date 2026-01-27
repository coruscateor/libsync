use std::error::Error;

use std::fmt::Display;

use std::future::Future;

use std::sync::{Mutex, MutexGuard};

use std::sync::atomic::{AtomicUsize, Ordering};

use std::collections::{HashMap, HashSet, VecDeque};

use std::task::{Poll, Waker};

use paste::paste;

use accessorise::impl_get_val;

use inc_dec::IntIncDecSelf;

pub struct WakerQueueInternals
{

    pub queue: VecDeque<QueuedWaker>,
    pub id: usize,
    //pub id_states: HashMap<usize, bool>
    pub active_ids: HashSet<usize>

}

impl WakerQueueInternals
{

    pub fn new() -> Self
    {

        Self
        {

            queue: VecDeque::new(),
            id: 0,
            //id_states: HashMap::new()
            active_ids: HashSet::new()

        }

    }

    pub fn with_capacity(capacity: usize) -> Self
    {

        Self
        {

            queue: VecDeque::with_capacity(capacity),
            id: 0,
            //id_states: HashMap::with_capacity(capacity)
            active_ids: HashSet::with_capacity(capacity)

        }

    }

}

pub struct WakerQueue
{

    waker_queue_internals: Mutex<Option<WakerQueueInternals>>

}

impl WakerQueue
{

    pub fn new() -> Self
    {

        Self
        {

            waker_queue_internals: Mutex::new(Some(WakerQueueInternals::new()))

        }

    }

    pub fn with_capacity(size: usize) -> Self
    {

        Self
        {

            waker_queue_internals: Mutex::new(Some(WakerQueueInternals::with_capacity(size)))

        }

    }

    fn clear_poison_get_mg(&self) -> MutexGuard<'_, Option<WakerQueueInternals>>
    {

        let lock_result = self.waker_queue_internals.lock();

        match lock_result
        {

            Ok(mg) =>
            {

                mg

            }
            Err(err) =>
            {

                self.waker_queue_internals.clear_poison();

                err.into_inner()

            }

        }

    }

    pub fn when_open_ref<T, F>(&self, func: F) -> Option<T>
        where F: Fn(&WakerQueueInternals) -> T
    {

        let mg = self.clear_poison_get_mg();

        match &*mg
        {

            Some(val) =>
            {

                Some(func(val))

            }
            None => None
            
        }

    }

    pub fn when_open_mut<T, F>(&self, func: F) -> Option<T>
        where F: Fn(&mut WakerQueueInternals) -> T
    {

        let mut mg = self.clear_poison_get_mg();

        match &mut *mg
        {

            Some(val) =>
            {

                Some(func(val))

            }
            None => None
            
        }

    }

    pub fn len(&self) -> Option<usize>
    {

        self.when_open_ref(|internals|
        {

            internals.queue.len()

        })

        /*
            let mg = self.clear_poison_get_mg();

            match &*mg
            {

                Some(val) =>
                {

                    Some(val.len())

                }
                None => None

            }
        */

    }

    pub fn capacity(&self) -> Option<usize>
    {

        self.when_open_ref(|internals| {

            internals.queue.capacity()

        })

    }

    pub fn wake_one(&self) -> bool
    {

        let waker;

        {

            let mut mg = self.clear_poison_get_mg();

            match &mut *mg
            {

                Some(val) =>
                {

                    if let Some(front_waker) = val.queue.pop_front()
                    {

                        val.active_ids.remove(&front_waker.id);

                        waker = front_waker;

                    }
                    else
                    {

                        return false;
                        
                    }

                    /*
                    if val.active_ids.remove(&waker.id)
                    {

                        waker.wake();

                        return true;

                    }
                    else
                    {

                        //Could panic here in debug maybe.

                        let mut inserted = false;

                        while !inserted
                        {

                            let new_id = val.id.wpp();

                            waker.id = new_id;

                            inserted = val.active_ids.insert(new_id);
                            
                        }

                        val.queue.push_back(waker);

                    }
                    */

                    //val.id_states.entry(key)

                }
                None =>
                {
                    
                    return false;

                }

            }

        }

        waker.wake();

        true

    }

    pub fn wake_me<'a>(&'a self) -> WakerQueueWakeMe<'a>
    {

        WakerQueueWakeMe::new(self)

        /*
        let mg = self.clear_poison_get_mg();

        match &*mg
        {

            Some(val) =>
            {

                let new_id = val.id.wpp();

                

            }
            None => None
            
        }
        */

    }

}

#[derive(Debug)]
pub struct WakerQueueWakeMeClosedError
{
}

impl WakerQueueWakeMeClosedError
{

    pub fn new() -> Self
    {

        Self
        {}

    }

    pub fn err() -> Result<(), Self>
    {

        Err(Self::new())

    }

}

impl Display for WakerQueueWakeMeClosedError
{

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        
        write!(f, "WakerQueue Closed")

    }

}

impl Error for WakerQueueWakeMeClosedError
{    
}

pub struct WakerQueueWakeMe<'a>
{

    waker_queue_ref: &'a WakerQueue,
    opt_waker_id: Option<usize>

}

impl<'a> WakerQueueWakeMe<'a>
{

    pub fn new(waker_queue_ref: &'a WakerQueue) -> Self //, waker_id: usize) -> Self
    {

        Self
        {

            waker_queue_ref,
            opt_waker_id: None //Some(waker_id)

        }

    }

}

impl Future for WakerQueueWakeMe<'_>
{

    type Output = Result<(), WakerQueueWakeMeClosedError>;

    fn poll(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> Poll<Self::Output>
    {

        match self.opt_waker_id
        {

            Some(id) =>
            {

                let mut mg = self.waker_queue_ref.clear_poison_get_mg();

                match &mut *mg
                {

                    Some(val) =>
                    {

                        if !val.active_ids.contains(&id)
                        {

                            //The task has been successfully awoken.

                            return Poll::Ready(Ok(()));

                        }

                    }
                    None =>
                    {

                        return Poll::Ready(WakerQueueWakeMeClosedError::err());

                    }

                }

            }
            None =>
            {

                //The task is going to "sleep". Update the WQI so it can be woken up later.

                let mut inserted = false;

                let waker = cx.waker().clone();

                let mut id = 0;

                //

                let mut mg = self.waker_queue_ref.clear_poison_get_mg();

                match &mut *mg
                {

                    Some(val) =>
                    {

                        while !inserted
                        {

                            //Find the next avalible id.

                            id = val.id.wpp();

                            inserted = val.active_ids.insert(id);
                            
                        }

                        let queued_waker = QueuedWaker::new(waker, id);

                        val.queue.push_back(queued_waker);

                    }
                    None =>
                    {

                        return Poll::Ready(WakerQueueWakeMeClosedError::err());

                    }

                }

                //

                //Store the id in the future.

                let self_mut = unsafe
                {
                    
                    self.get_unchecked_mut()

                };

                self_mut.opt_waker_id = Some(id);                     

            }

        }

        Poll::Pending

    }

}

impl Drop for WakerQueueWakeMe<'_>
{

    fn drop(&mut self)
    {

        // Make sure that the waker id gets removed.
        
        if let Some(id) = self.opt_waker_id
        {

            let mut mg = self.waker_queue_ref.clear_poison_get_mg();

            if let Some(wqi) = &mut *mg
            {

                wqi.active_ids.remove(&id);

            }
            
        }

    }

}
