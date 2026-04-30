use std::{fmt::{Display, write}, ops::Deref, sync::Arc};

#[cfg(feature="use_std_sync")]
use std::sync::{ RwLockReadGuard, RwLockWriteGuard, TryLockError };

#[cfg(any(feature="use_parking_lot_sync", feature="use_parking_lot_fair_sync"))]
use parking_lot::{ RwLockReadGuard, RwLockWriteGuard };

use std::fmt::Debug;

pub struct Reader<'a, T>
{

    read_guard: RwLockReadGuard<'a, T>

}

impl<'a, T> Reader<'a, T>
{

    pub fn new(read_guard: RwLockReadGuard<'a, T>) -> Self
    {

        Self
        {

            read_guard

        }

    }

}

impl<'a, T> Deref for Reader<'a, T>
{

    type Target = T;

    fn deref(&self) -> &Self::Target
    {

        &*self.read_guard
        
    }

}

impl<'a, T> Debug for Reader<'a, T>
    where T: Debug
{

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Reader").field("read_guard", &self.read_guard).finish()
    }

}


impl<'a, T> Display for Reader<'a, T>
    where T: Display
{

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {

        write!(f, "{}", &self.read_guard)

    }

}
