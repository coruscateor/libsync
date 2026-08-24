use std::sync::{Arc, Weak};

#[cfg(feature="use_std_sync")]
use crate::get_mg;

use super::ChannelSharedDetails;

use crate::{BoundedSendError, PreferredMutexType};

use delegate::delegate;

use std::fmt::Debug;

pub struct Receiver<T>
{

    shared_details: Arc<PreferredMutexType<ChannelSharedDetails<T>>>,
    senders_count: Weak<()>,
    receivers_count: Arc<()>

}

impl<T> Receiver<T>
{


}