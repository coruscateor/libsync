#[cfg(feature="use_std_sync")]
use std::sync::MutexGuard;

use std::fmt::Debug;

pub enum TryLocked<T> //'a,
{

    Result(T),
    //MutexGuard(MutexGuard<'a, T>),
    WouldBlock
    
}

impl<'a, T> TryLocked<T> //'a,
{

    pub fn is_result(&self) -> bool
    {

        matches!(self, Self::Result(_))

    }

    /*
    pub fn is_mutex_guard(&self) -> bool
    {

        matches!(self, Self::MutexGuard(_))

    }
    */

    pub fn is_would_block(&self) -> bool
    {

        matches!(self, Self::WouldBlock)

    }

}

impl<T> Debug for TryLocked<T> //TryLocked<'a, T>
    where T: Debug
{

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Result(arg0) => f.debug_tuple("Result").field(arg0).finish(),
            Self::WouldBlock => write!(f, "WouldBlock"),
        }
    }

    /*
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MutexGuard(arg0) => f.debug_tuple("MutexGuard").field(arg0).finish(),
            Self::WouldBlock => write!(f, "WouldBlock"),
        }
    }
    */

}
