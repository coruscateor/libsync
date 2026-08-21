use std::task::Waker;


pub struct MultiShotSharedDetails<T>
{

    pub opt_object: Option<T>,
    pub opt_waker:Option<Waker>,
    //pub should_be_awake: bool
    pub session_number: u32

}

impl<T> MultiShotSharedDetails<T>
{

    pub fn new() -> Self
    {

        Self
        {

            opt_object: Default::default(),
            opt_waker: Default::default(),
            //should_be_awake: false
            session_number: Default::default()

        }

    }

}