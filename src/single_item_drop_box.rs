use std::sync::Arc;

use crate::PreferredMutexType;

pub struct SingleItemDropBox<T>
{

    object: Arc<PreferredMutexType<Option<T>>>

}

impl<T> SingleItemDropBox<T>
{

    pub fn new() -> Self
    {

        Self
        { 
            
            object: Arc::new(PreferredMutexType::new(None))
        
        }

    }

    pub fn with(object: T) -> Self
    {

        Self
        {
            
            object: Arc::new(PreferredMutexType::new(Some(object)))
        
        }

    }

}