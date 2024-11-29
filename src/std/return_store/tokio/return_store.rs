use std::{sync::Arc, time::Duration};

//use rand::rngs::ThreadRng;

use tokio::{sync::{AcquireError, Notify}, time::error::Elapsed};

//use super::ReturnStoreState; //, BaseReturnStore, BaseReturner};

use crate::{results, std::return_store::{BaseReturnStore, BaseReturner, ReturnStoreState, Returns}};

use delegate::delegate;

use crate::tokio_helpers::SemaphoreController;

pub struct ReturnStore<T>
{

    base_state: BaseReturnStore<T, SemaphoreController> //Notify>

}

impl<T> ReturnStore<T>
{

    pub fn new() -> Self
    {

        Self
        {

            base_state: BaseReturnStore::new(SemaphoreController::new()) //Notify::new())

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

        self.base_state.notifier().forget_permit();

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

            pub fn is_done(&self) -> bool;

            pub fn is_invalid(&self) -> bool;

            pub fn is_valid(&self) -> bool;

            pub fn try_get(&self) -> Option<T>;

        }

    }

    pub async fn get(&self) -> Option<T>
    {

        //Loop because the Semaphore might already have a ticket

        loop
        {

            let (should_wait, item) = self.base_state.try_get_or_should_wait();

            if should_wait
            {
    
                let acquire_res = self.base_state.notifier().acquire().await; //.notified().await;
    
                if let Ok(res) = acquire_res
                {

                    res.forget();
                    
                }

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

    pub async fn get_timeout(&self, duration: Duration) -> Result<Option<T>, Elapsed>
    {

        let (should_wait, item) = self.base_state.try_get_or_should_wait();

        if should_wait
        {

            let acquire_timeout_res = self.base_state.notifier().acquire_timeout(duration).await;

            match acquire_timeout_res
            {

                Ok(acquire_res) =>
                {

                    if let Ok(res) = acquire_res
                    {
    
                        res.forget();
                        
                    }

                }
                Err(err) =>
                {

                    return Err(err);

                }

            }

            Ok(self.base_state.try_get())

        }
        else
        {

            Ok(item)
            
        }

    }

}

pub struct Returner<T>
{

    base_state: BaseReturner<T, SemaphoreController> //Notify>

}

impl<T> Returns<T, SemaphoreController> for Returner<T> //Notify
{

    fn new(state: &Arc<ReturnStoreState<T, SemaphoreController>>, state_id: u32) -> Self //Notify
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

    fn done(mut self, to_return: T) -> Result<(), Option<T>>
    {

        let result = self.base_state.done(to_return);

        if result.is_ok()
        {

            self.base_state.notifier().add_permit();

        }

        result

        //self.base_state.notifier().notify_waiters();
        
         //.notify_one();

    }

    fn opt_done(mut self, to_return: Option<T>) -> Result<(), Option<T>>
    {

        let result = self.base_state.opt_done(to_return);

        if result.is_ok()
        {

            self.base_state.notifier().add_permit();

        }

        result

        //self.base_state.notifier().notify_waiters();

         //.notify_one();

    }

    fn done_none(mut self) -> Result<(), Option<T>>
    {

        let result = self.base_state.done_none();

        if result.is_ok()
        {

            self.base_state.notifier().add_permit();

        }

        result

        //self.base_state.notifier().notify_waiters();
        
         //.notify_one();

    }

}

impl<T> Drop for Returner<T>
{

    fn drop(&mut self)
    {

        //If the Returner hasn't already been invalidated, invalidate it and notify the other thread.

        if self.base_state.invalidate()
        {

            self.base_state.notifier().add_permit(); //.notify_one();
            
            //self.base_state.notifier().notify_waiters();

        }
        
    }

}
