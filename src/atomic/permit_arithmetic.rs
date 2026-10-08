use std::sync::atomic::{AtomicUsize, Ordering};


pub fn add_permits(permits_ref: &AtomicUsize, mut current_permits: usize, mut permits_to_add: usize) -> usize
{

    loop
    {

        /*
        if permits == 0
        {

            return Some(0);

        }
        */

        if let Some(new_permits) = current_permits.checked_add(permits_to_add)
        {

            let res = permits_ref.compare_exchange(current_permits, new_permits, Ordering::SeqCst, Ordering::SeqCst);

            match res
            {

                Ok(_num) =>
                {

                    return permits_to_add;

                }
                Err(num) =>
                {

                    current_permits = num;

                }

            }

        }
        else
        {

            permits_to_add = usize::MAX - current_permits;
            
        }
        
    }
    
}

pub fn remove_permits(permits_ref: &AtomicUsize, mut current_permits: usize, mut permits_to_remove: usize) -> usize
{

    loop
    {

        /*
        if permits == 0
        {

            return Some(0);

        }
        */

        if let Some(new_permits) = current_permits.checked_sub(permits_to_remove)
        {

            let res = permits_ref.compare_exchange(current_permits, new_permits, Ordering::SeqCst, Ordering::SeqCst);

            match res
            {

                Ok(_num) =>
                {

                    return permits_to_remove;

                }
                Err(num) =>
                {

                    current_permits = num;

                }

            }

        }
        else
        {

            permits_to_remove = current_permits;
            
        }
        
    }

}