//! Internal module describing the serialization/deserialization of messages
//!
//! The DFHack API includes a set of headers and error code wrapping the serialized
//! protobuf messages. This module implements this logic.

use crate::DFHackError;
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use derive_more::Display;
use num_enum::TryFromPrimitive;
use protobuf::Message;
use std::convert::TryFrom;

pub trait Send {
    fn send<T: std::io::Write>(&self, stream: &mut T) -> crate::DFHackResult<()>;
}

pub trait Receive {
    fn receive<T: std::io::Read>(stream: &mut T) -> crate::DFHackResult<Self>
    where
        Self: Sized;
}

// https://github.com/DFHack/dfhack/blob/0.47.05-r4/library/include/RemoteClient.h#L53
#[derive(Copy, Clone, PartialEq, TryFromPrimitive)]
#[repr(i16)]
#[derive(Display)]
pub enum RpcReplyCode {
    Result = -1,
    Fail = -2,
    Text = -3,
    Quit = -4,
}

// https://github.com/DFHack/dfhack/blob/0.47.05-r4/library/include/RemoteClient.h#L42
#[derive(Copy, Clone, PartialEq, TryFromPrimitive)]
#[repr(i32)]
#[derive(Display)]
pub enum CommandResult {
    LinkFailure = -3,
    NeedsConsole = -2,
    NotImplemented = -1,
    Ok = 0,
    Failure = 1,
    WrongUsage = 2,
    NotFound = 3,
}

// https://docs.dfhack.org/en/stable/docs/Remote.html#handshake-request
// https://docs.dfhack.org/en/stable/docs/Remote.html#handshake-reply
#[derive(Display)]
#[display(fmt = "Handshake {} version {}", magic, version)]
pub struct Handshake {
    pub magic: String,
    pub version: i32,
}

// https://docs.dfhack.org/en/stable/docs/Remote.html#header
#[derive(Display)]
#[display(fmt = "Header id {}, size {}", id, size)]
pub struct Header {
    pub id: i16,
    padding: i16,
    pub size: i32,
}

// https://docs.dfhack.org/en/stable/docs/Remote.html#request
#[derive(Display)]
#[display(fmt = "protobuf message id {}", id)]
pub struct Request<TMessage: protobuf::Message> {
    pub id: i16,
    pub message: TMessage,
}

pub enum Reply<TMessage: protobuf::Message> {
    // https://docs.dfhack.org/en/stable/docs/Remote.html#text
    Text(crate::messages::CoreTextNotification),

    // https://docs.dfhack.org/en/stable/docs/Remote.html#result
    Result(TMessage),

    // https://docs.dfhack.org/en/stable/docs/Remote.html#failure
    Failure(CommandResult),
}

// https://docs.dfhack.org/en/stable/docs/Remote.html#quit
#[derive(Display)]
pub struct Quit {}

impl Handshake {
    pub fn new(magic: String, version: i32) -> Self {
        Handshake { magic, version }
    }
}

impl Send for Handshake {
    fn send<T: std::io::Write>(&self, stream: &mut T) -> crate::DFHackResult<()> {
        log::trace!("Sending {}", self);
        stream.write(self.magic.as_bytes())?;
        self.version.send(stream)?;
        log::trace!("Sent handshake");
        Ok(())
    }
}

impl Receive for Handshake {
    fn receive<T: std::io::Read>(stream: &mut T) -> crate::DFHackResult<Self>
    where
        Self: Sized,
    {
        log::trace!("Receiving handshake");
        let mut magic = [0_u8; 8];
        stream.read_exact(&mut magic)?;

        let version = i32::receive(stream)?;
        let handshake = Self::new(String::from_utf8(magic.to_vec()).unwrap(), version);
        log::trace!("Received {}", handshake);
        Ok(handshake)
    }
}

impl Header {
    pub fn new(id: i16, size: i32) -> Self {
        Header {
            id,
            size,
            padding: 0,
        }
    }
}

impl Send for Header {
    fn send<T: std::io::Write>(&self, stream: &mut T) -> crate::DFHackResult<()> {
        log::trace!("Sending {}", self);
        self.id.send(stream)?;
        self.padding.send(stream)?;
        self.size.send(stream)?;
        log::trace!("Sent {}", self);
        Ok(())
    }
}

