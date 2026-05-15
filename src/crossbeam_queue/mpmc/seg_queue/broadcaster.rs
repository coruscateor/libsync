use std::collections::{HashMap, TryReserveError};

use delegate::delegate;

use inc_dec::IncDecSelf;

use super::Sender;

///
/// Broadcast messages to multiple channels.
/// 
pub struct Broadcaster<T>
    where T: Clone
{

    senders: HashMap<usize, Sender<T>>

}

impl<T> Broadcaster<T>
    where T: Clone
{

    pub fn new() -> Self
    {

        Self
        {

            senders: HashMap::new()

        }

    }

    pub fn with_capacity(capacity: usize) -> Self
    {

        Self
        {

            senders: HashMap::with_capacity(capacity)

        }

    }

    delegate!
    {

        to self.senders
        {

            pub fn len(&self) -> usize;

            pub fn capacity(&self) -> usize;

            pub fn reserve(&mut self, additional: usize);

            pub fn try_reserve(&mut self, additional: usize) -> Result<(), TryReserveError>;

            pub fn shrink_to_fit(&mut self);

            pub fn shrink_to(&mut self, min_capacity: usize);

            pub fn clear(&mut self);

        }

    }

    pub fn contains(&self, sender: &Sender<T>) -> bool
    {

        self.senders.contains_key(&sender.shared_details_ptr_addr())

    }

    pub fn insert(&mut self, sender: Sender<T>) -> Option<Sender<T>>
    {

        let addr = sender.shared_details_ptr_addr();

        self.senders.insert(addr, sender)

    }

    pub fn remove(&mut self, sender: Sender<T>) -> Option<Sender<T>>
    {

        self.senders.remove(&sender.shared_details_ptr_addr())

    }

    ///
    /// Sends a value to one or more channels and returns the number of channels that were removed due to errors.
    /// 
    pub fn send(&mut self, value: T) -> usize
    {

        let len = self.senders.len();

        if len == 1
        {

            let mut key_to_remove = 0;

            let mut return_val = 0;

            for (addr, sender) in self.senders.iter()
            {

                if let Err(_) = sender.send(value)
                {

                    key_to_remove = *addr;

                    return_val = 1;

                }

                break;

            }

            if return_val == 1
            {

                let _ = self.senders.remove(&key_to_remove);

            }

            return_val

        }
        else if len > 1
        {

            let mut keys_to_remove = Vec::new();

            let last_len = len - 1;

            let mut index = 0;

            for (addr, sender) in self.senders.iter()
            {

                if index < last_len
                {

                    let cloned_value = value.clone();

                    if let Err(_) = sender.send(cloned_value)
                    {

                        keys_to_remove.push(*addr);

                    }

                    index.pp();

                }
                else
                {

                    if let Err(_) = sender.send(value)
                    {

                        keys_to_remove.push(*addr);

                    }

                    break;
                    
                }

            }

            for key_to_remove in keys_to_remove.iter()
            {

                let _ = self.senders.remove(key_to_remove);

            }

            keys_to_remove.len()

        }
        else
        {

            //No senders or keys to remove.
            
            0

        }

    }

}