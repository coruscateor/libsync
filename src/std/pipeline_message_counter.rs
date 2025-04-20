use std::sync::Arc;

use delegate::delegate;

///
/// For maintaining a count of the messages sent though pipelines.
/// 
#[derive(Clone)]
pub struct PipelineMessageCounter
{

    counter: Arc<Arc<()>>

}

impl PipelineMessageCounter
{

    pub fn new() -> Self
    {

        Self
        {

            counter: Arc::new(Arc::new(()))

        }

    }

    //The returned count should exclude the initial interior Arc instance which stays as part of the PipelineMessageCounter.

    pub fn count(&self) -> usize
    {

        Arc::strong_count(self.counter.as_ref()) - 1

    }

    pub fn has_messages(&self) -> bool
    {

        self.count() > 0

    }

    pub fn increment(&self) -> IncrementedPipelineMessageCounter
    {

        IncrementedPipelineMessageCounter::new(self.counter.as_ref())

    }

    pub fn increment_with_message<T>(&self, message: T) -> CountedPipelineMessage<T>
    {

        let incremented = self.increment();

        CountedPipelineMessage::new(incremented, message)

    }

    pub fn increment_with_message_mut<T>(&self, message: T) -> CountedPipelineMessageMut<T>
    {

        let incremented = self.increment();

        CountedPipelineMessageMut::new(incremented, message)
        
    }

    pub fn increment_without_message_mut<T>(&self) -> CountedPipelineMessageMut<T>
    {

        let incremented = self.increment();

        CountedPipelineMessageMut::none(incremented)
        
    }

    pub fn is(&self, other: &Self) -> bool
    {

        Arc::ptr_eq(&self.counter, &other.counter)

    }
    
    pub fn instance_count(&self) -> usize
    {

        Arc::strong_count(&self.counter)

    }

}

///
/// For keeping the message count incremented until dropped. 
/// 
pub struct IncrementedPipelineMessageCounter
{

    counter: Arc<()>

}

impl IncrementedPipelineMessageCounter
{

    pub fn new(counter: &Arc<()>) -> Self
    {

        Self
        {

            counter: counter.clone()

        }

    }

    //There's always at least one of the provided and cloned Arc instance.

    pub fn count(&self) -> usize
    {

        Arc::strong_count(&self.counter) - 1

    }

    pub fn has_messages(&self) -> bool
    {

        self.count() > 0

    }

    pub fn is(&self, other: &Self) -> bool
    {

        Arc::ptr_eq(&self.counter, &other.counter)

    }

}

///
/// A counted message.
/// 
pub struct CountedPipelineMessage<T>
{

    incremented: IncrementedPipelineMessageCounter,
    message: T

}

impl<T> CountedPipelineMessage<T>
{

    pub fn new(incremented: IncrementedPipelineMessageCounter, message: T) -> Self
    {

        Self
        {

            incremented,
            message

        }

    }

    pub fn incremented(&self) -> &IncrementedPipelineMessageCounter
    {

        &self.incremented

    }

    pub fn take(self) -> T
    {

        self.message

    }

    pub fn take_incremented(self) -> IncrementedPipelineMessageCounter
    {

        self.incremented

    }

    pub fn take_both(self) -> (IncrementedPipelineMessageCounter, T)
    {

        (self.incremented, self.message)

    }

}

impl<T> AsRef<T> for CountedPipelineMessage<T>
{

    fn as_ref(&self) -> &T
    {

        &self.message
        
    }

}

impl<T> AsMut<T> for CountedPipelineMessage<T>
{

    fn as_mut(&mut self) -> &mut T
    {

        &mut self.message

    }

}

/*
impl<T> CountedPipelineMessage<Option<T>>
{

    delegate!
    {

        to self.message
        {

            pub const fn is_some(&self) -> bool;

            pub const fn is_none(&self) -> bool;

            pub fn take(&mut self) -> Option<T>;

            pub fn take_if<P>(&mut self, predictate: P) -> Option<T>
                where P: FnOnce(&mut T) -> bool;
            
            pub fn replace(&mut self, value: T) -> Option<T>;

        }

    }

}
*/

///
/// A counted message object that is mutable to the point where the message can be removed completely.
/// 
pub struct CountedPipelineMessageMut<T>
{

    incremented: IncrementedPipelineMessageCounter,
    message: Option<T>

}

impl<T> CountedPipelineMessageMut<T>
{

    pub fn new(incremented: IncrementedPipelineMessageCounter, message: T) -> Self
    {

        Self
        {

            incremented,
            message: Some(message)

        }

    }

    pub fn option(incremented: IncrementedPipelineMessageCounter, message: Option<T>) -> Self
    {

        Self
        {

            incremented,
            message

        }

    }

    pub fn none(incremented: IncrementedPipelineMessageCounter) -> Self
    {

        Self
        {

            incremented,
            message: None

        }

    }

    pub fn incremented(&self) -> &IncrementedPipelineMessageCounter
    {

        &self.incremented

    }

    pub fn take_incremented(self) -> IncrementedPipelineMessageCounter
    {

        self.incremented

    }

    pub fn take_both(self) -> (IncrementedPipelineMessageCounter, Option<T>)
    {

        (self.incremented, self.message)

    }

    pub fn as_ref(&self) -> &Option<T>
    {

        &self.message
        
    }

    pub fn as_mut(&mut self) -> &mut Option<T>
    {

        &mut self.message

    }

    delegate!
    {

        to self.message
        {

            pub const fn is_some(&self) -> bool;

            pub const fn is_none(&self) -> bool;

            pub fn take(&mut self) -> Option<T>;

            pub fn take_if<P>(&mut self, predictate: P) -> Option<T>
                where P: FnOnce(&mut T) -> bool;
            
            pub fn replace(&mut self, value: T) -> Option<T>;

        }

    }

}

/*
impl<T> AsRef<T> for MutCountedPipelineMessage<T>
{

    fn as_ref(&self) -> &Option<T>
    {

        &self.message
        
    }

}

impl<T> AsMut<T> for MutCountedPipelineMessage<T>
{

    fn as_mut(&mut self) -> &mut T
    {

        &mut self.message

    }

}
*/

