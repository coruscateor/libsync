use std::fmt::Debug;

///
/// For containing the objects that are shared between the sender and the receiver parts of a channel.
/// 
pub struct ChannelSharedDetailsWithEmptyQueue<MQ, EQ>
{

    pub message_queue: MQ,
    pub empty_queue: EQ

}

impl<MQ, EQ> ChannelSharedDetailsWithEmptyQueue<MQ, EQ>
{

    pub fn new(message_queue: MQ, empty_queue: EQ) -> Self
    {

        Self
        {

            message_queue,
            empty_queue

        }

    }

}

impl<MQ, EQ> Debug for ChannelSharedDetailsWithEmptyQueue<MQ, EQ>
    where MQ: Debug,
          EQ: Debug
{

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ChannelSharedDetailsWithEmptyQueue").field("message_queue", &self.message_queue).field("empty_queue", &self.empty_queue).finish()
    }

}

pub struct ChannelSharedDetailsWithBothQueues<MQ, WQ>
{

    pub message_queue: MQ,
    pub empty_queue: WQ,
    pub full_queue: WQ

}

impl<MQ, WQ> ChannelSharedDetailsWithBothQueues<MQ, WQ>
{

    pub fn new(message_queue: MQ, empty_queue: WQ, full_queue: WQ) -> Self
    {

        Self
        {

            message_queue,
            empty_queue,
            full_queue

        }

    }

}

impl<MQ, WQ> Debug for ChannelSharedDetailsWithBothQueues<MQ, WQ>
    where MQ: Debug,
          WQ: Debug
{

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ChannelSharedDetailsWithBothQueues").field("message_queue", &self.message_queue).field("empty_queue", &self.empty_queue).field("full_queue", &self.full_queue).finish()
    }

}
