use std::{any::{Any, TypeId}, ptr, sync::atomic::{AtomicBool, AtomicI8, AtomicI16, AtomicI32, AtomicI64, AtomicIsize, AtomicPtr, AtomicU8, AtomicU16, AtomicU32, AtomicU64, AtomicUsize, Ordering}};

use crate::{PreferredMutexType, ValueOrMutexGuard};

use std::fmt::Debug;

#[cfg(feature="use_std_sync")]
use crate::sync::get_mg;

use delegate::delegate;

use crate::MutexGuard;

pub enum AtomicOrMutex<T = ()>
    where T: 'static
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

impl<T> AtomicOrMutex<T>
    where T: 'static
{

    pub fn is_atomic_bool(&self) -> bool
    {

        matches!(self, Self::AtomicBool(_))

    }

    pub fn is_atomic_i8(&self) -> bool
    {

        matches!(self, Self::AtomicI8(_))

    }
    
    pub fn is_atomic_i16(&self) -> bool
    {

        matches!(self, Self::AtomicI16(_))

    }

    pub fn is_atomic_i32(&self) -> bool
    {

        matches!(self, Self::AtomicI32(_))

    }

    pub fn is_atomic_i64(&self) -> bool
    {

        matches!(self, Self::AtomicI64(_))

    }

    pub fn is_atomic_isize(&self) -> bool
    {

        matches!(self, Self::AtomicI8(_))

    }

    pub fn is_atomic_ptr(&self) -> bool
    {

        matches!(self, Self::AtomicPtr(_))

    }

    pub fn is_atomic_u8(&self) -> bool
    {

        matches!(self, Self::AtomicU8(_))

    }
    
    pub fn is_atomic_u16(&self) -> bool
    {

        matches!(self, Self::AtomicU16(_))

    }

    pub fn is_atomic_u32(&self) -> bool
    {

        matches!(self, Self::AtomicU32(_))

    }

    pub fn is_atomic_u64(&self) -> bool
    {

        matches!(self, Self::AtomicU64(_))

    }

    pub fn is_atomic_usize(&self) -> bool
    {

        matches!(self, Self::AtomicUsize(_))

    }

    pub fn is_mutex(&self) -> bool
    {

        matches!(self, Self::Mutex(_))

    }

    pub fn is_lock_free(&self) -> bool
    {

        match self
        {

            Self::Mutex(_) =>
            {

                false

            }
            _ =>
            {

                true

            }

        }

    }

    fn into_t<'a>(any_value_ref: &'a dyn Any) -> &'a T
    {

        if let Some(val) = any_value_ref.downcast_ref::<T>()
        {  

            val

        }
        else
        {

            panic!("Error: Conversion failed")
            
        }

    }
    
    pub fn clone_value(&self) -> T
        where T: Clone
    {

        match self
        {

            AtomicOrMutex::AtomicBool(atomic) =>
            {

                let value = atomic.load(Ordering::Acquire);

                Self::into_t(&value).clone()

            },
            AtomicOrMutex::AtomicI8(atomic) =>
            {

                let value = atomic.load(Ordering::Acquire);

                Self::into_t(&value).clone()

            }
            AtomicOrMutex::AtomicI16(atomic) =>
            {

                let value = atomic.load(Ordering::Acquire);

                Self::into_t(&value).clone()

            }
            AtomicOrMutex::AtomicI32(atomic) =>
            {

                let value = atomic.load(Ordering::Acquire);

                Self::into_t(&value).clone()

            }
            AtomicOrMutex::AtomicI64(atomic) =>
            {

                let value = atomic.load(Ordering::Acquire);

                Self::into_t(&value).clone()

            }
            AtomicOrMutex::AtomicIsize(atomic) =>
            {

                let value = atomic.load(Ordering::Acquire);

                Self::into_t(&value).clone()

            }
            AtomicOrMutex::AtomicPtr(atomic) =>
            {

                let value = atomic.load(Ordering::Acquire);

                Self::into_t(&value).clone()

            }
            AtomicOrMutex::AtomicU8(atomic) =>
            {

                let value = atomic.load(Ordering::Acquire);

                Self::into_t(&value).clone()

            }
            AtomicOrMutex::AtomicU16(atomic) =>
            {

                let value = atomic.load(Ordering::Acquire);

                Self::into_t(&value).clone()

            }
            AtomicOrMutex::AtomicU32(atomic) =>
            {

                let value = atomic.load(Ordering::Acquire);

                Self::into_t(&value).clone()

            }
            AtomicOrMutex::AtomicU64(atomic) =>
            {

                let value = atomic.load(Ordering::Acquire);

                Self::into_t(&value).clone()

            }
            AtomicOrMutex::AtomicUsize(atomic) =>
            {

                let value = atomic.load(Ordering::Acquire);

                Self::into_t(&value).clone()

            }
            AtomicOrMutex::Mutex(mutex) =>
            {

                #[cfg(feature="use_std_sync")]
                let mg = get_mg(mutex);

                #[cfg(any(feature="use_parking_lot_sync", feature="use_parking_lot_fair_sync"))]
                let mg = mutex.lock();

                mg.clone()

            }

        }

    }

    pub fn value_or_mg<'a>(&'a self) -> ValueOrMutexGuard<'a, T>
        where T: Clone
    {

        match self
        {

            AtomicOrMutex::AtomicBool(atomic) =>
            {

                let value = atomic.load(Ordering::Acquire);

                ValueOrMutexGuard::Value(Self::into_t(&value).clone())

            },
            AtomicOrMutex::AtomicI8(atomic) =>
            {

                let value = atomic.load(Ordering::Acquire);

                ValueOrMutexGuard::Value(Self::into_t(&value).clone())

            }
            AtomicOrMutex::AtomicI16(atomic) =>
            {

                let value = atomic.load(Ordering::Acquire);

                ValueOrMutexGuard::Value(Self::into_t(&value).clone())

            }
            AtomicOrMutex::AtomicI32(atomic) =>
            {

                let value = atomic.load(Ordering::Acquire);

                ValueOrMutexGuard::Value(Self::into_t(&value).clone())

            }
            AtomicOrMutex::AtomicI64(atomic) =>
            {

                let value = atomic.load(Ordering::Acquire);

                ValueOrMutexGuard::Value(Self::into_t(&value).clone())

            }
            AtomicOrMutex::AtomicIsize(atomic) =>
            {

                let value = atomic.load(Ordering::Acquire);

                ValueOrMutexGuard::Value(Self::into_t(&value).clone())

            }
            AtomicOrMutex::AtomicPtr(atomic) =>
            {

                let value = atomic.load(Ordering::Acquire);

                ValueOrMutexGuard::Value(Self::into_t(&value).clone())

            }
            AtomicOrMutex::AtomicU8(atomic) =>
            {

                let value = atomic.load(Ordering::Acquire);

                ValueOrMutexGuard::Value(Self::into_t(&value).clone())

            }
            AtomicOrMutex::AtomicU16(atomic) =>
            {

                let value = atomic.load(Ordering::Acquire);

                ValueOrMutexGuard::Value(Self::into_t(&value).clone())

            }
            AtomicOrMutex::AtomicU32(atomic) =>
            {

                let value = atomic.load(Ordering::Acquire);

                ValueOrMutexGuard::Value(Self::into_t(&value).clone())

            }
            AtomicOrMutex::AtomicU64(atomic) =>
            {

                let value = atomic.load(Ordering::Acquire);

                ValueOrMutexGuard::Value(Self::into_t(&value).clone())

            }
            AtomicOrMutex::AtomicUsize(atomic) =>
            {

                let value = atomic.load(Ordering::Acquire);

                ValueOrMutexGuard::Value(Self::into_t(&value).clone())

            }
            AtomicOrMutex::Mutex(mutex) =>
            {

                #[cfg(feature="use_std_sync")]
                let mg = get_mg(mutex);

                #[cfg(any(feature="use_parking_lot_sync", feature="use_parking_lot_fair_sync"))]
                let mg = mutex.lock();

                ValueOrMutexGuard::MutexGuard(mg)

            }

        }

    }

    pub fn get_mg<'a>(&'a self) -> Option<MutexGuard<'a, T>>
    {

        match self
        {

            Self::Mutex(mutex) =>
            {

                #[cfg(feature="use_std_sync")]
                let mg = get_mg(mutex);

                #[cfg(any(feature="use_parking_lot_sync", feature="use_parking_lot_fair_sync"))]
                let mg = mutex.lock();

                Some(mg)

            }
            _ =>
            {

                None

            }

        }

    }

    fn update_value<F>(any_value_ref: &mut dyn Any, fun: &mut F)
        where F: FnMut(&mut T)
    {

        if let Some(val) = any_value_ref.downcast_mut::<T>()
        {

            fun(val)

        }
        else
        {

            panic!("Error: Conversion failed")
            
        }

    }

    pub fn update<F>(&self, fun: &mut F)
        where T: Clone,
              F: FnMut(&mut T)
    {

        match self
        {

            AtomicOrMutex::AtomicBool(atomic) =>
            {

                let mut value = atomic.load(Ordering::Acquire);

                loop
                {

                    let origninal_value = value.clone();

                    Self::update_value(&mut value, fun);

                    let res = atomic.compare_exchange(origninal_value, value, Ordering::Relaxed, Ordering::Acquire);

                    match res
                    {

                        Ok(_) =>
                        {

                            return;

                        }
                        Err(val) =>
                        {

                            value = val;

                        }

                    }

                }

            },
            AtomicOrMutex::AtomicI8(atomic) =>
            {

                let mut value = atomic.load(Ordering::Acquire);

                loop
                {

                    let origninal_value = value.clone();

                    Self::update_value(&mut value, fun);

                    let res = atomic.compare_exchange(origninal_value, value, Ordering::Relaxed, Ordering::Acquire);

                    match res
                    {

                        Ok(_) =>
                        {

                            return;

                        }
                        Err(val) =>
                        {

                            value = val;

                        }

                    }

                }

            }
            AtomicOrMutex::AtomicI16(atomic) =>
            {

                let mut value = atomic.load(Ordering::Acquire);

                loop
                {

                    let origninal_value = value.clone();

                    Self::update_value(&mut value, fun);

                    let res = atomic.compare_exchange(origninal_value, value, Ordering::Relaxed, Ordering::Acquire);

                    match res
                    {

                        Ok(_) =>
                        {

                            return;

                        }
                        Err(val) =>
                        {

                            value = val;

                        }

                    }

                }

            }
            AtomicOrMutex::AtomicI32(atomic) =>
            {

                let mut value = atomic.load(Ordering::Acquire);

                loop
                {

                    let origninal_value = value.clone();

                    Self::update_value(&mut value, fun);

                    let res = atomic.compare_exchange(origninal_value, value, Ordering::Relaxed, Ordering::Acquire);

                    match res
                    {

                        Ok(_) =>
                        {

                            return;

                        }
                        Err(val) =>
                        {

                            value = val;

                        }

                    }

                }

            }
            AtomicOrMutex::AtomicI64(atomic) =>
            {

                let mut value = atomic.load(Ordering::Acquire);

                loop
                {

                    let origninal_value = value.clone();

                    Self::update_value(&mut value, fun);

                    let res = atomic.compare_exchange(origninal_value, value, Ordering::Relaxed, Ordering::Acquire);

                    match res
                    {

                        Ok(_) =>
                        {

                            return;

                        }
                        Err(val) =>
                        {

                            value = val;

                        }

                    }

                }

            }
            AtomicOrMutex::AtomicIsize(atomic) =>
            {

                let mut value = atomic.load(Ordering::Acquire);

                loop
                {

                    let origninal_value = value.clone();

                    Self::update_value(&mut value, fun);

                    let res = atomic.compare_exchange(origninal_value, value, Ordering::Relaxed, Ordering::Acquire);

                    match res
                    {

                        Ok(_) =>
                        {

                            return;

                        }
                        Err(val) =>
                        {

                            value = val;

                        }

                    }

                }

            }
            AtomicOrMutex::AtomicPtr(atomic) =>
            {

                let mut value = atomic.load(Ordering::Acquire);

                loop
                {

                    let origninal_value = value.clone();

                    Self::update_value(&mut value, fun);

                    let res = atomic.compare_exchange(origninal_value, value, Ordering::Relaxed, Ordering::Acquire);

                    match res
                    {

                        Ok(_) =>
                        {

                            return;

                        }
                        Err(val) =>
                        {

                            value = val;

                        }

                    }

                }

            }
            AtomicOrMutex::AtomicU8(atomic) =>
            {

                let mut value = atomic.load(Ordering::Acquire);

                loop
                {

                    let origninal_value = value.clone();

                    Self::update_value(&mut value, fun);

                    let res = atomic.compare_exchange(origninal_value, value, Ordering::Relaxed, Ordering::Acquire);

                    match res
                    {

                        Ok(_) =>
                        {

                            return;

                        }
                        Err(val) =>
                        {

                            value = val;

                        }

                    }

                }

            }
            AtomicOrMutex::AtomicU16(atomic) =>
            {

                let mut value = atomic.load(Ordering::Acquire);

                loop
                {

                    let origninal_value = value.clone();

                    Self::update_value(&mut value, fun);

                    let res = atomic.compare_exchange(origninal_value, value, Ordering::Relaxed, Ordering::Acquire);

                    match res
                    {

                        Ok(_) =>
                        {

                            return;

                        }
                        Err(val) =>
                        {

                            value = val;

                        }

                    }

                }

            }
            AtomicOrMutex::AtomicU32(atomic) =>
            {

                let mut value = atomic.load(Ordering::Acquire);

                loop
                {

                    let origninal_value = value.clone();

                    Self::update_value(&mut value, fun);

                    let res = atomic.compare_exchange(origninal_value, value, Ordering::Relaxed, Ordering::Acquire);

                    match res
                    {

                        Ok(_) =>
                        {

                            return;

                        }
                        Err(val) =>
                        {

                            value = val;

                        }

                    }

                }

            }
            AtomicOrMutex::AtomicU64(atomic) =>
            {

                let mut value = atomic.load(Ordering::Acquire);

                loop
                {

                    let origninal_value = value.clone();

                    Self::update_value(&mut value, fun);

                    let res = atomic.compare_exchange(origninal_value, value, Ordering::Relaxed, Ordering::Acquire);

                    match res
                    {

                        Ok(_) =>
                        {

                            return;

                        }
                        Err(val) =>
                        {

                            value = val;

                        }

                    }

                }

            }
            AtomicOrMutex::AtomicUsize(atomic) =>
            {

                let mut value = atomic.load(Ordering::Acquire);

                loop
                {

                    let origninal_value = value.clone();

                    Self::update_value(&mut value, fun);

                    let res = atomic.compare_exchange(origninal_value, value, Ordering::Relaxed, Ordering::Acquire);

                    match res
                    {

                        Ok(_) =>
                        {

                            return;

                        }
                        Err(val) =>
                        {

                            value = val;

                        }

                    }

                }

            }
            AtomicOrMutex::Mutex(mutex) =>
            {

                #[cfg(feature="use_std_sync")]
                let mut mg = get_mg(mutex);

                #[cfg(any(feature="use_parking_lot_sync", feature="use_parking_lot_fair_sync"))]
                let mut mg = mutex.lock();

                fun(&mut mg);

            }

        }

    }

}

