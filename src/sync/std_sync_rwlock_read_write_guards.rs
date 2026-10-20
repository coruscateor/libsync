use std::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard, TryLockError};

pub fn read_get_rg<T>(rw_lock: &RwLock<T>) -> RwLockReadGuard<'_, T>
{

    let lock_result = rw_lock.read();

    match lock_result
    {

        Ok(mg) =>
        {

            mg

        }
        Err(err) =>
        {

            rw_lock.clear_poison();

            err.into_inner()

        }

    }

}

pub fn write_get_wg<T>(rw_lock: &RwLock<T>) -> RwLockWriteGuard<'_, T>
{

    let lock_result = rw_lock.write();

    match lock_result
    {

        Ok(wg) =>
        {

            wg

        }
        Err(err) =>
        {

            rw_lock.clear_poison();

            err.into_inner()

        }

    }

}

pub fn try_read_get_rg<T>(rw_lock: &RwLock<T>) -> Option<RwLockReadGuard<'_, T>>
{

    let lock_result = rw_lock.try_read();

    match lock_result
    {

        Ok(rg) =>
        {

            Some(rg)

        }
        Err(err) =>
        {

            match err
            {

                TryLockError::Poisoned(poison_error) =>
                {

                    rw_lock.clear_poison();

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

pub fn try_write_get_wg<T>(rw_lock: &RwLock<T>) -> Option<RwLockWriteGuard<'_, T>>
{

    let lock_result = rw_lock.try_write();

    match lock_result
    {

        Ok(wg) =>
        {

            Some(wg)

        }
        Err(err) =>
        {

            match err
            {

                TryLockError::Poisoned(poison_error) =>
                {

                    rw_lock.clear_poison();

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