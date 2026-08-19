use std::task::Waker;


pub struct MultiShotSharedDetails<T>
{

    pub object: Option<T>,
    pub waker:Option<Waker>,
    pub should_be_awake: bool

}

impl<T> MultiShotSharedDetails<T>
{

    pub fn new() -> Self
    {

        Self
        {

            object: Default::default(),
            waker: Default::default(),
            should_be_awake: false

        }

    }

}