use std::sync::atomic::{AtomicUsize, Ordering};

pub struct ScopedIncrementer<'a>
{

    inc_dec_value: &'a AtomicUsize

}

impl<'a> ScopedIncrementer<'a>
{

    pub fn new(inc_dec_value: &'a AtomicUsize) -> Self
    {

        //Increment the value

        inc_dec_value.fetch_add(1, Ordering::SeqCst);

        Self
        {

            inc_dec_value

        }

    }

    pub fn current_value(&self) -> usize
    {

        self.inc_dec_value.load(Ordering::Acquire)

    }

}

impl<'a> Drop for ScopedIncrementer<'a>
{

    fn drop(&mut self)
    {

        self.inc_dec_value.fetch_sub(1, Ordering::SeqCst);

    }

}