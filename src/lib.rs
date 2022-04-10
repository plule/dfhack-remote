use std::io::{Read, Write};

pub struct DfClient {
    stream: std::net::TcpStream,
}

impl DfClient {
    pub fn connect() -> std::io::Result<DfClient> {
        let mut client = DfClient {
            stream: std::net::TcpStream::connect("127.0.0.1:5000")?,
        };
        let magic_request = "DFHack?\n";
        let version: i32 = 1;

        let mut handshake_request = magic_request.as_bytes().to_owned();
        handshake_request.append(&mut version.to_le_bytes().to_vec());

        client.stream.write(&handshake_request)?;
        //client.stream.flush()?;

        let mut handshake_reply = [0_u8; 12];
        client.stream.read_exact(&mut handshake_reply)?;

        let magic_reply = String::from_utf8(handshake_reply[0..8].to_vec()).unwrap();
        assert!(magic_reply == "DFHack!\n");
        let version_reply = i32::from_le_bytes(handshake_reply[8..12].try_into().unwrap());
        assert!(version_reply == 1);

        client.stream.flush()?;
        Ok(client)
    }
}

const RPC_REPLY_RESULT: i16 = -1;
const RPC_REPLY_FAIL: i16 = -2;
const RPC_REPLY_TEXT: i16 = -3;
const RPC_REQUEST_QUIT: i16 = -4;

impl Drop for DfClient {
    fn drop(&mut self) {
        let mut payload = Vec::new();
        let id = RPC_REQUEST_QUIT;
        let padding: i16 = 0;
        let size: i32 = 0;

        payload.append(&mut id.to_le_bytes().to_vec());
        payload.append(&mut padding.to_le_bytes().to_vec());
        payload.append(&mut size.to_le_bytes().to_vec());
        self.stream.write(&payload);
        self.stream.flush();
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
