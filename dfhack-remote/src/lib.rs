#![warn(missing_docs)]
#![doc(test(no_crate_inject))]
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
//!
//! The code is split into three crates:
//!
//! - `dfhack_proto_srcs` downloads and extract the proto from the DFHack source code at build time.
//! - `dfhack_proto` builds the protobuf messages and plugin structs containing the RPC
//! - `dfhack_remote` is the main entrypoint. It implements the actual protocol and exposes the features.
//!
//! Internally, the `message` module handles the serialization/deserialization logic that sits on top of protobuf,
//! and the `protocol` module handles the exchange flow.
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
mod message;
mod protocol;

/// Protobuf messages exchange as input and output of all the DFHack remote API.
///
/// This module is auto-generated from DFHack sources. It is reexported from the
/// `dfhack_proto`.
pub mod messages {
    pub use dfhack_proto::messages::*;
}
/// Plugins exposing the feature of the DFHack remote API.
///
/// This module is auto-generated from DFHack sources. It is reexported from the
/// `dfhack_proto`.
pub mod plugins {
    pub use dfhack_proto::plugins::*;
}

/// Connect to Dwarf Fortress using the default settings
///
/// It will try to connect to `127.0.0.1:5000`, DFHack default address.
/// The port can be overriden with `DFHACK_PORT`, which is also taken in account by DFHack.
///
/// For remote connexion, see [connect_to].
///
/// # Examples
///
/// ```no_run
/// use dfhack_remote;
///
/// let mut dfhack = dfhack_remote::connect().unwrap();
/// let df_version = dfhack.core.get_df_version().unwrap();
/// println!("DwarfFortress {}",  df_version.get_value());
/// ```
pub fn connect() -> DFHackResult<plugins::Plugins<protocol::Protocol, DFHackError>> {
    let connexion = protocol::Protocol::connect()?;
    Ok(plugins::Plugins::from(connexion))
}

/// Connect to Dwarf Fortress with a given address
///
/// # Arguments
///
/// * `address` - Address of the DFHack server. By default, DFHack runs of `127.0.0.1:5000`
///
/// # Examples
///
/// ```no_run
/// use dfhack_remote;
/// let mut dfhack = dfhack_remote::connect_to("127.0.0.1:5000").unwrap();
/// let df_version = dfhack.core.get_df_version().unwrap();
/// println!("DwarfFortress {}",  df_version.get_value());
/// ```
///
pub fn connect_to(
    address: &str,
) -> DFHackResult<plugins::Plugins<protocol::Protocol, DFHackError>> {
    let connexion = protocol::Protocol::connect_to(address)?;
    Ok(plugins::Plugins::from(connexion))
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
