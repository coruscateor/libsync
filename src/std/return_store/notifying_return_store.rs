use std::{sync::{Arc, WaitTimeoutResult}, time::Duration};

use crate::std::Notifier;

static SHOULD_NEVER_BE_POISONED_MESSAGE: &str = "This Mutex got poisoned somehow. This should never happen.";


pub struct NotifyingReturnStoreState<T>
{

    item: Option<T>,
    state_id: u32

}

impl<T> NotifyingReturnStoreState<T>
{

    pub fn new() -> Self
    {

        Self
        {

            item: None,
            state_id: 1

        }

    }

    pub fn is_done(&self) -> bool
    {

        self.state_id == 1

    }

    pub fn is_invalid(&self) -> bool
    {

        self.state_id == 0

    }

    pub fn is_active(&self) -> bool
    {

        self.state_id > 1

    }

    pub fn state_id_is(&self, state_id: u32) -> bool
    {

        self.state_id == state_id

    }

}

pub struct NotifyingReturnStore<T>
{

    arc_notfifer: Arc<Notifier<NotifyingReturnStoreState<T>>>,
    state_id: u32

}

impl<T> NotifyingReturnStore<T>
{

    pub fn new() -> Self
    {

        Self
        {

            arc_notfifer: Arc::new(Notifier::new(NotifyingReturnStoreState::new())),
            state_id: 0

        }

    }

    pub fn try_get(&self) -> Option<T>
    {

        let mut guard = self.arc_notfifer.lock().expect(SHOULD_NEVER_BE_POISONED_MESSAGE);

        let taken_opt = guard.item.take();

       if taken_opt.is_some()
       {

            guard.state_id = 0;

       }

       taken_opt

    }

    //(Value, did wait?)

    pub fn wait<'a>(&'a self) -> (Option<T>, bool)
    {

        let mut value = None;

        let res_opt = self.arc_notfifer.wait_fn(|guard|
        {

            //Is the "returning value" session still valid?

            if guard.state_id_is(self.state_id)
            {

                true

            }
            else if guard.is_done()
            {

                value = guard.item.take();

                false
                
            }
            else
            {

                false

            }

        });

        if value.is_some()
        {

            (value, false)

        }
        else
        {

            if let Some(res) = res_opt
            {
    
                let mut guard = res.expect(SHOULD_NEVER_BE_POISONED_MESSAGE);
    
                (guard.item.take(), true)
                
            }
            else
            {
    
                (None, false)
                
            }

        }

        //let mut guard = self.arc_notfifer.lock().expect(SHOULD_NEVER_BE_POISONED_MESSAGE);

        //let taken_opt = guard.item.take();

    }

    pub fn wait_timeout<'a>(&'a self, dur: Duration) -> (Option<WaitTimeoutResult>, Option<T>, bool)
    {

        let mut value = None;

        let res_opt = self.arc_notfifer.wait_timeout_fn(dur, |guard|
        {

            //Is the "returning value" session still valid?

            if guard.state_id_is(self.state_id)
            {

                true

            }
            else if guard.is_done()
            {

                value = guard.item.take();

                false
                
            }
            else
            {

                false

            }

        });

        if value.is_some()
        {

            (None, value, false)

        }
        else
        {

            if let Some(res) = res_opt
            {

                let res = res.expect(SHOULD_NEVER_BE_POISONED_MESSAGE);

                let mut res = res.expect(SHOULD_NEVER_BE_POISONED_MESSAGE);

                (Some(res.1), res.0.item.take(), true)

                /*
                match guard
                {
                    Ok(res) => todo!(),
                    Err(err) => todo!(),
                }
                */

                //(guard..item.take(), true)
                
            }
            else
            {

                (None, None, false)
                
            }

        }

        //let mut guard = self.arc_notfifer.lock().expect(SHOULD_NEVER_BE_POISONED_MESSAGE);

        //let taken_opt = guard.item.take();

    }

    pub fn new_returner(&mut self) -> NotifyingReturner<T>
    {

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

        {

            let mut guard = self.arc_notfifer.lock().expect(SHOULD_NEVER_BE_POISONED_MESSAGE);

            guard.state_id = state_id;

            guard.item = None;

        }

        NotifyingReturner::new(&self.arc_notfifer, state_id)

    }

    pub fn invalidate(&mut self) -> bool
    {

        if self.state_id <= 1
        {

            return false;

        }

        let mut guard = self.arc_notfifer.lock().expect(SHOULD_NEVER_BE_POISONED_MESSAGE);

        guard.state_id = 0;

        return true;

    }

}

pub struct NotifyingReturner<T>
{

    arc_notfifer: Arc<Notifier<NotifyingReturnStoreState<T>>>,
    state_id: u32

}

impl<T> NotifyingReturner<T>
{

    pub fn new(arc_notfifer: &Arc<Notifier<NotifyingReturnStoreState<T>>>, state_id: u32) -> Self
    {

        Self
        {

            arc_notfifer: arc_notfifer.clone(),
            state_id

        }

    }

    pub fn is_valid(&self) -> bool
    {

        let guard = self.arc_notfifer.lock().expect(SHOULD_NEVER_BE_POISONED_MESSAGE);

        guard.state_id_is(self.state_id)

    }

    pub fn set_opt_done(self, to_return: Option<T>) -> Result<(), Option<T>> //bool
    {
        
        let result;

        //let is_valid;

        {

            let mut guard = self.arc_notfifer.lock().expect(SHOULD_NEVER_BE_POISONED_MESSAGE);

            //is_valid = guard.state_id_is(self.state_id);

            if guard.state_id_is(self.state_id) //is_valid
            {

                guard.item = to_return;

                guard.state_id = 1;

                result = Ok(())

            }
            else
            {

                result = Err(to_return);
                
            }

        }

        if result.is_ok() //is_valid
        {

            self.arc_notfifer.notify_one();

        }

        result

        //is_valid

    }

    pub fn set_done(self, to_return: T) -> Result<(), Option<T>> //bool
    {

        self.set_opt_done(Some(to_return))

    }

    pub fn set_done_none(self) -> Result<(), Option<T>> //bool
    {

        self.set_opt_done(None)

    }
    
}