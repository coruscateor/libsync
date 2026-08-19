use std::fmt::Debug;

use pastey::paste;

use accessorise::impl_ref_getter;

///
/// For containing the objects that are shared between the sender and the receiver parts of a channel.
/// 
pub struct ChannelSharedDetails<Q, N>
{

    message_queue: Q,
    notifier: N

}

impl<Q, N> ChannelSharedDetails<Q, N>
{

    pub fn new(message_queue: Q, notifier: N) -> Self
    {

        Self
        {

            message_queue,
            notifier

        }

    }

    impl_ref_getter!(message_queue, Q);

    impl_ref_getter!(notifier, N);

}

impl<Q, N> Debug for ChannelSharedDetails<Q, N>
    where Q: Debug,
          N: Debug
{

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ChannelSharedDetails").field("message_queue", &self.message_queue).field("notifier", &self.notifier).finish()
    }

}
