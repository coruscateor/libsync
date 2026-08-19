use std::sync::{Mutex, MutexGuard};

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