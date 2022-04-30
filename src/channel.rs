//! Internal module describing the exchange flow
//!
//! Implements the flow of sending and receiving messages.
//! This includes the custom workflow of binding RPC methods
//! before being able to use them.

use std::{collections::HashMap, fmt};

use crate::{
    message::{self, Receive, Send},
    Error,
};
use num_enum::TryFromPrimitiveError;

#[derive(PartialEq, Eq, Hash, Clone)]
pub struct Method {
    pub plugin: String,
    pub name: String,
}

impl Method {
    fn new(plugin: String, name: String) -> Self {
        Method { plugin, name }
    }
}

pub struct Channel {
    stream: std::net::TcpStream,
    bindings: HashMap<Method, i16>,
}

const MAGIC_QUERY: &str = "DFHack?\n";
const MAGIC_REPLY: &str = "DFHack!\n";
const VERSION: i32 = 1;

const BIND_METHOD_ID: i16 = 0;
const RUN_COMMAND_ID: i16 = 1;

impl dfhack_proto::Channel for Channel {
    type TError = crate::Error;

    fn request<TRequest, TReply>(
        &mut self,
        plugin: std::string::String,
        name: std::string::String,
        request: TRequest,
    ) -> crate::Result<TReply>
    where
        TRequest: protobuf::Message,
        TReply: protobuf::Message,
    {
        let method = Method::new(plugin, name);

        // did not manage to use the entry api due to borrow checker
        let maybe_id = self.bindings.get(&method);

        let id = match maybe_id {
            Some(id) => *id,
            None => {
                let id = self.bind_method::<TRequest, TReply>(&method)?;
                self.bindings.insert(method, id);
                id
            }
        };

        self.request_raw(id, request)
    }
}

impl Channel {
    pub fn connect() -> crate::Result<Self> {
        let port = match std::env::var("DFHACK_PORT") {
            Ok(p) => p,
            Err(_) => "5000".to_string(),
        };
        Self::connect_to(&format!("127.0.0.1:{}", port))
    }

    pub fn connect_to(address: &str) -> crate::Result<Channel> {
        log::info!("Connecting to {}", address);
        let mut client = Channel {
            stream: std::net::TcpStream::connect(address)?,
            bindings: HashMap::new(),
        };

        client.bindings.insert(
            Method::new("".to_string(), "BindMethod".to_string()),
            BIND_METHOD_ID,
        );
        client.bindings.insert(
            Method::new("".to_string(), "RunCommand".to_string()),
            RUN_COMMAND_ID,
        );

        let handshake_request = message::Handshake::new(MAGIC_QUERY.to_string(), VERSION);
        handshake_request.send(&mut client.stream)?;
        let handshake_reply = message::Handshake::receive(&mut client.stream)?;

        if handshake_reply.magic != MAGIC_REPLY {
            return Err(Error::ProtocolError(format!(
                "Unexpected magic {}",
                handshake_reply.magic
            )));
        }

        if handshake_reply.version != VERSION {
            return Err(Error::ProtocolError(format!(
                "Unexpected magic version {}",
                handshake_reply.version
            )));
        }

        Ok(client)
    }

    pub fn request_raw<TIN: protobuf::Message, TOUT: protobuf::Message>(
        &mut self,
        id: i16,
        message: TIN,
    ) -> crate::Result<TOUT> {
        let request = message::Request::new(id, message);
        request.send(&mut self.stream)?;

        loop {
            let reply: message::Reply<TOUT> = message::Reply::receive(&mut self.stream)?;
            match reply {
                message::Reply::Text(text) => {
                    for fragment in text.get_fragments() {
                        println!("{}", fragment.get_text());
                    }
                }
                message::Reply::Result(result) => return Ok(result),
                message::Reply::Fail(command_result) => {
                    return Err(Error::RpcError(command_result))
                }
            }
        }
    }

    pub fn bind_method<TIN: protobuf::Message, TOUT: protobuf::Message>(
        &mut self,
        method: &Method,
    ) -> crate::Result<i16> {
        let input_msg = TIN::descriptor_static().full_name();
        let output_msg = TOUT::descriptor_static().full_name();
        self.bind_method_by_name(&method.plugin, &method.name, input_msg, output_msg)
    }

