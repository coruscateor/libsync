use std::{error::Error, fmt::{Debug, Display, Formatter}};

///
/// The result of a send attempt on a channel.
/// 
pub type SendResult<T> = Result<(), T>;

pub struct SendError<T>
{

    value: T

}

impl<T> SendError<T>
{

    pub fn new(value: T) -> Self
    {

        Self
        {

            value

        }

    }

    pub fn take(self) -> T
    {

        self.value

    }

}

impl<T> Debug for SendError<T>
    where T: Debug
{

    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SendError").field("value", &self.value).finish()
    }

}

impl<T> Display for SendError<T>
{

    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result
    {
      
        f.write_str("A SendError has occured.")

    }

}

impl<T> Error for SendError<T>
   where T: Debug
{
}

impl<T> PartialEq for SendError<T>
   where T: PartialEq
{

    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }

}

impl<T> Eq for SendError<T>
   where T: Eq
{
}

///
/// Provides the reason for the error if a message cannot be sent on a bounded channel.
/// 
pub enum BoundedSendError<T>
{

    Full(T),
    Closed(T),
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

        if let BoundedSendError::Closed(_) = self
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
            BoundedSendError::Closed(val) => val
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
            BoundedSendError::Closed(val) => write!(f, "Closed({val})")

        }
        
    }

}

impl<T> Debug for BoundedSendError<T>
    where T: Debug
{

    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Full(arg0) => f.debug_tuple("Full").field(arg0).finish(),
            Self::Closed(arg0) => f.debug_tuple("Closed").field(arg0).finish(),
        }
    }

}

impl<T> PartialEq for BoundedSendError<T>
    where T: PartialEq
{

    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Full(l0), Self::Full(r0)) => l0 == r0,
            (Self::Closed(l0), Self::Closed(r0)) => l0 == r0,
            _ => false,
        }
    }

}

impl<T> Eq for BoundedSendError<T>
    where T: Eq
{
}

/*
impl<T> Into<T> for BoundedSendError<T>
{

    fn into(self) -> T
    {

        match self
        {
            BoundedSendError::Full(val) => val,
            BoundedSendError::Closed(val) => val
        }

    }

}
*/

///
/// The result of a send attempt on a bounded channel.
/// 
pub type BoundedSendResult<T> = Result<(), BoundedSendError<T>>;

///
/// Provides the reasons for why a value cannot be received from a channel.
/// 
#[derive(Debug, PartialEq, Eq)]
pub enum ReceiveError
{

    Empty,
    Closed

}

impl Display for ReceiveError
{

    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result
    {

        match self
        {

            ReceiveError::Empty => write!(f, "Empty"),
            ReceiveError::Closed => write!(f, "Closed")

        }
        
    }
    
}

///
/// The result of a receive attempt on a channel.
/// 
pub type ReceiveResult<T> = Result<T, ReceiveError>;

//Timeouts

///
/// Was there a BoundedSendError or a time-out?
/// 
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

///
/// Did the operation time-out or was there another issue?
/// 
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

///
/// Was there a ReceiveError or a time-out?
/// 
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

