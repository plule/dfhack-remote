//! # dfhack_remote
//!
//! dfhack_remote is a library allowing users to interact with Dwarf Fortress using a remote API.
//!
//! [Dwarf Fortress] is a video game by Bay 12 Games.
//! [DFHack] is a fan made mod for Dwarf Fortress adding many
//! features to the game.
//! One of these feature is a [remote API] enabling
//! the remote control of certain Dwarf Fortress features.
//! This crates is a client for this remote API, enabling rust tool developers to
//! interact with Dwarf Fortress.
//!
//! [Dwarf Fortress]: http://www.bay12games.com/dwarves/
//! [DFHack]: https://docs.dfhack.org/en/stable/
//! [remote API]: https://docs.dfhack.org/en/stable/docs/Remote.html
use protocol::Protocol;
use std::{cell::RefCell, rc::Rc};

mod generated {
    pub mod messages;
    pub mod plugins;
}

/// Protobuf messages exchange as input and output of all the DFHack remote API.
pub mod messages {
    pub use crate::generated::messages::*;
}

/// Plugins exposing the feature of the DFHack remote API.
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

/// Result type emitted by DFHack API calls
pub type DFHackResult<T> = std::result::Result<T, DFHackError>;

/// Error type emitted by DFHack API calls
#[derive(Debug)]
pub enum DFHackError {
    CommunicationFailure(std::io::Error),
    BadMagicFailure(String),
    BadVersionFailure(i32),
    ProtobufError(protobuf::ProtobufError),
    UnknownReplyCode(i16),
    RpcError(),
}

/// Main entrypoint to the DFHack API
///
/// This structure holds an instance of each exposed plugin,
/// ready to communicate with Dwarf Fortress.
pub struct DFHack {
    pub core: plugins::Core,
    pub isoworld: plugins::Isoworldremote,
    pub rename: plugins::Rename,
    pub remote_fortress_reader: plugins::RemoteFortressReader,
}

impl DFHack {
    /// Connect to DFHack
    ///
    /// # Arguments
    ///
    /// * `address` - Address of the DFHack server. By default, DFHack runs of `127.0.0.1:5000`
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use dfhack_remote::DFHack;
    /// let mut dfhack = DFHack::connect_to("127.0.0.1:5000").unwrap();
    /// let df_version = dfhack.core.get_df_version().unwrap();
    /// println!("DwarfFortress {}",  df_version.get_value());
    /// ```
    ///
    pub fn connect_to(address: &str) -> DFHackResult<Self> {
        let client = Protocol::connect(address)?;
        let client = Rc::new(RefCell::new(client));
        Ok(Self {
            core: plugins::Core::new(Rc::clone(&client)),
            isoworld: plugins::Isoworldremote::new(Rc::clone(&client)),
            rename: plugins::Rename::new(Rc::clone(&client)),
            remote_fortress_reader: plugins::RemoteFortressReader::new(Rc::clone(&client)),
        })
    }

    /// Connect to Dwarf Fortress through DFHack.
    ///
    /// By default it will try to connect to `127.0.0.1:5000`, DFHack default address.
    /// The port can be overriden with DFHACK_PORT, which is also taken in account by DFHack.
    ///
    /// For remote connexion, see [DFHack::connect_to].
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use dfhack_remote::DFHack;
    ///
    /// let mut dfhack = ::DFHack::connect();
    /// let df_version = dfhack.core.get_df_version().unwrap();
    /// println!("DwarfFortress {}",  df_version.get_value());
    /// ```
    pub fn connect() -> DFHackResult<Self> {
        let port = match std::env::var("DFHACK_PORT") {
            Ok(p) => p,
            Err(_) => "5000".to_string(),
        };
        Self::connect_to(&format!("127.0.0.1:{}", port))
    }
}
