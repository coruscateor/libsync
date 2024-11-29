use std::sync::Arc;

use delegate::delegate;

use super::{BaseReturnStore, BaseReturner, ReturnStoreState, Returns};

//Maybe you just want to poll for a result and not wait.

pub struct PolledReturnStore<T>
{

    base_state: BaseReturnStore<T>

}

impl<T> PolledReturnStore<T>
{

    pub fn new() -> Self
    {

        Self
        {

            base_state: BaseReturnStore::new(())
        }

    }

    delegate!
    {

        to self.base_state
        {

            pub fn is_done(&self) -> bool;

            pub fn is_invalid(&self) -> bool;

            pub fn is_valid(&self) -> bool;

            pub fn try_get(&self) -> Option<T>;

            pub fn new_returner(&mut self) -> PolledReturner<T>;

        }

    }

}

pub struct PolledReturner<T>
{

    base_state: BaseReturner<T>

}

impl<T> Returns<T, ()> for PolledReturner<T>
{

    fn new(state: &Arc<ReturnStoreState<T>>, state_id: u32) -> Self
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

            fn is_valid(&self) -> bool;

            fn invalidate(&self) -> bool;

            fn done(mut self, to_return: T) -> Result<(), Option<T>>;

            fn opt_done(mut self, to_return: Option<T>) -> Result<(), Option<T>>;

            fn done_none(mut self) -> Result<(), Option<T>>;

        }

    }

}




