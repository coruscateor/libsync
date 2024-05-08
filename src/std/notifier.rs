use std::{sync::{Condvar, LockResult, Mutex, MutexGuard, PoisonError, TryLockError, WaitTimeoutResult}, time::Duration};

use delegate::delegate;

///
/// Returned from a Notifier::wait call
/// 
pub struct NotifierWaitResult<'a, T>
{

    lr: LockResult<MutexGuard<'a, T>>

}

impl<'a, T> NotifierWaitResult<'a, T>
{

    pub fn new(lr: LockResult<MutexGuard<'a, T>>) -> Self
    {

        Self
        {

            lr

        }

    }

    pub fn is_ok(&self) -> bool
    {

        if let Ok(_) = self.lr
        {

            return true;

        }

        false

    }

    pub fn mutex_guard_ref(&self) -> &MutexGuard<'a, T>
    {

        match &self.lr
        {
            
            Ok(val) =>
            {

                return val;

            },
            Err(err) => 
            {

                err.get_ref()

            }

        }

    }

    pub fn mutex_guard_mut(&mut self) -> &mut MutexGuard<'a, T>
    {

        match &mut self.lr
        {

            Ok(val) =>
            {

                return val;

            },
            Err(err) => 
            {

                err.get_mut()

            }

        }
        
    }

}

///
/// Returned from a Notifier::wait_timeout call
/// 
pub struct NotifierWaitTimeoutResult<'a, T>
{

    lr: LockResult<(MutexGuard<'a, T>, WaitTimeoutResult)>

}

impl<'a, T> NotifierWaitTimeoutResult<'a, T>
{

    pub fn new(lr: LockResult<(MutexGuard<'a, T>, WaitTimeoutResult)>) -> Self
    {

        Self
        {

            lr

        }

    }

    pub fn is_ok(&self) -> bool
    {

        if let Ok(_) = self.lr
        {

            return true;

        }

        false

    }

    pub fn has_timed_out(&self) -> bool
    {

        match &self.lr
        {

            Ok(val) =>
            {

                return val.1.timed_out();

            },
            Err(err) => 
            {

                err.get_ref().1.timed_out()

            }

        }

    }

    ///
    /// Checks both the returned MutexGuard and WaitTimeoutResult
    /// 
    /// The returned tuple has the following parameters:
    /// 
    /// 0: Is the MutexGuard Ok? 1: Has timed out?
    /// 
    pub fn status(&self) -> (bool, bool)
    {

        match &self.lr
        {

            Ok(val) =>
            {

                return (true, val.1.timed_out());

            },
            Err(err) => 
            {

                (false, err.get_ref().1.timed_out())

            }

        }

    }

    ///
    /// Gets a MutexGuard, WaitTimeoutResult tuple reference.
    /// 
    pub fn mg_wto_ref(&'a self) -> &(MutexGuard<'_, T>, WaitTimeoutResult)
    {

        match &self.lr
        {

            Ok(val) =>
            {

                return val;

            },
            Err(err) => 
            {

                err.get_ref()

            }

        }

    }

    pub fn mutex_guard_ref(&self) -> &MutexGuard<'a, T>
    {

        match &self.lr
        {
            
            Ok(val) =>
            {

                return &val.0;

            },
            Err(err) => 
            {

                &err.get_ref().0

            }

        }

    }

    pub fn mutex_guard_mut(&'a mut self) -> &mut MutexGuard<'a, T>
    {

        match &mut self.lr
        {

            Ok(val) =>
            {

                return &mut val.0;

            },
            Err(err) => 
            {

                &mut err.get_mut().0

            }

        }
        
    }

}

//#[derive(Default)]

///
/// Block threads until notified (or time-out).
/// 
/// Comprised of a std::sync::Mutex and a Condvar.
/// 
pub struct Notifier<T = ()>
{

    mtx: Mutex<T>,
    cndvr: Condvar

}

impl<T> Notifier<T>
{

    pub fn new(value: T) -> Self
    {

        Self
        {

            mtx: Mutex::new(value),
            cndvr: Condvar::new()

        }

    }

    pub fn wait<'a>(&'a self) -> NotifierWaitResult<'a, T>
    {

        let mtx_lk_res = self.mtx.lock();

        let mtx_lk;

        match mtx_lk_res
        {

            Ok(res) =>
            {

                mtx_lk = res;

            },
            Err(_) => 
            {

                return NotifierWaitResult::<'a, T>::new(mtx_lk_res);

            }

        }

        NotifierWaitResult::new(self.cndvr.wait(mtx_lk))

    }

    pub fn wait_timeout<'a>(&'a self, dur: Duration) -> Result<NotifierWaitTimeoutResult<'a, T>, PoisonError<MutexGuard<'_, T>>>
    {

        let mtx_lk_res = self.mtx.lock();

        let mtx_lk;

        match mtx_lk_res
        {

            Ok(res) =>
            {

                mtx_lk = res;

            },
            Err(err) => 
            {

                return Err(err);

            }

        }

        Ok(NotifierWaitTimeoutResult::new(self.cndvr.wait_timeout(mtx_lk, dur)))

    }

    pub fn try_wait<'a>(&'a self) -> Result<NotifierWaitResult<'a, T>, TryLockError<MutexGuard<'_, T>>>
    {

        let mtx_lk_res = self.mtx.try_lock();

        let mtx_lk;

        match mtx_lk_res
        {

            Ok(res) =>
            {

                mtx_lk = res;

            },
            Err(err) => 
            {

                return Err(err);

            }

        }

        Ok(NotifierWaitResult::new(self.cndvr.wait(mtx_lk)))

    }

