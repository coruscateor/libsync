
pub type SendResult<T> = Result<(), T>;

pub enum BoundedSendError<T>
{

    Full(T),
    NoReceivers(T),
    //ValueIrrecoverable

}

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

    NotTimedOut((ReceiveError)),
    TimedOut

}

