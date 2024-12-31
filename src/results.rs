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
    where T: Display
{

    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result
    {

        match self
        {

            BoundedSendError::Full(val) => write!(f, "Full({val})"),
            BoundedSendError::NoReceivers(val) => write!(f, "NoSenders({val})")

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

#[derive(Debug)]
pub enum BoundedSendErrorType
{

    Full,
    NoReceivers,
    //ValueIrrecoverable

}

impl Display for BoundedSendErrorType
{

    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result
    {

        match self
        {

            BoundedSendErrorType::Full => write!(f, "Full"),
            BoundedSendErrorType::NoReceivers => write!(f, "NoReceivers")

        }
        
    }
    
}

#[derive(Debug)]
pub enum ReceiveError
{

    Empty,
    NoSenders

}

impl Display for ReceiveError
{

    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result
    {

        match self
        {

            ReceiveError::Empty => write!(f, "Empty"),
            ReceiveError::NoSenders => write!(f, "NoSenders")

        }
        
    }
    
}

pub type ReceiveResult<T> = Result<T, ReceiveError>;

//Timeouts

#[derive(Debug)]
pub enum TimeoutBoundedSendError<T>
{

    NotTimedOut(BoundedSendError<T>),
    TimedOut(T)

}

impl<T> Display for TimeoutBoundedSendError<T>
    where T: Display
{

    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result
    {

        match self
        {

            TimeoutBoundedSendError::NotTimedOut(val) => write!(f, "NotTimedOut({val})"),
            TimeoutBoundedSendError::TimedOut(val) => write!(f, "TimedOut({val})")

        }
         
    }

}

#[derive(Debug)]
pub enum TimeoutSendError<T>
{

    NotTimedOut(T),
    TimedOut(T)

}

impl<T> Display for TimeoutSendError<T>
    where T: Display
{

    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result
    {

        match self
        {

            TimeoutSendError::NotTimedOut(val) => write!(f, "NotTimedOut({val})"),
            TimeoutSendError::TimedOut(val) => write!(f, "TimedOut({val})")

        }
         
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

        match self
        {

            TimeoutReceiveError::NotTimedOut(val) => write!(f, "NotTimedOut({val})"),
            TimeoutReceiveError::TimedOut=> write!(f, "TimedOut")

        }
        
    }
    
}

