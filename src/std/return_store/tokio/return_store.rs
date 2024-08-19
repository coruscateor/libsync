use std::sync::Arc;

//use rand::rngs::ThreadRng;

use tokio::sync::Notify;

//use super::ReturnStoreState; //, BaseReturnStore, BaseReturner};

use crate::std::return_store::{ReturnStoreState, BaseReturnStore, BaseReturner, Returns};

use delegate::delegate;

pub struct ReturnStore<T>
{

    base_state: BaseReturnStore<T, Notify>

}

impl<T> ReturnStore<T>
{

    pub fn new() -> Self
    {

        Self
        {

            base_state: BaseReturnStore::new(Notify::new())

        }

    }

    /*
    pub fn with_thread_rng(rng: &ThreadRng) -> Self
    {

        Self
        {

            base_state: BaseReturnStore::with_thread_rng(Notify::new(), rng)

        }

    }
    */

    pub fn new_returner(&mut self) -> Returner<T>
    {

        self.base_state.new_returner()

        //let returner = self.base_state.new_returner();

        //Make sure the Notifiy state has been reset.

        //self.base_state.notifier().notify_waiters();

        //returner

    }

    delegate!
    {

        to self.base_state
        {

            pub fn state_is_done(&self) -> bool;

            pub fn state_is_invalid(&self) -> bool;

            pub fn state_is_active(&self) -> bool;

            pub fn try_get(&self) -> Option<T>;

        }

    }

    pub async fn get(&self) -> Option<T>
    {

        //Loop because the Notifiy object might already have a ticket

        loop
        {

            let (should_wait, item) = self.base_state.try_get_or_should_wait();

            if should_wait
            {
    
                self.base_state.notifier().notified().await;
    
            }
            else
            {
    
                return item;
                
            }
            
        }

        /*
        if let Some(res) = self.base_state.try_get()
        {

            return Some(res);
            
        }

        self.base_state.notifier().notified().await;

        self.base_state.try_get()
        */

    }

}

pub struct Returner<T>
{

    base_state: BaseReturner<T, Notify>

}

impl<T> Returns<T, Notify> for Returner<T>
{

    fn new(state: &Arc<ReturnStoreState<T, Notify>>, state_id: u32) -> Self
    {
        
        Self
        {

            base_state: BaseReturner::new(state, state_id)

        }

    }

    delegate!
    {

        to self.base_state
        {

            fn invalidate(&self) -> bool;

            fn is_valid(&self) -> bool;

        }

    }

    //Call notify_waiters to avoid storing permits. - Bad...

    fn done(mut self, to_return: T)
    {

        self.base_state.set_done(to_return);

        //self.base_state.notifier().notify_waiters();
        
        self.base_state.notifier().notify_one();

    }

    fn opt_done(mut self, to_return: Option<T>)
    {

        self.base_state.set_opt_done(to_return);

        //self.base_state.notifier().notify_waiters();

        self.base_state.notifier().notify_one();

    }

    fn done_none(mut self)
    {

        self.base_state.set_done_none();

        //self.base_state.notifier().notify_waiters();
        
        self.base_state.notifier().notify_one();

    }

}

impl<T> Drop for Returner<T>
{

    fn drop(&mut self)
    {

        //If the Returner hasn't already been invalidated, invalidate it and notify the other thread.

        if self.base_state.invalidate()
        {

            self.base_state.notifier().notify_one();
            
            //self.base_state.notifier().notify_waiters();

        }
        
    }

}