    pub fn bind_method_by_name(
        &mut self,
        plugin: &str,
        method: &str,
        input_msg: &str,
        output_msg: &str,
    ) -> crate::Result<i16> {
        log::debug!("Binding the method {}:{}", plugin, method);
        let mut request = crate::CoreBindRequest::new();
        request.set_method(method.to_owned());
        request.set_input_msg(input_msg.to_string());
        request.set_output_msg(output_msg.to_string());
        request.set_plugin(plugin.to_owned());
        let reply: crate::CoreBindReply = match self.request_raw(BIND_METHOD_ID, request) {
            Ok(reply) => reply,
            Err(_) => {
                log::error!("Error attempting to bind {}", method);
                return Err(Error::FailedToBind(format!(
                    "{}::{} ({}->{})",
                    plugin, method, input_msg, output_msg,
                )));
            }
        };
        let id = reply.get_assigned_id() as i16;
        log::debug!("{}:{} bound to {}", plugin, method, id);
        Ok(id)
    }
}

impl Drop for Channel {
    fn drop(&mut self) {
        let quit = message::Quit::new();
        let res = quit.send(&mut self.stream);
        if let Err(failure) = res {
            println!(
                "Warning: failed to close the connection to dfhack-remote: {}",
                failure
            );
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        match self {
            Error::ProtocolError(message) => {
                write!(f, "Protocol Error: {message}.")
            }
            Error::CommunicationFailure(error) => {
                write!(f, "Communication failure: {error}")
            }
            Error::ProtobufError(error) => {
                write!(f, "Protobuf error: {error}")
            }
            Error::RpcError(result) => {
                write!(f, "Command result error: {result}")
            }
            Error::FailedToBind(method) => write!(f, "Failed to bind {}", method),
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Self::CommunicationFailure(err)
    }
}

impl From<protobuf::ProtobufError> for Error {
    fn from(err: protobuf::ProtobufError) -> Self {
        Self::ProtobufError(err)
    }
}

impl From<TryFromPrimitiveError<message::RpcReplyCode>> for Error {
    fn from(err: TryFromPrimitiveError<message::RpcReplyCode>) -> Self {
        Self::ProtocolError(format!("Unknown DFHackReplyCode : {}", err.number))
    }
}

impl From<std::string::FromUtf8Error> for Error {
    fn from(err: std::string::FromUtf8Error) -> Self {
        Self::ProtocolError(format!("Invalid string error: {}", err))
    }
}

#[cfg(test)]
mod tests {
    #[cfg(feature = "test-with-df")]
    mod withdf {
        use crate::Error;

        #[test]
        fn bind() {
            use crate::channel::Channel;
            let mut channel = Channel::connect().unwrap();

            channel
                .bind_method_by_name(
                    "",
                    "GetVersion",
                    "dfproto.EmptyMessage",
                    "dfproto.StringMessage",
                )
                .unwrap();
        }

        #[test]
        fn bad_bind() {
            use crate::channel::Channel;
            let mut channel = Channel::connect().unwrap();

            let err = channel
                .bind_method_by_name(
                    "",
                    "GetVersion",
                    "dfproto.EmptyMessage",
                    "dfproto.EmptyMessage",
                )
                .unwrap_err();
            assert!(std::matches!(err, Error::FailedToBind(_)));

            let err = channel
                .bind_method_by_name(
                    "dorf",
                    "GetVersion",
                    "dfproto.StringMessage",
                    "dfproto.EmptyMessage",
                )
                .unwrap_err();
            assert!(std::matches!(err, Error::FailedToBind(_)));
        }

        #[test]
        #[cfg(feature = "reflection")]
        fn bind_all() {
            use dfhack_proto::{reflection::StubReflection, stubs::Stubs};

            use crate::channel::Channel;
            let mut channel = Channel::connect().unwrap();
            let methods = Stubs::<Channel>::list_methods();

            for method in &methods {
                channel
                    .bind_method_by_name(
                        &method.plugin_name,
                        &method.name,
                        &method.input_type,
                        &method.output_type,
                    )
                    .unwrap();
            }
        }
    }
}