impl Receive for Header {
    fn receive<T: std::io::Read>(stream: &mut T) -> crate::DFHackResult<Self>
    where
        Self: Sized,
    {
        log::trace!("Receiving header");
        let header = Header {
            id: i16::receive(stream)?,
            padding: i16::receive(stream)?,
            size: i32::receive(stream)?,
        };
        log::trace!("Received {}", header);
        Ok(header)
    }
}

impl<TMessage: protobuf::Message> Request<TMessage> {
    pub fn new(id: i16, message: TMessage) -> Self {
        Self { id, message }
    }
}

impl<TMessage: protobuf::Message> Send for Request<TMessage> {
    fn send<T: std::io::Write>(&self, stream: &mut T) -> crate::DFHackResult<()> {
        let mut payload: Vec<u8> = Vec::new();
        self.message.write_to_vec(&mut payload)?;
        let header = Header::new(self.id, payload.len() as i32);

        log::trace!(
            "Sending protobuf message {}",
            TMessage::descriptor_static().full_name()
        );
        header.send(stream)?;
        stream.write(&payload)?;
        log::trace!("Sent protobuf message");
        Ok(())
    }
}

impl<TMessage: protobuf::Message> Receive for Reply<TMessage> {
    fn receive<T: std::io::Read>(stream: &mut T) -> crate::DFHackResult<Self>
    where
        Self: Sized,
    {
        let header = Header::receive(stream)?;

        log::trace!(
            "Receiving protobuf message {}",
            TMessage::descriptor_static().full_name()
        );

        let mut buf = vec![0u8; header.size as usize];

        stream.read_exact(&mut buf)?;

        let reply_code = RpcReplyCode::try_from(header.id)?;

        match reply_code {
            RpcReplyCode::Result => {
                log::trace!("Received result");
                let reply = TMessage::parse_from_bytes(&buf)?;
                log::trace!("Received {}", reply.descriptor().full_name());
                Ok(Reply::Result(reply))
            }
            RpcReplyCode::Fail => {
                log::trace!("Received failure");
                // TODO remove these unwraps
                let code_bytes: [u8; 4] = buf.try_into().unwrap();
                let code: i32 = i32::from_le_bytes(code_bytes);

                log::trace!("RPC error {}", code);
                let res = CommandResult::try_from(code).unwrap();
                Ok(Reply::Failure(res))
            }
            RpcReplyCode::Text => {
                log::trace!("Received text");
                let reply = crate::messages::CoreTextNotification::parse_from_bytes(&buf)?;
                Ok(Reply::Text(reply))
            }
            RpcReplyCode::Quit => Err(DFHackError::RpcError()),
        }
    }
}

impl Quit {
    pub fn new() -> Self {
        Self {}
    }
}

impl Send for Quit {
    fn send<T: std::io::Write>(&self, stream: &mut T) -> crate::DFHackResult<()> {
        log::trace!("Sending {}", self);
        let header = Header::new(RpcReplyCode::Quit as i16, 0);
        header.send(stream)?;
        log::trace!("Sent {}", self);
        Ok(())
    }
}

impl Send for i16 {
    fn send<T: std::io::Write>(&self, stream: &mut T) -> crate::DFHackResult<()> {
        stream.write_i16::<LittleEndian>(*self)?;
        Ok(())
    }
}

impl Send for i32 {
    fn send<T: std::io::Write>(&self, stream: &mut T) -> crate::DFHackResult<()> {
        stream.write_i32::<LittleEndian>(*self)?;
        Ok(())
    }
}

impl Receive for i16 {
    fn receive<T: std::io::Read>(stream: &mut T) -> crate::DFHackResult<Self>
    where
        Self: Sized,
    {
        Ok(stream.read_i16::<LittleEndian>()?)
    }
}

impl Receive for i32 {
    fn receive<T: std::io::Read>(stream: &mut T) -> crate::DFHackResult<Self>
    where
        Self: Sized,
    {
        Ok(stream.read_i32::<LittleEndian>()?)
    }
}
