use std::{fmt::{Debug, Display}, ops::{Deref, DerefMut}};

#[cfg(feature="use_std_sync")]
use super::get_mg;

use crate::{PreferredMutexType, MutexGuard};

pub struct FauxRwlock<T>
{

    mutex: PreferredMutexType<T>

}

impl<T> FauxRwlock<T>
{

    pub fn new(object: T) -> Self
    {

        Self
        {

            mutex: PreferredMutexType::new(object)

        }

    }

    pub fn read(&self) -> FauxRwLockReadGuard<'_, T>
    {

        #[cfg(feature="use_std_sync")]
        let mg = get_mg(&self.mutex);

        #[cfg(any(feature="use_parking_lot_sync", feature="use_parking_lot_fair_sync"))]
        let mg = self.mutex.lock();
        
        FauxRwLockReadGuard::new(mg)

    }

    pub fn try_read(&self) -> Option<FauxRwLockReadGuard<'_, T>>
    {

        if let Ok(mg) = self.mutex.try_lock()
        {

            Some(FauxRwLockReadGuard::new(mg))

        }
        else
        {

            None
            
        }

    }

    pub fn write(&self) -> FauxRwLockWriteGuard<'_, T>
    {

        #[cfg(feature="use_std_sync")]
        let mg = get_mg(&self.mutex);

        #[cfg(any(feature="use_parking_lot_sync", feature="use_parking_lot_fair_sync"))]
        let mg = self.mutex.lock();
        
        FauxRwLockWriteGuard::new(mg)

    }

    pub fn try_write(&self) -> Option<FauxRwLockWriteGuard<'_, T>>
    {

        if let Ok(mg) = self.mutex.try_lock()
        {

            Some(FauxRwLockWriteGuard::new(mg))

        }
        else
        {

            None
            
        }

    }

}


impl<T> Default for FauxRwlock<T>
    where T: Default
{

    fn default() -> Self
    {

        Self
        {
            
            mutex: Default::default()
        
        }

    }

}

/*
impl<T> Display for FauxRwlock<T>
    where T: Display
{

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        
        write!(f, "{}", self.mutex)
        
    }

}
*/

impl<T> Debug for FauxRwlock<T>
    where T: Debug
{

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("FauxRwlock").field("mutex", &self.mutex).finish()
    }

}

pub struct FauxRwLockReadGuard<'a, T>
{

    mutex_guard: MutexGuard<'a, T>

}

impl<'a, T> FauxRwLockReadGuard<'a, T>
{

    pub fn new(mutex_guard: MutexGuard<'a, T>) -> Self
    {

        Self
        {

            mutex_guard

        }

    }

}

impl<'a, T> Deref for FauxRwLockReadGuard<'a, T>
{

    type Target = T;

    fn deref(&self) -> &Self::Target
    {

        &*self.mutex_guard
        
    }

}

impl<'a, T> Display for FauxRwLockReadGuard<'a, T>
    where T: Display
{

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        
        write!(f, "{}", self.mutex_guard)
        
    }

}

impl<'a, T> Debug for FauxRwLockReadGuard<'a, T>
    where T: Debug
{

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("FauxRwLockReadGuard").field("mutex_guard", &self.mutex_guard).finish()
    }

}

pub struct FauxRwLockWriteGuard<'a, T>
{

    mutex_guard: MutexGuard<'a, T>

}

impl<'a, T> FauxRwLockWriteGuard<'a, T>
{

    pub fn new(mutex_guard: MutexGuard<'a, T>) -> Self
    {

        Self
        {

            mutex_guard

        }

    }

}

impl<'a, T> Deref for FauxRwLockWriteGuard<'a, T>
{

    type Target = T;

    fn deref(&self) -> &Self::Target
    {

        &*self.mutex_guard
        
    }

}

impl<'a, T> DerefMut for FauxRwLockWriteGuard<'a, T>
{

    fn deref_mut(&mut self) -> &mut Self::Target
    {

        &mut *self.mutex_guard
        
    }

}

impl<'a, T> Display for FauxRwLockWriteGuard<'a, T>
    where T: Display
{

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        
        write!(f, "{}", self.mutex_guard)
        
    }

}

impl<'a, T> Debug for FauxRwLockWriteGuard<'a, T>
    where T: Debug
{

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("FauxRwLockWriteGuard").field("mutex_guard", &self.mutex_guard).finish()
    }
    
}