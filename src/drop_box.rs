use std::sync::Arc;

use std::collections::VecDeque;

use crate::{PreferredMutexType, WakerQueueInternals};

pub struct DropBoxInternals<T>
{

    pub items: VecDeque<T>,
    pub waker_queue_internals: WakerQueueInternals

}

impl<T> DropBoxInternals<T>
{

    pub fn new() -> Self
    {

        Self
        {

            items: VecDeque::new(),
            waker_queue_internals: WakerQueueInternals::new()

        }

    }

    pub fn with_capacity(capacity: usize) -> Self
    {

        Self
        {

            items: VecDeque::with_capacity(capacity),
            waker_queue_internals: WakerQueueInternals::with_capacity(capacity)

        }

    }

}

pub struct DropBox<T>
{

    object: Arc<PreferredMutexType<DropBoxInternals<T>>>

}

impl<T> DropBox<T>
{

    pub fn new() -> Self
    {

        Self
        { 
            
            object: Arc::new(PreferredMutexType::new(DropBoxInternals::new()))
        
        }

    }
    
    pub fn with_capacity(size: usize) -> Self
    {

        Self
        {

            waker_queue_internals: Mutex::new(Some(WakerQueueInternals::with_capacity(size)))

        }

    }

    pub fn with(object: T) -> Self
    {

        let mut dbi = DropBoxInternals::new();

        dbi.items.push_front(object);

        Self
        {
            
            object: Arc::new(PreferredMutexType::new(dbi))
        
        }

    }

    #[cfg(feature="use_std_sync")]
    fn get_mg(&self) -> MutexGuard<'_, Option<WakerQueueInternals>>
    {

        let lock_result = self.waker_queue_internals.lock();

        match lock_result
        {

            Ok(mg) =>
            {

                mg

            }
            Err(err) =>
            {

                self.waker_queue_internals.clear_poison();

                err.into_inner()

            }

        }

    }

    pub fn active_ids_len(&self) -> Option<usize>
    {

        #[cfg(feature="use_std_sync")]
        let mut mg = self.get_mg();

        #[cfg(any(feature="use_parking_lot_sync", feature="use_parking_lot_fair_sync"))]
        let mut mg = self.internal_mut_state.lock();

        if let Some(val) = &mut *mg
        {

            return Some(val.active_ids.len());

        } 

        None

    }

    pub fn active_ids_capacity(&self) -> Option<usize>
    {

        #[cfg(feature="use_std_sync")]
        let mut mg = self.get_mg();

        #[cfg(any(feature="use_parking_lot_sync", feature="use_parking_lot_fair_sync"))]
        let mut mg = self.internal_mut_state.lock();

        if let Some(val) = &mut *mg
        {

            return Some(val.active_ids.capacity());

        } 

        None

    }

    pub fn is_closed(&self) -> bool
    {

        #[cfg(feature="use_std_sync")]
        let mg = self.get_mg();

        #[cfg(any(feature="use_parking_lot_sync", feature="use_parking_lot_fair_sync"))]
        let mg = self.internal_mut_state.lock();

        mg.is_none()

    }

}