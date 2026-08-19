use std::task::Waker;

use pastey::paste;

use accessorise::impl_val_getter;

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

    impl_val_getter!(id, usize);

    pub fn wake(self)
    {

        self.waker.wake();

    }
    
}
