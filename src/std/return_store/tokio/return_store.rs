use std::sync::Arc;

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

            pub fn is_done(&self) -> bool;

            pub fn try_get(&self) -> Option<T>;

        }

    }

    pub async fn get(&self) -> Option<T>
    {

        if let Some(res) = self.base_state.try_get()
        {

            return Some(res);
            
        }

        self.base_state.notifier().notified().await;

        self.base_state.try_get()

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

    fn done(mut self, to_return: T)
    {

        self.base_state.set_done(to_return);

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

        }
        
    }

}
