use std::{cell::RefCell, rc::Rc};

use protocol::Protocol;

mod generated;

pub use generated::*;

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
    pub core: plugins::Core,
    pub isoworld: plugins::Isoworld,
    pub rename: plugins::Rename,
    pub remote_fortress_reader: plugins::RemoteFortressReader,
}

impl DFHack {
    pub fn connect() -> DFHackResult<Self> {
        let client = Protocol::connect()?;
        let client = Rc::new(RefCell::new(client));
        Ok(Self {
            core: plugins::Core::new(Rc::clone(&client)),
            isoworld: plugins::Isoworld::new(Rc::clone(&client)),
            rename: plugins::Rename::new(Rc::clone(&client)),
            remote_fortress_reader: plugins::RemoteFortressReader::new(Rc::clone(&client)),
        })
    }
}
