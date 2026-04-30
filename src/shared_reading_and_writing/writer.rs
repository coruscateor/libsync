use std::{fmt::{Display, write}, ops::{Deref, DerefMut}, sync::Arc};

#[cfg(feature="use_std_sync")]
use std::sync::{ RwLockReadGuard, RwLockWriteGuard, TryLockError };

#[cfg(any(feature="use_parking_lot_sync", feature="use_parking_lot_fair_sync"))]
use parking_lot::{ RwLockReadGuard, RwLockWriteGuard };

use std::fmt::Debug;

pub struct Writer<'a, T>
{

    write_guard: RwLockWriteGuard<'a, T>

}

impl<'a, T> Writer<'a, T>
{

    pub fn new(write_guard: RwLockWriteGuard<'a, T>) -> Self
    {

        Self
        {

            write_guard

        }

    }

}

impl<'a, T> Deref for Writer<'a, T>
{

    type Target = T;

    fn deref(&self) -> &Self::Target
    {

        &*self.write_guard
        
    }

}

impl<'a, T> DerefMut for Writer<'a, T>
{

    fn deref_mut(&mut self) -> &mut Self::Target
    {
        
        &mut *self.write_guard
        
    }

}

impl<'a, T> Debug for Writer<'a, T>
    where T: Debug
{

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Writer").field("write_guard", &self.write_guard).finish()
    }

}


impl<'a, T> Display for Writer<'a, T>
    where T: Display
{

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {

        write!(f, "{}", &self.write_guard)

    }

}
