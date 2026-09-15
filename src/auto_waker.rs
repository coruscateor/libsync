use std::task::Waker;

#[derive(Debug, Default)]
pub struct AutoWaker
{

    opt_waker: Option<Waker>

}

impl AutoWaker
{

    pub fn new(waker: Waker) -> Self
    {

        Self
        {

            opt_waker: Some(waker)

        }

    }

    pub fn empty() -> Self
    {

        Self
        {
            
            opt_waker: None
        
        }

    }

    pub fn has_awoken(&self) -> bool
    {

        self.opt_waker.is_none()

    }

    pub fn has_not_awoken(&self) -> bool
    {

        self.opt_waker.is_some()

    }

    pub fn try_swap(&mut self, waker: Waker) -> Option<Waker>
    {

        let taken = self.opt_waker.take();

        self.opt_waker = Some(waker);

        taken

    }

    pub fn wake(&mut self) -> bool
    {

        if let Some(waker) = self.opt_waker.take()
        {

            waker.wake();

            true

        }
        else
        {

            false
            
        }

    }
    
}

impl Drop for AutoWaker
{

    fn drop(&mut self)
    {

        self.wake();

    }

}
