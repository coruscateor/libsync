
pub type SendResult<T> = Result<(), T>;

pub enum BoundedSendError<T>
{

    Full(T),
    NoReceivers(T),
    //ValueIrrecoverable

}

impl<T> BoundedSendError<T>
{

    pub fn is_full(&self) -> bool
    {

        if let BoundedSendError::Full(_) = self
        {

            true

        }
        else
        {

            false   

        }

    }

    pub fn has_no_receivers(&self) -> bool
    {

        if let BoundedSendError::NoReceivers(_) = self
        {

            true

        }
        else
        {

            false   

        }

    }

    pub fn take(self) -> T
    {

        match self
        {
            BoundedSendError::Full(val) => val,
            BoundedSendError::NoReceivers(val) => val
        }

    }

}

/*
impl<T> Into<T> for BoundedSendError<T>
{

    fn into(self) -> T
    {

        match self
        {
            BoundedSendError::Full(val) => val,
            BoundedSendError::NoReceivers(val) => val
        }

    }

}
*/

pub type BoundedSendResult<T> = Result<(), BoundedSendError<T>>;

pub enum BoundedSendErrorType
{

    Full,
    NoReceivers,
    //ValueIrrecoverable

}

pub enum ReceiveError
{

    Empty,
    NoSenders

}

pub type ReceiveResult<T> = Result<T, ReceiveError>;

//Timeouts

pub enum TimeoutBoundedSendError<T>
{

    NotTimedOut(BoundedSendError<T>),
    TimedOut(T)

}


pub enum TimeoutSendError<T>
{

    NotTimedOut(T),
    TimedOut(T)

}

pub enum TimeoutReceiveError
{

    NotTimedOut(ReceiveError),
    TimedOut

}

