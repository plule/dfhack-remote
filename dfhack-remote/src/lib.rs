#![warn(missing_docs)]
#![doc = include_str!("../../README.md")]

mod message;
mod protocol;

#[doc(no_inline)]
pub use dfhack_proto::messages::*;
pub use dfhack_proto::plugins::*;

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
/// println!("DwarfFortress {}",  df_version);
/// ```
pub fn connect() -> DFHackResult<Plugins<protocol::Protocol, DFHackError>> {
    let connexion = protocol::Protocol::connect()?;
    Ok(Plugins::from(connexion))
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
/// println!("DwarfFortress {}",  df_version);
/// ```
///
pub fn connect_to(address: &str) -> DFHackResult<Plugins<protocol::Protocol, DFHackError>> {
    let connexion = protocol::Protocol::connect_to(address)?;
    Ok(Plugins::from(connexion))
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
