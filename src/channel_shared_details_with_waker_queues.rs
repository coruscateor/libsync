use std::{fmt::Debug, sync::atomic::{AtomicBool, Ordering}};

///
/// For containing the objects that are shared between the sender and the receiver parts of a channel.
/// 
pub struct ChannelSharedDetailsWithEmptyQueue<MQ, EQ, CT = ()>
{

    pub message_queue: MQ,
    pub empty_queue: EQ,
    pub is_closed: CT

}

impl<MQ, EQ, CT> ChannelSharedDetailsWithEmptyQueue<MQ, EQ, CT>
{

    pub fn new(message_queue: MQ, empty_queue: EQ, is_closed: CT) -> Self
    {

        Self
        {

            message_queue,
            empty_queue,
            is_closed

        }

    }

    pub fn first_two(message_queue: MQ, empty_queue: EQ) -> Self
        where CT: Default
    {

        Self
        {

            message_queue,
            empty_queue,
            is_closed: Default::default()

        }

    }

}

impl<MQ, EQ, CT> Debug for ChannelSharedDetailsWithEmptyQueue<MQ, EQ, CT>
    where MQ: Debug,
          EQ: Debug,
          CT: Debug
{

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ChannelSharedDetailsWithEmptyQueue").field("message_queue", &self.message_queue).field("empty_queue", &self.empty_queue).field("is_closed", &self.is_closed).finish()
    }

}

impl<MQ, EQ> ChannelSharedDetailsWithEmptyQueue<MQ, EQ, AtomicBool>
{

    pub fn with_atomic_bool(message_queue: MQ, empty_queue: EQ) -> Self
    {

        Self
        {

            message_queue,
            empty_queue,
            is_closed: AtomicBool::new(false)

        }

    }

    pub fn is_closed(&self) -> bool
    {

        self.is_closed.load(Ordering::Acquire)

    }

    pub fn set_closed(&self)
    {

        self.is_closed.store(true, Ordering::Release);

    }

}

pub struct ChannelSharedDetailsWithBothQueues<MQ, WQ, CT = ()>
{

    pub message_queue: MQ,
    pub empty_queue: WQ,
    pub full_queue: WQ,
    pub is_closed: CT

}

impl<MQ, WQ, CT> ChannelSharedDetailsWithBothQueues<MQ, WQ, CT>
{

    pub fn new(message_queue: MQ, empty_queue: WQ, full_queue: WQ, is_closed: CT) -> Self
    {

        Self
        {

            message_queue,
            empty_queue,
            full_queue,
            is_closed

        }

    }

    pub fn first_three(message_queue: MQ, empty_queue: WQ, full_queue: WQ) -> Self
        where CT: Default
    {

        Self
        {

            message_queue,
            empty_queue,
            full_queue,
            is_closed: Default::default()

        }

    }

}

impl<MQ, WQ, CT> Debug for ChannelSharedDetailsWithBothQueues<MQ, WQ, CT>
    where MQ: Debug,
          WQ: Debug,
          CT: Debug
{

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ChannelSharedDetailsWithBothQueues").field("message_queue", &self.message_queue).field("empty_queue", &self.empty_queue).field("full_queue", &self.full_queue).field("is_closed", &self.is_closed).finish()
    }

}

impl<MQ, WQ> ChannelSharedDetailsWithBothQueues<MQ, WQ, AtomicBool>
{

    pub fn with_atomic_bool(message_queue: MQ, empty_queue: WQ, full_queue: WQ) -> Self
    {

        Self
        {

            message_queue,
            empty_queue,
            full_queue,
            is_closed: AtomicBool::new(false)

        }

    }

    pub fn is_closed(&self) -> bool
    {

        self.is_closed.load(Ordering::Acquire)

    }

    pub fn set_closed(&self)
    {

        self.is_closed.store(true, Ordering::Release);

    }

}
