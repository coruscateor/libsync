use std::{any::{Any, TypeId}, sync::atomic::{AtomicBool, AtomicI8, AtomicI16, AtomicI32, AtomicI64, AtomicIsize, AtomicPtr, AtomicU8, AtomicU16, AtomicU32, AtomicU64, AtomicUsize}};

use crate::PreferredMutexType;

pub enum AtomicOrMutex<T>
{

    AtomicBool(AtomicBool),
    AtomicI8(AtomicI8),
    AtomicI16(AtomicI16),
    AtomicI32(AtomicI32),
    AtomicI64(AtomicI64),
    AtomicIsize(AtomicIsize),
    AtomicPtr(AtomicPtr<T>),
    AtomicU8(AtomicU8),
    AtomicU16(AtomicU16),
    AtomicU32(AtomicU32),
    AtomicU64(AtomicU64),
    AtomicUsize(AtomicUsize),
    Mutex(PreferredMutexType<T>)
    
}

pub struct AtomicValueOrMutex<T>
{

    value: AtomicOrMutex<T>

}

impl<T> AtomicValueOrMutex<T>
{

    pub fn new(value: T) -> Self
        where T: 'static
    {

        let any_value_ref: &dyn Any = &value;

        let aom_value;

        if let Some(val) = any_value_ref.downcast_ref::<bool>()
        {  

            aom_value = AtomicOrMutex::AtomicBool(AtomicBool::new(*val));

        }
        else if let Some(val) = any_value_ref.downcast_ref::<i8>()
        {  

            aom_value = AtomicOrMutex::AtomicI8(AtomicI8::new(*val));

        }
        else if let Some(val) = any_value_ref.downcast_ref::<i16>()
        {  

            aom_value = AtomicOrMutex::AtomicI16(AtomicI16::new(*val));

        }
        else if let Some(val) = any_value_ref.downcast_ref::<i32>()
        {  

            aom_value = AtomicOrMutex::AtomicI32(AtomicI32::new(*val));

        }
        else if let Some(val) = any_value_ref.downcast_ref::<i64>()
        {  

            aom_value = AtomicOrMutex::AtomicI64(AtomicI64::new(*val));

        }
        else if let Some(val) = any_value_ref.downcast_ref::<isize>()
        {  

            aom_value = AtomicOrMutex::AtomicIsize(AtomicIsize::new(*val));

        }
        else
        {

            aom_value = AtomicOrMutex::Mutex(PreferredMutexType::new(value));

        }
            

        Self
        {

            value: aom_value

        }

    }

}