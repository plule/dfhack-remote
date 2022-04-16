use std::{cell::RefCell, rc::Rc};

use protocol::Protocol;

mod generated {
    pub mod messages;
    pub mod plugins;
}

pub use crate::generated::messages::*;
pub mod plugins {
    pub use crate::generated::plugins::*;

    macro_rules! make_plugin_request {
        ($func_name:ident, $method_name:literal, EmptyMessage, $response_type:path) => {
            pub fn $func_name(&mut self) -> crate::DFHackResult<$response_type> {
                let request = crate::generated::messages::EmptyMessage::new();
                self.client.borrow_mut().request(
                    self.name.to_string(),
                    $method_name.to_string(),
                    request,
                )
            }
        };
        ($func_name:ident, $method_name:literal, $request_type:path, $response_type:path) => {
            pub fn $func_name(
                &mut self,
                request: $request_type,
            ) -> crate::DFHackResult<$response_type> {
                self.client.borrow_mut().request(
                    self.name.to_string(),
                    $method_name.to_string(),
                    request,
                )
            }
        };
    }

    pub(crate) use make_plugin_request;
}

mod message;
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
    pub isoworld: plugins::Isoworldremote,
    pub rename: plugins::Rename,
    pub remote_fortress_reader: plugins::RemoteFortressReader,
}

impl DFHack {
    pub fn connect() -> DFHackResult<Self> {
        let client = Protocol::connect()?;
        let client = Rc::new(RefCell::new(client));
        Ok(Self {
            core: plugins::Core::new(Rc::clone(&client)),
            isoworld: plugins::Isoworldremote::new(Rc::clone(&client)),
            rename: plugins::Rename::new(Rc::clone(&client)),
            remote_fortress_reader: plugins::RemoteFortressReader::new(Rc::clone(&client)),
        })
    }
}