impl<T> Default for AtomicOrMutex<T>
{

    fn default() -> Self
    {

        Self::AtomicBool(AtomicBool::new(Default::default()))

    }
}

impl<T> Debug for AtomicOrMutex<T>
    where T: Debug
{

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::AtomicBool(arg0) => f.debug_tuple("AtomicBool").field(arg0).finish(),
            Self::AtomicI8(arg0) => f.debug_tuple("AtomicI8").field(arg0).finish(),
            Self::AtomicI16(arg0) => f.debug_tuple("AtomicI16").field(arg0).finish(),
            Self::AtomicI32(arg0) => f.debug_tuple("AtomicI32").field(arg0).finish(),
            Self::AtomicI64(arg0) => f.debug_tuple("AtomicI64").field(arg0).finish(),
            Self::AtomicIsize(arg0) => f.debug_tuple("AtomicIsize").field(arg0).finish(),
            Self::AtomicPtr(arg0) => f.debug_tuple("AtomicPtr").field(arg0).finish(),
            Self::AtomicU8(arg0) => f.debug_tuple("AtomicU8").field(arg0).finish(),
            Self::AtomicU16(arg0) => f.debug_tuple("AtomicU16").field(arg0).finish(),
            Self::AtomicU32(arg0) => f.debug_tuple("AtomicU32").field(arg0).finish(),
            Self::AtomicU64(arg0) => f.debug_tuple("AtomicU64").field(arg0).finish(),
            Self::AtomicUsize(arg0) => f.debug_tuple("AtomicUsize").field(arg0).finish(),
            Self::Mutex(arg0) => f.debug_tuple("Mutex").field(arg0).finish(),
        }
    }

}

