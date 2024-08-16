use std::cell::UnsafeCell;

use std::sync::Arc;

use std::sync::atomic::{AtomicU32, Ordering};

use rand::prelude::*;

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

    /*
    pub fn notifier(&self) -> &N
    {

        &self.notifier

    }
    */

}

pub struct BaseReturnStore<T, N = ()>
{

    state: Arc<ReturnStoreState<T, N>>,
    rng: ThreadRng

}

impl<T, N> BaseReturnStore<T, N>
{

    pub fn new(notifier: N) -> Self
    {

        Self
        {

            state: Arc::new(ReturnStoreState::new(notifier)),
            rng: rand::thread_rng()

        }

    }

    pub fn is_done(&self) -> bool
    {

        self.state.storage_state.load(Ordering::Acquire) == 1

    }

    pub fn new_returner<R>(&mut self) -> R //impl Returns<T, N> //Returner<T>
        where R: Returns<T, N>
    {

        let state_id = self.rng.gen_range(2..u32::MAX);

        self.state.storage_state.store(state_id, Ordering::Release);

        //Returner::new(&self.state, state_id)

        R::new(&self.state, state_id)

        //Returns::new(&self.state, state_id)

    }

    pub fn try_get(&self) -> Option<T>
    {

        if self.is_done()
        {

            let res = unsafe { self.state.storage.get().as_mut() };

            if let Some(contained) = res
            {

                return contained.take();

            }

        }

        None

    }

    pub fn notifier(&self) -> &N
    {

        &self.state.notifier

    }

    /*
    delegate!
    {

        to self.state
        {



        }

    }
    */

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

    pub fn set_done(&mut self, to_return: T)
    {

        if self.is_valid()
        {

            let ptr = self.state.storage.get();

            unsafe
            {
                
                *ptr = Some(to_return);
            
            }

            let _ = self.state.storage_state.compare_exchange(self.state_id, 1, Ordering::Acquire, Ordering::Relaxed);

        }

    }

    pub fn notifier(&self) -> &N
    {

        &self.state.notifier

    }

    pub fn invalidate(&self) -> bool
    {

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

}


