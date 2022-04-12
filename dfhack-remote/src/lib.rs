use std::{cell::RefCell, rc::Rc};

use plugins::{
    core::Core, isoworld::Isoworld, remote_fortress_reader::RemoteFortressReader, rename::Rename,
};
use protocol::Protocol;

mod message;
pub mod plugins;
mod protocol;

pub type DFHackResult<T> = std::result::Result<T, DFHackError>;

#[derive(Debug)]
pub enum DFHackError {
    CommunicationFailure(std::io::Error),
    BadMagicFailure(String),
    BadVersionFailure(i32),
    ProtobufError(protobuf::ProtobufError),
    UnknownReplyCode(i16),
    RpcError(),
}

pub struct DFHack {
    pub core: Core,
    pub isoworld: Isoworld,
    pub rename: Rename,
    pub remote_fortress_reader: RemoteFortressReader,
}

impl DFHack {
    pub fn connect() -> DFHackResult<Self> {
        let client = Protocol::connect()?;
        let client = Rc::new(RefCell::new(client));
        Ok(Self {
            core: Core::new(Rc::clone(&client)),
            isoworld: Isoworld::new(Rc::clone(&client)),
            rename: Rename::new(Rc::clone(&client)),
            remote_fortress_reader: RemoteFortressReader::new(Rc::clone(&client)),
        })
    }
}
