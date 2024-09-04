use std::fmt::{Display, Formatter};

pub type SendResult<T> = Result<(), T>;

#[derive(Debug)]
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

impl<T> Display for BoundedSendError<T>
{

    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result
    {

        write!(f, "{}", self)
        
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

#[derive(Debug)]
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

impl Display for ReceiveError
{

    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result
    {

        write!(f, "{}", self)
        
    }
    
}

#[derive(Debug)]
pub enum TimeoutSendError<T>
{

    NotTimedOut(T),
    TimedOut(T)

}

impl<T> Display for TimeoutSendError<T>
{

    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result
    {

        write!(f, "{}", self)
        
    }

}

#[derive(Debug)]
pub enum TimeoutReceiveError
{

    NotTimedOut(ReceiveError),
    TimedOut

}

impl Display for TimeoutReceiveError
{

    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result
    {

        write!(f, "{}", self)
        
    }
    
}

