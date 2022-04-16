#![warn(missing_docs)]
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
//! ## Examples
//!
//! Displaying some information about the current world.
//!
//! ```no_run
#![doc = include_str!("../examples/display_world_info.rs")]
//! ```
//!
//! Pausing or unpausing the game
//! ``` no_run
#![doc = include_str!("../examples/pause_unpause.rs")]
//! ```
//!
//! The generated API is mostly a direct translation from the raw RPC,
//! and as such is quite verbose.
//!
//! It includes some minor syntaxic sugar such as omitting `EmptyMessage` inputs.
//!
//! ## The DFHack API
//!
//! The DFHack [remote API] relies on protobuf for serializing messages.
//! This means that all the input and output of each message is relying on protobuf code generation to create
//! a type-safe experience.
//!
//! `dfhack_remote` is using the crates [protobuf] and [protobuf-codegen-pure] for the protobuf code generation.
//!
//! However, DFHack is not using `gRPC` for representing the remote calls, but a specific protocol.
//! The definitions of RPC of DFHack is described with comments inserted in the `.proto` files. In order
//! to bind all the RPC, this crates is generating all the API entrypoints directly from the [DFHack] source code.
//!
//! ### Building for a different DFHack version
//!
//! The DFHack source used for code generation can be controlled at build time using the `DFHACK_ZIP_URL`
//! environment variable. For example, setting this environment variable to `https://github.com/DFHack/dfhack/archive/refs/heads/develop.zip`
//! at build time would target the latest changes included in DFHack.
//!
//!
//! ## Crate structure
//!
//! This crate main entrypoint is the [crate::DFHack] structure.
//! The generated code is located in the [messages] module for the protobuf message that are exchanged,
//! and in the [plugins] module for the methods that can be called.
//!
//! Internally, all the generated code is in the `generated` module. The `message` module handles the
//! serialization/deserialization logic that sits on top of protobuf, and the `protocol` module handles
//! the exchange flow.
//!
//! Lastly, `dfhack_remote` is relying on `dfhack_proto_srcs` to download and extract the proto from the DFHack source code.
//!
//! ## Testing
//!
//! Most of the tests require the ability to communicate with Dwarf Fortress.
//! This can be enabled by setting the environment variable `DF_EXE` to the Dwarf Fortress executable.
//! Once this is setup, you can run these tests with the `test-with-df` feature enabled:
//!
//! ```txt
//! cargo test --features test-with-df
//! ```
//!
//! This will run the first save of this Dwarf Fortress installation. You should prepare a dedicated save with a pocket world for that purpose.
//!
//!
//! [Dwarf Fortress]: http://www.bay12games.com/dwarves/
//! [DFHack]: https://docs.dfhack.org/en/stable/
//! [remote API]: https://docs.dfhack.org/en/stable/docs/Remote.html
//! [protobuf]: https://crates.io/crates/protobuf
//! [protobuf-codegen-pure]: https://crates.io/crates/protobuf-codegen-pure
use protocol::Protocol;
use std::{cell::RefCell, rc::Rc};

mod message;
mod protocol;
mod generated {
    pub mod messages;
    pub mod plugins;
}

/// Protobuf messages exchange as input and output of all the DFHack remote API.
///
/// This module is auto-generated from DFHack sources.
pub mod messages {
    pub use crate::generated::messages::*;
}

/// Plugins exposing the feature of the DFHack remote API.
///
/// This module is auto-generated from DFHack sources.
pub mod plugins {
    pub use crate::generated::plugins::*;

    /// Macro generating a request
    ///
    /// This macro assumes that it is invoked in the implementation of a plugin
    /// containing a `name` attribute, and a `client` attribute.
    macro_rules! make_plugin_request {
        (
            $(#[$meta:meta])*
            $func_name:ident, $method_name:literal, EmptyMessage, $response_type:path
        ) => {
            $(#[$meta])*
            pub fn $func_name(&mut self) -> crate::DFHackResult<$response_type> {
                let request = crate::generated::messages::EmptyMessage::new();
                self.client.borrow_mut().request(
                    self.name.to_string(),
                    $method_name.to_string(),
                    request,
                )
            }
        };
        (
            $(#[$meta:meta])*
            $func_name:ident, $method_name:literal, $request_type:path, $response_type:path
        ) => {
            $(#[$meta])*
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

/// Result type emitted by DFHack API calls
pub type DFHackResult<T> = std::result::Result<T, DFHackError>;

/// Error type emitted by DFHack API calls
#[derive(Debug)]
pub enum DFHackError {
    /// A low level connexion error
    ///
    /// This can mean that the address is wrong,
    /// that Dwarf Fortress crashed, or a library bug occured.
    CommunicationFailure(std::io::Error),

    /// Failure of the handshake with DFHack
    ///
    /// This can mean that the software is not DFHack
    BadMagicFailure(String),

    /// Bad version during the handshake with DFHack
    ///
    /// This can mean that the DFHack protocol was updated
    /// and is not compatible with the version of this library
    BadVersionFailure(i32),

    /// Protobuf serialization or deserialization error
    ProtobufError(protobuf::ProtobufError),

    /// Unknown reply code during the exchange
    UnknownReplyCode(i16),

    /// DFHack RPC Error
    RpcError(),
}

/// Main entrypoint to the DFHack API
///
/// This structure holds an instance of each exposed plugin,
/// ready to communicate with Dwarf Fortress.
pub struct DFHack {
    /// The core plugin exposes the base of the API
    pub core: plugins::Core,

    /// Isoworld plugin
    pub isoworld: plugins::Isoworldremote,

    /// Rename plugin
    pub rename: plugins::Rename,

    /// RemoteFortressReader plugin
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
    /// let mut dfhack = DFHack::connect().unwrap();
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
