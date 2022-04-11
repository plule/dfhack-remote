use std::fmt;
use std::io::{Read, Write};

use protobuf::Message;

pub mod protos;

pub struct DfClient {
    stream: std::net::TcpStream,
}

#[derive(Copy, Clone, PartialEq)]
enum DFHackReplyCode {
    RpcReplyResult = -1,
    RpcReplyFail = -2,
    RpcReplyText = -3,
}

const RPC_REQUEST_QUIT: i16 = -4;

const MAGIC_QUERY: &str = "DFHack?\n";
const MAGIC_REPLY: &str = "DFHack!\n";
const VERSION: i32 = 1;

const BIND_METHOD_ID: i16 = 0;
const RUN_COMMAND_ID: i16 = 1;

impl DfClient {
    pub fn connect() -> Result<DfClient> {
        let mut client = DfClient {
            stream: std::net::TcpStream::connect("127.0.0.1:5000")?,
        };

        client.stream.write(&DfClient::handshake_request())?;

        let mut handshake_reply = [0_u8; 12];
        client.stream.read_exact(&mut handshake_reply)?;
        DfClient::check_handshake_reply(&handshake_reply.to_vec())?;

        Ok(client)
    }

    fn handshake_request() -> Vec<u8> {
        let mut handshake_request = MAGIC_QUERY.as_bytes().to_owned();
        handshake_request.append(&mut VERSION.to_le_bytes().to_vec());
        handshake_request
    }

    fn check_handshake_reply(reply: &Vec<u8>) -> Result<()> {
        let magic = String::from_utf8(reply[0..8].to_vec()).unwrap();
        if magic != MAGIC_REPLY {
            return Err(DfRemoteError::BadMagicFailure(magic));
        }
        let version = i32::from_le_bytes(reply[8..12].try_into().unwrap());
        if version != 1 {
            return Err(DfRemoteError::BadVersionFailure(version));
        }
        Ok(())
    }

    fn request<TIN: protobuf::Message, TOUT: protobuf::Message>(
        &mut self,
        id: i16,
        message: TIN,
    ) -> Result<TOUT> {
        let mut payload: Vec<u8> = Vec::new();
        message.write_to_vec(&mut payload)?;
        self.send(id, &payload)?;

        loop {
            let (reply_code, size) = self.read_header()?;

            match reply_code {
                DFHackReplyCode::RpcReplyResult => {
                    let mut buf = vec![0u8; size as usize];
                    self.stream.read_exact(&mut buf)?;
                    let reply = TOUT::parse_from_bytes(&buf)?;
                    return Ok(reply);
                }
                DFHackReplyCode::RpcReplyFail => return Err(DfRemoteError::RpcError()),
                DFHackReplyCode::RpcReplyText => {
                    let mut buf = vec![0u8; size as usize];
                    self.stream.read_exact(&mut buf)?;
                    let reply = protos::CoreProtocol::CoreTextNotification::parse_from_bytes(&buf)?;
                    for fragment in reply.get_fragments() {
                        println!("{}", fragment.get_text());
                    }
                }
            }
        }
    }

    fn send(&mut self, id: i16, data: &Vec<u8>) -> Result<()> {
        let header = DfClient::header(id, data.len() as i32);
        self.stream.write(&header)?;
        self.stream.write(data)?;
        Ok(())
    }

    fn header(id: i16, size: i32) -> Vec<u8> {
        let mut payload = Vec::new();
        let padding: i16 = 0;

        payload.append(&mut id.to_le_bytes().to_vec());
        payload.append(&mut padding.to_le_bytes().to_vec());
        payload.append(&mut size.to_le_bytes().to_vec());
        payload
    }

    fn read_header(&mut self) -> Result<(DFHackReplyCode, i32)> {
        let mut id_bytes = [0_u8; 2];
        self.stream.read_exact(&mut id_bytes)?;
        let id = i16::from_le_bytes(id_bytes);

        let mut padding_bytes = [0_u8; 2];
        self.stream.read_exact(&mut padding_bytes)?;

        let mut size_bytes = [0_u8; 4];
        self.stream.read_exact(&mut size_bytes)?;
        let size = i32::from_le_bytes(size_bytes);

        let reply = match id {
            -1 => DFHackReplyCode::RpcReplyResult,
            -2 => DFHackReplyCode::RpcReplyFail,
            -3 => DFHackReplyCode::RpcReplyText,
            _ => return Err(DfRemoteError::UnknownReplyCode(id)),
        };

        Ok((reply, size))
    }

    fn bind_method<TIN: protobuf::Message, TOUT: protobuf::Message>(
        &mut self,
        method: String,
        plugin: String,
    ) -> Result<i16> {
        let mut request = protos::CoreProtocol::CoreBindRequest::new();
        let input_msg = TIN::descriptor_static().full_name();
        let output_msg = TOUT::descriptor_static().full_name();
        request.set_method(method);
        request.set_input_msg(input_msg.to_string());
        request.set_output_msg(output_msg.to_string());
        request.set_plugin(plugin);
        let reply: protos::CoreProtocol::CoreBindReply =
            self.request(BIND_METHOD_ID, request).unwrap();
        Ok(reply.get_assigned_id() as i16)
    }
}

impl Drop for DfClient {
    fn drop(&mut self) {
        let res = self.send(RPC_REQUEST_QUIT, &vec![]);
        if let Err(failure) = res {
            println!(
                "Warning: failed to close the connection to dfhack-remote: {}",
                failure
            );
        }
    }
}

pub type Result<T> = std::result::Result<T, DfRemoteError>;

#[derive(Debug)]
pub enum DfRemoteError {
    CommunicationFailure(std::io::Error),
    BadMagicFailure(String),
    BadVersionFailure(i32),
    ProtobufError(protobuf::ProtobufError),
    UnknownReplyCode(i16),
    RpcError(),
}

impl fmt::Display for DfRemoteError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        match self {
            DfRemoteError::BadMagicFailure(magic) => {
                write!(f, "Handshake failed: bad magic {magic}.")
            }
            DfRemoteError::BadVersionFailure(version) => {
                write!(f, "Handshake failed: unsupported version {version}.")
            }
            DfRemoteError::CommunicationFailure(error) => {
                write!(f, "Communication failure: {error}")
            }
            DfRemoteError::ProtobufError(error) => {
                write!(f, "Protobuf error: {error}")
            }
            DfRemoteError::UnknownReplyCode(code) => {
                write!(f, "Unknown DFHack reply code: {code}")
            }
            DfRemoteError::RpcError() => {
                write!(f, "RPC Error")
            }
        }
    }
}

impl From<std::io::Error> for DfRemoteError {
    fn from(err: std::io::Error) -> Self {
        Self::CommunicationFailure(err)
    }
}

impl From<protobuf::ProtobufError> for DfRemoteError {
    fn from(err: protobuf::ProtobufError) -> Self {
        Self::ProtobufError(err)
    }
}

#[cfg(test)]
mod tests {
    use crate::DfClient;

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn connect() {
        let mut client = DfClient::connect().unwrap();
        let id = client
            .bind_method::<crate::protos::CoreProtocol::EmptyMessage, crate::protos::BasicApi::GetWorldInfoOut>(
                "GetWorldInfo".to_string(),
                "".to_string(),
            )
            .unwrap();
        let request = crate::protos::CoreProtocol::EmptyMessage::new();
        let world_info: crate::protos::BasicApi::GetWorldInfoOut =
            client.request(id, request).unwrap();

        println!(
            "Welcome to {}",
            world_info.get_world_name().get_english_name()
        );
        assert!(id > 0);
    }
}
