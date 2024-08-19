use std::cell::UnsafeCell;

use std::sync::Arc;

use std::sync::atomic::{AtomicU32, Ordering};

//use rand::prelude::*;

pub struct ReturnStoreState<T, N = ()>
{

    storage: UnsafeCell<Option<T>>,
    storage_state: AtomicU32,
    notifier: N

}

impl<T, N> ReturnStoreState<T, N>
{

    pub fn new(notifier: N) -> Self
    {

        Self
        {

            storage: UnsafeCell::new(None),
            storage_state: AtomicU32::new(0),
            notifier

        }

    }

}

pub struct BaseReturnStore<T, N = ()>
{

    state: Arc<ReturnStoreState<T, N>>,
    //rng: ThreadRng
    state_id: u32

}

impl<T, N> BaseReturnStore<T, N>
{

    pub fn new(notifier: N) -> Self
    {

        Self
        {

            state: Arc::new(ReturnStoreState::new(notifier)),
            //rng: rand::thread_rng()
            state_id: 1

        }

    }

    /*
    pub fn with_thread_rng(notifier: N, rng: &ThreadRng) -> Self
    {

        Self
        {

            state: Arc::new(ReturnStoreState::new(notifier)),
            rng: rng.clone()

        }

    }
    */

    pub fn state_is_done(&self) -> bool
    {

        self.state.storage_state.load(Ordering::Acquire) == 1

    }

    pub fn state_is_invalid(&self) -> bool
    {

        self.state.storage_state.load(Ordering::Acquire) == 0

    }

    pub fn state_is_active(&self) -> bool
    {

        self.state.storage_state.load(Ordering::Acquire) == self.state_id

    }

    //0 - Invalid/Cancelled

    //1 - Operation complete

    //2..u32::MAX - result pending

    pub fn new_returner<R>(&mut self) -> R
        where R: Returns<T, N>
    {

        //let state_id = self.rng.gen_range(2..u32::MAX);

        let state_id = self.state_id;

        let opt_state_id = state_id.checked_add(1);

        match opt_state_id
        {

            Some(res) =>
            {

                self.state_id = res;

            }
            None =>
            {

                self.state_id = 2;

            }

        }

        self.state.storage_state.store(self.state_id, Ordering::Release);

        R::new(&self.state, state_id)

    }

    pub fn try_get(&self) -> Option<T>
    {

        if !self.state_is_active()
        {

            let res = unsafe { self.state.storage.get().as_mut() };

            if let Some(contained) = res
            {

                return contained.take();

            }

        }

        None

    }

    pub fn try_get_or_should_wait(&self) -> (bool, Option<T>)
    {

        if self.state_is_active()
        {

            return (true, None);

        }

        let res = unsafe { self.state.storage.get().as_mut() };

        if let Some(contained) = res
        {

            return (false, contained.take());

        }

        (false, None)

    }

    pub fn notifier(&self) -> &N
    {

        &self.state.notifier

    }

}

pub struct BaseReturner<T, N = ()>
{

    state: Arc<ReturnStoreState<T, N>>,
    state_id: u32 

}

impl<T, N> BaseReturner<T, N>
{

    pub fn new(state: &Arc<ReturnStoreState<T, N>>, state_id: u32) -> Self
    {

        Self
        {

            state: state.clone(),
            state_id

        }

    }

    pub fn is_valid(&self) -> bool
    {

        self.state.storage_state.load(Ordering::Acquire) == self.state_id

    }

    //Called just prior to being dropped.

    pub fn set_done(&mut self, to_return: T)
    {

       self.set_opt_done(Some(to_return));

    }

    pub fn set_opt_done(&mut self, to_return: Option<T>)
    {

        if self.is_valid()
        {

            let ptr = self.state.storage.get();

            unsafe
            {
                
                *ptr = to_return;
            
            }

            let _ = self.state.storage_state.compare_exchange(self.state_id, 1, Ordering::Acquire, Ordering::Relaxed);

            self.state_id = 1;

        }

    }

    pub fn set_done_none(&mut self)
    {

        self.set_opt_done(None);

    }

    pub fn notifier(&self) -> &N
    {

        &self.state.notifier

    }

    //Called when being dropped.

    pub fn invalidate(&self) -> bool
    {

        if self.state_id == 1
        {

            return false;

        }

        match self.state.storage_state.compare_exchange(self.state_id, 0, Ordering::Acquire, Ordering::Relaxed)
        {

            Ok(_res) => true,
            Err(_err) => false

        }

    }

}

pub trait Returns<T, N>
{

    fn new(state: &Arc<ReturnStoreState<T, N>>, state_id: u32) -> Self;

    fn is_valid(&self) -> bool;

    fn invalidate(&self) -> bool;

    fn done(self, to_return: T);

    fn opt_done(self, to_return: Option<T>);

    fn done_none(self);

}


