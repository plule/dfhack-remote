use std::fmt;
use std::io::{Read, Write};

pub mod protos;

pub struct DfClient {
    stream: std::net::TcpStream,
}

#[derive(Copy, Clone)]
enum DFHackReplyCode {
    RpcReplyResult = -1,
    RpcReplyFail = -2,
    RpcReplyText = -3,
    RpcRequestQuit = -4,
}

const MAGIC_QUERY: &str = "DfHack?\n";
const MAGIC_REPLY: &str = "DfHack!\n";
const VERSION: i32 = 1;

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

    fn send(&mut self, code: DFHackReplyCode, data: &Vec<u8>) -> Result<()> {
        let header = DfClient::header(code, data.len() as i32);
        self.stream.write(&header)?;
        self.stream.write(data)?;
        Ok(())
    }

    fn header(code: DFHackReplyCode, size: i32) -> Vec<u8> {
        let mut payload = Vec::new();
        let id = code as i16;
        let padding: i16 = 0;

        payload.append(&mut id.to_le_bytes().to_vec());
        payload.append(&mut padding.to_le_bytes().to_vec());
        payload.append(&mut size.to_le_bytes().to_vec());
        payload
    }
}

impl Drop for DfClient {
    fn drop(&mut self) {
        let res = self.send(DFHackReplyCode::RpcRequestQuit, &vec![]);
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
        }
    }
}

impl From<std::io::Error> for DfRemoteError {
    fn from(err: std::io::Error) -> Self {
        Self::CommunicationFailure(err)
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
        let client = DfClient::connect().unwrap();
    }
}
