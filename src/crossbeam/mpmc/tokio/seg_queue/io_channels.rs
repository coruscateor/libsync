//! 
//!  Call io_channels to get IOClient and IOServer objects in circumstances where you need both input and output channels e.g. with actors.
//! 

use super::{Sender, Receiver, channel};

use std::fmt::Debug;

///
/// For use on the “client side” of the actor.
/// 
pub struct IOClient<IM, OM>          
{

    input_sender: Sender<IM>,
    output_receiver: Receiver<OM>,

}

impl<IM, OM> IOClient<IM, OM>
{

    pub fn new(input_sender: Sender<IM>, output_receiver: Receiver<OM>) -> Self
    {

        Self
        {

            input_sender,
            output_receiver

        }

    }

    pub fn input_sender_ref(&self) -> &Sender<IM>
    {

        &self.input_sender

    }

    pub fn output_receiver_ref(&self) -> &Receiver<OM>
    {

        &self.output_receiver

    }

    pub fn split(self) -> (Sender<IM>, Receiver<OM>)
    {

        (self.input_sender, self.output_receiver)

    }

}

impl<IM, OM> Clone for IOClient<IM, OM>
{

    fn clone(&self) -> Self
    {

        Self
        {
            
            input_sender: self.input_sender.clone(),
            output_receiver: self.output_receiver.clone()
            
        }

    }

}

impl<IM, OM> Debug for IOClient<IM, OM>
    where IM: Debug,
          OM: Debug
{

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("IOClient").field("input_sender", &self.input_sender).field("output_receiver", &self.output_receiver).finish()
    }

}

///
/// For use on the “server side” of the actor.
/// 
pub struct IOServer<IM, OM>
{

    input_receiver: Receiver<IM>,
    output_sender: Sender<OM>,

}

impl<IM, OM> IOServer<IM, OM>
{

    pub fn new(input_receiver: Receiver<IM>, output_sender: Sender<OM>) -> Self
    {

        Self
        {

            input_receiver,
            output_sender

        }

    }

    pub fn input_receiver_ref(&self) -> &Receiver<IM>
    {

        &self.input_receiver

    }

    pub fn output_sender_ref(&self) -> &Sender<OM>
    {

        &self.output_sender

    }

    pub fn split(self) -> (Receiver<IM>, Sender<OM>)
    {

        (self.input_receiver, self.output_sender)

    }

}

impl<IM, OM> Clone for IOServer<IM, OM>
{

    fn clone(&self) -> Self
    {

        Self
        {
            
            input_receiver: self.input_receiver.clone(),
            output_sender: self.output_sender.clone()
            
        }

    }

}

impl<IM, OM> Debug for IOServer<IM, OM>
    where IM: Debug,
          OM: Debug
{

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("IOServer").field("input_receiver", &self.input_receiver).field("output_sender", &self.output_sender).finish()
    }
    
}

///
/// Initialises unbounded input and output channels.
/// 
pub fn io_channels<IM, OM>() -> (IOClient<IM, OM>, IOServer<IM, OM>)
{

    let (input_sender, input_receiver) = channel();

    let (output_sender, output_receiver) = channel();

    (IOClient::new(input_sender, output_receiver), IOServer::new(input_receiver, output_sender))

}
