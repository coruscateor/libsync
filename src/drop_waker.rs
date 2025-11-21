use std::task::Waker;

pub struct DropWaker
{

    opt_waker: Option<Waker>

}

impl DropWaker
{

    pub fn new(waker: Waker) -> Self
    {

        Self
        {

            opt_waker: Some(waker)

        }

    }

    pub fn has_waker(&self) -> bool
    {

        self.opt_waker.is_some()

    }

}

impl Drop for DropWaker
{

    fn drop(&mut self)
    {

        if let Some(waker) = self.opt_waker.take()
        {

            waker.wake();

        }
       
    }

}