#[derive(Default)]
pub struct AtomicValueOrMutex<T = ()>
    where T: 'static
{

    value: AtomicOrMutex<T>

}

impl<T> AtomicValueOrMutex<T>
    where T: 'static
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
        else if let Some(val) = any_value_ref.downcast_ref::<*mut T>()
        {  

            aom_value = AtomicOrMutex::AtomicPtr(AtomicPtr::new(*val));

        }
        else if let Some(val) = any_value_ref.downcast_ref::<u8>()
        {  

            aom_value = AtomicOrMutex::AtomicU8(AtomicU8::new(*val));

        }
        else if let Some(val) = any_value_ref.downcast_ref::<u16>()
        {  

            aom_value = AtomicOrMutex::AtomicU16(AtomicU16::new(*val));

        }
        else if let Some(val) = any_value_ref.downcast_ref::<u32>()
        {  

            aom_value = AtomicOrMutex::AtomicU32(AtomicU32::new(*val));

        }
        else if let Some(val) = any_value_ref.downcast_ref::<u64>()
        {  

            aom_value = AtomicOrMutex::AtomicU64(AtomicU64::new(*val));

        }
        else if let Some(val) = any_value_ref.downcast_ref::<usize>()
        {  

            aom_value = AtomicOrMutex::AtomicUsize(AtomicUsize::new(*val));

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

    delegate!
    {

        to self.value
        {

            pub fn is_atomic_bool(&self) -> bool;

            pub fn is_atomic_i8(&self) -> bool;
    
            pub fn is_atomic_i16(&self) -> bool;

            pub fn is_atomic_i32(&self) -> bool;

            pub fn is_atomic_i64(&self) -> bool;

            pub fn is_atomic_isize(&self) -> bool;

            pub fn is_atomic_ptr(&self) -> bool;

            pub fn is_atomic_u8(&self) -> bool;
    
            pub fn is_atomic_u16(&self) -> bool;

            pub fn is_atomic_u32(&self) -> bool;

            pub fn is_atomic_u64(&self) -> bool;

            pub fn is_atomic_usize(&self) -> bool;

            pub fn is_mutex(&self) -> bool;

            pub fn is_lock_free(&self) -> bool;

            pub fn clone_value(&self) -> T
                where T: Clone;

            pub fn value_or_mg<'a>(&'a self) -> ValueOrMutexGuard<'a, T>
                where T: Clone;

            pub fn get_mg<'a>(&'a self) -> Option<MutexGuard<'a, T>>;

            pub fn update<F>(&self, fun: &mut F)
                where T: Clone,
                      F: FnMut(&mut T);
            
        }

    }

}

impl<T> Debug for AtomicValueOrMutex<T>
    where T: Debug
{

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AtomicValueOrMutex").field("value", &self.value).finish()
    }

}
