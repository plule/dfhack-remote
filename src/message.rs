use crate::{protos, DfRemoteError};
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use num_enum::TryFromPrimitive;
use protobuf::Message;
use std::convert::TryFrom;

pub trait Send {
    fn send<T: std::io::Write>(&self, stream: &mut T) -> crate::Result<()>;
}

pub trait Receive {
    fn receive<T: std::io::Read>(stream: &mut T) -> crate::Result<Self>
    where
        Self: Sized;
}

// https://github.com/DFHack/dfhack/blob/0.47.05-r4/library/include/RemoteClient.h#L53
#[derive(Copy, Clone, PartialEq, TryFromPrimitive)]
#[repr(i16)]
pub enum RpcReplyCode {
    Result = -1,
    Fail = -2,
    Text = -3,
    Quit = -4,
}

// https://github.com/DFHack/dfhack/blob/0.47.05-r4/library/include/RemoteClient.h#L42
#[derive(Copy, Clone, PartialEq, TryFromPrimitive)]
#[repr(i32)]
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
pub struct Handshake {
    pub magic: String,
    pub version: i32,
}

// https://docs.dfhack.org/en/stable/docs/Remote.html#header
pub struct Header {
    pub id: i16,
    padding: i16,
    pub size: i32,
}

// https://docs.dfhack.org/en/stable/docs/Remote.html#request
pub struct Request<TMessage: protobuf::Message> {
    pub id: i16,
    pub message: TMessage,
}

pub enum Reply<TMessage: protobuf::Message> {
    // https://docs.dfhack.org/en/stable/docs/Remote.html#text
    Text(protos::CoreProtocol::CoreTextNotification),

    // https://docs.dfhack.org/en/stable/docs/Remote.html#result
    Result(TMessage),

    // https://docs.dfhack.org/en/stable/docs/Remote.html#failure
    Failure(CommandResult),
}

// https://docs.dfhack.org/en/stable/docs/Remote.html#quit
pub struct Quit {}

impl Handshake {
    pub fn new(magic: String, version: i32) -> Self {
        Handshake { magic, version }
    }
}

impl Send for Handshake {
    fn send<T: std::io::Write>(&self, stream: &mut T) -> crate::Result<()> {
        stream.write(self.magic.as_bytes())?;
        stream.write_i32::<LittleEndian>(self.version)?;
        Ok(())
    }
}

impl Receive for Handshake {
    fn receive<T: std::io::Read>(stream: &mut T) -> crate::Result<Self>
    where
        Self: Sized,
    {
        let mut magic = [0_u8; 8];
        stream.read_exact(&mut magic)?;

        let version = stream.read_i32::<LittleEndian>()?;

        Ok(Self::new(
            String::from_utf8(magic.to_vec()).unwrap(),
            version,
        ))
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
    fn send<T: std::io::Write>(&self, stream: &mut T) -> crate::Result<()> {
        stream.write_i16::<LittleEndian>(self.id)?;
        stream.write_i16::<LittleEndian>(self.padding)?;
        stream.write_i32::<LittleEndian>(self.size)?;
        Ok(())
    }
}

impl Receive for Header {
    fn receive<T: std::io::Read>(stream: &mut T) -> crate::Result<Self>
    where
        Self: Sized,
    {
        Ok(Header {
            id: stream.read_i16::<LittleEndian>()?,
            padding: stream.read_i16::<LittleEndian>()?,
            size: stream.read_i32::<LittleEndian>()?,
        })
    }
}

impl<TMessage: protobuf::Message> Request<TMessage> {
    pub fn new(id: i16, message: TMessage) -> Self {
        Self { id, message }
    }
}

impl<TMessage: protobuf::Message> Send for Request<TMessage> {
    fn send<T: std::io::Write>(&self, stream: &mut T) -> crate::Result<()> {
        let mut payload: Vec<u8> = Vec::new();
        self.message.write_to_vec(&mut payload)?;
        let header = Header::new(self.id, payload.len() as i32);

        header.send(stream)?;
        stream.write(&payload)?;
        Ok(())
    }
}

impl<TMessage: protobuf::Message> Receive for Reply<TMessage> {
    fn receive<T: std::io::Read>(stream: &mut T) -> crate::Result<Self>
    where
        Self: Sized,
    {
        let header = Header::receive(stream)?;

        let mut buf = vec![0u8; header.size as usize];

        if header.size > 1 {
            // TODO for errors, dfhack would return
            // size of 1 with no data but can't read it
            stream.read_exact(&mut buf)?;
        } else {
            // assign it to CR_ERROR before understanding better
            buf = 1i32.to_le_bytes().to_vec();
        }

        let reply_code = RpcReplyCode::try_from(header.id)?;

        match reply_code {
            RpcReplyCode::Result => {
                let reply = TMessage::parse_from_bytes(&buf)?;
                Ok(Reply::Result(reply))
            }
            RpcReplyCode::Fail => {
                // TODO remove these unwraps
                let code_bytes: [u8; 4] = buf.try_into().unwrap();
                let code: i32 = i32::from_le_bytes(code_bytes);
                let res = CommandResult::try_from(code).unwrap();
                Ok(Reply::Failure(res))
            }
            RpcReplyCode::Text => {
                let reply = protos::CoreProtocol::CoreTextNotification::parse_from_bytes(&buf)?;
                Ok(Reply::Text(reply))
            }
            RpcReplyCode::Quit => Err(DfRemoteError::RpcError()),
        }
    }
}

impl Quit {
    pub fn new() -> Self {
        Self {}
    }
}

impl Send for Quit {
    fn send<T: std::io::Write>(&self, stream: &mut T) -> crate::Result<()> {
        let header = Header::new(RpcReplyCode::Quit as i16, 0);
        header.send(stream)?;
        Ok(())
    }
}