    pub fn try_wait_timeout<'a>(&'a self, dur: Duration) -> Result<NotifierWaitTimeoutResult<'a, T>, TryLockError<MutexGuard<'_, T>>>
    {

        let mtx_lk_res = self.mtx.try_lock();

        let mtx_lk;

        match mtx_lk_res
        {

            Ok(res) =>
            {

                mtx_lk = res;

            },
            Err(err) => 
            {

                return Err(err);

            }

        }

        Ok(NotifierWaitTimeoutResult::new(self.cndvr.wait_timeout(mtx_lk, dur)))

    }

    delegate! {
        to self.cndvr {

            pub fn notify_one(&self);

            pub fn notify_all(&self);

        }
    }

    delegate! {
        to self.mtx {

            pub fn is_poisoned(&self);

            pub fn clear_poison(&self);

        }
    }

    pub fn try_set_notify_one(&self, value: T) -> bool
    {

        let mut successful = false;

        match self.mtx.lock()
        {

            Ok(mut res) =>
            {

                *res = value;

                successful = true;

            },
            Err(_err) => 
            {

               // false

            }

        }

        if successful
        {

            self.cndvr.notify_one();

        }

        successful

    }

    pub fn try_set_notify_all(&self, value: T) -> bool
    {

        let mut successful = false;

        match self.mtx.lock()
        {

            Ok(mut res) =>
            {

                *res = value;

                successful = true;

                //self.cndvr.notify_all();

                //true

            },
            Err(_err) =>
            {

                //false

            }

        }

        if successful
        {

            self.cndvr.notify_all();

        }

        successful

    }

    pub fn must_set_notify_one(&self, value: T)
    {

        {

            let mut res = self.mtx.lock().expect("Error: Failed to unlock Mutex.");

            *res = value;

        }

        self.cndvr.notify_one();

    }

    pub fn must_set_notify_all(&self, value: T)
    {

        {

            let mut res = self.mtx.lock().expect("Error: Failed to unlock Mutex.");

            *res = value;

        }

        self.cndvr.notify_all();

    }

}

impl<T> Default for Notifier<T>
    where T: Default
{

    fn default() -> Self
    {

        Self
        {

            mtx: Mutex::new(T::default()),
            cndvr: Condvar::default()

        }

    }

}


