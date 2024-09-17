use std::marker::PhantomData;

use delegate::delegate;

use super::{CountedPipelineMessage, CountedPipelineMessageMut, PipelineMessageCounter};

pub trait PipelineMessageContainer<T>
    : AsRef<T> + 
      AsMut<T>
    
{

    fn get_next<U>(self, message: U) -> impl PipelineMessageContainer<U>;

}

pub trait PipelineMessageContainerMut<T>
{

    fn as_ref(&self) -> &Option<T>;

    fn as_mut(&mut self) -> &mut Option<T>;

    fn is_some(&self) -> bool;

    fn is_none(&self) -> bool;

    fn take(&mut self) -> Option<T>;

    fn take_if<P>(&mut self, predictate: P) -> Option<T>
        where P: FnOnce(&mut T) -> bool;
    
    fn replace(&mut self, value: T) -> Option<T>;

    fn get_next<U>(self, message: U) -> impl PipelineMessageContainerMut<U>;

}

pub trait PipelineMessageContainerFactory<T>
{

    fn get(&self, message: T) -> impl PipelineMessageContainer<T>;

    fn get_mut(&self, message: T) -> impl PipelineMessageContainerMut<T>;

}

//Implementations

//PlainPipelineMessageContainer

pub struct PlainPipelineMessageContainer<T>
{

    message: T

}

impl<T> PlainPipelineMessageContainer<T>
{

    pub fn new(message: T) -> Self
    {

        Self
        {

            message

        }

    }

}

impl<T> PipelineMessageContainer<T> for PlainPipelineMessageContainer<T>
{

    fn get_next<U>(self, message: U) -> impl PipelineMessageContainer<U>
    {

        PlainPipelineMessageContainer::new(message)

    }

}

impl<T> AsRef<T> for PlainPipelineMessageContainer<T>
{

    fn as_ref(&self) -> &T
    {

        &self.message
        
    }

}

impl<T> AsMut<T> for PlainPipelineMessageContainer<T>
{

    fn as_mut(&mut self) -> &mut T
    {

        &mut self.message

    }

}

//PlainPipelineMessageContainerMut

pub struct PlainPipelineMessageContainerMut<T>
{

    message: Option<T>

}

impl<T> PlainPipelineMessageContainerMut<T>
{

    pub fn new(message: T) -> Self
    {

        Self
        {

            message: Some(message)

        }

    }

}

impl<T> PipelineMessageContainerMut<T> for PlainPipelineMessageContainerMut<T>
{

    fn as_ref(&self) -> &Option<T>
    {

        &self.message
        
    }

    fn as_mut(&mut self) -> &mut Option<T>
    {

        &mut self.message

    }

    delegate!
    {

        to self.message
        {
        
            fn is_some(&self) -> bool;
        
            fn is_none(&self) -> bool;
        
            fn take(&mut self) -> Option<T>;
        
            fn take_if<P>(&mut self, predictate: P) -> Option<T>
                where P: FnOnce(&mut T) -> bool;
            
            fn replace(&mut self, value: T) -> Option<T>;

        }

    }

    fn get_next<U>(self, message: U) -> impl PipelineMessageContainerMut<U>
    {

        PlainPipelineMessageContainerMut::new(message)

    }

}

//PlainPipelineMessageContainerFactory

pub struct PlainPipelineMessageContainerFactory<T>
{

    phantom_data: PhantomData<T>

}

impl<T> PlainPipelineMessageContainerFactory<T>
{

    pub fn new() -> Self
    {

        Self
        {

            phantom_data: PhantomData::default() 

        }

    }

}

impl<T> PipelineMessageContainerFactory<T> for PlainPipelineMessageContainerFactory<T>
{

    fn get(&self, message: T) -> impl PipelineMessageContainer<T>
    {

        PlainPipelineMessageContainer::new(message)
        
    }

    fn get_mut(&self, message: T) -> impl PipelineMessageContainerMut<T>
    {

        PlainPipelineMessageContainerMut::new(message)

    }

}

//CountedPipelineMessageContainer

pub struct CountedPipelineMessageContainer<T>
{

    message: CountedPipelineMessage<T>

}

impl<T> CountedPipelineMessageContainer<T>
{

    pub fn new(message: CountedPipelineMessage<T>) -> Self
    {

        Self
        {

            message

        }

    }

}

impl<T> PipelineMessageContainer<T> for CountedPipelineMessageContainer<T>
{

    fn get_next<U>(self, message: U) -> impl PipelineMessageContainer<U>
    {

        let incremented = self.message.take_incremented();

        let cpm = CountedPipelineMessage::new(incremented, message);

        CountedPipelineMessageContainer::new(cpm)

    }

}

impl<T> AsRef<T> for CountedPipelineMessageContainer<T>
{

    fn as_ref(&self) -> &T
    {

        self.message.as_ref()
        
    }

}

impl<T> AsMut<T> for CountedPipelineMessageContainer<T>
{

    fn as_mut(&mut self) -> &mut T
    {

        self.message.as_mut()

    }

}

//CountednPipelineMessageContainerMut

pub struct CountedPipelineMessageContainerMut<T>
{

    message: CountedPipelineMessageMut<T>

}

impl<T> CountedPipelineMessageContainerMut<T>
{

    pub fn new(message: CountedPipelineMessageMut<T>) -> Self
    {

        Self
        {

            message

        }

    }

}

impl<T> PipelineMessageContainerMut<T> for CountedPipelineMessageContainerMut<T>
{

    delegate!
    {

        to self.message
        {

            fn as_ref(&self) -> &Option<T>;

            fn as_mut(&mut self) -> &mut Option<T>;
        
            fn is_some(&self) -> bool;
        
            fn is_none(&self) -> bool;
        
            fn take(&mut self) -> Option<T>;
        
            fn take_if<P>(&mut self, predictate: P) -> Option<T>
                where P: FnOnce(&mut T) -> bool;
            
            fn replace(&mut self, value: T) -> Option<T>;

        }

    }

    fn get_next<U>(self, message: U) -> impl PipelineMessageContainerMut<U>
    {

        let incremented = self.message.take_incremented();

        let cpm = CountedPipelineMessageMut::new(incremented, message);

        CountedPipelineMessageContainerMut::new(cpm)

    }

}

//CountedPipelineMessageContainerFactory

pub struct CountedPipelineMessageContainerFactory<T>
{

    pipeline_message_counter: PipelineMessageCounter,
    phantom_data: PhantomData<T>

}

impl<T> CountedPipelineMessageContainerFactory<T>
{

    pub fn new(pipeline_message_counter: PipelineMessageCounter) -> Self
    {

        Self
        {

            pipeline_message_counter,
            phantom_data: PhantomData::default() 

        }

    }

}

impl<T> PipelineMessageContainerFactory<T> for CountedPipelineMessageContainerFactory<T>
{

    fn get(&self, message: T) -> impl PipelineMessageContainer<T>
    {

        CountedPipelineMessageContainer::new(self.pipeline_message_counter.increment_with_message(message))
        
    }

    fn get_mut(&self, message: T) -> impl PipelineMessageContainerMut<T>
    {

        CountedPipelineMessageContainerMut::new(self.pipeline_message_counter.increment_with_message_mut(message))

    }

}


