use std::task::Waker;

use paste::paste;

use accessorise::impl_get_val;

#[derive(Debug)]
pub struct QueuedWaker
{

    waker: Waker,
    id: usize

}

impl QueuedWaker
{

    pub fn new(waker: Waker, id: usize) -> Self
    {

        Self
        {

            waker,
            id

        }

    }

    impl_get_val!(id, usize);

    pub fn wake(self)
    {

        self.waker.wake();

    }
    
}
