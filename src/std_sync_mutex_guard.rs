use std::sync::{Mutex, MutexGuard, TryLockError, TryLockResult};

pub fn get_mg<T>(mutex: &Mutex<T>) -> MutexGuard<'_, T> //>
{

    let lock_result = mutex.lock();

    match lock_result
    {

        Ok(mg) =>
        {

            mg

        }
        Err(err) =>
        {

            mutex.clear_poison();

            err.into_inner()

        }

    }

}

pub fn try_get_mg<T>(mutex: &Mutex<T>) -> Option<MutexGuard<'_, T>>
{

    let lock_result = mutex.try_lock();

    match lock_result
    {

        Ok(mg) =>
        {

            Some(mg)

        }
        Err(err) =>
        {

            match err
            {

                TryLockError::Poisoned(poison_error) =>
                {

                    mutex.clear_poison();

                    Some(poison_error.into_inner())

                }
                TryLockError::WouldBlock =>
                {

                    None

                }

            }

        }

    }

}