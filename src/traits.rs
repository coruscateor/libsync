use crate::{BoundedSendResult, ReceiveResult, SendResult};

pub trait SenderTrait<T>
{

    fn send(&self, value: T) -> SendResult<T>;

}

pub trait BoundedSenderTrait<T>
{

    fn send(&self, value: T) -> BoundedSendResult<T>;

}

pub trait ReceiverTrait<T>
{

    fn recv(&self) -> ReceiveResult<T>;

}