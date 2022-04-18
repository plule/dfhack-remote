#![warn(missing_docs)]
#![doc = include_str!("../README.md")]

mod channel;
mod message;

#[doc(no_inline)]
pub use dfhack_proto::messages::*;
pub use dfhack_proto::stubs::*;

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
pub fn connect() -> DFHackResult<Stubs<channel::DFHackChannel, DFHackError>> {
    let connexion = channel::DFHackChannel::connect()?;
    Ok(Stubs::from(connexion))
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
pub fn connect_to(address: &str) -> DFHackResult<Stubs<channel::DFHackChannel, DFHackError>> {
    let connexion = channel::DFHackChannel::connect_to(address)?;
    Ok(Stubs::from(connexion))
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

#[cfg(test)]
mod tests {
    #[ctor::ctor]
    fn init() {
        simple_logger::SimpleLogger::new()
            .with_level(log::LevelFilter::Debug)
            .init()
            .unwrap();
    }
    #[cfg(feature = "test-with-df")]
    mod withdf {
        use std::process::Child;
        use std::sync::Mutex;

        use rand::Rng;
        #[cfg(test)]
        lazy_static::lazy_static! {
            static ref DF_PROCESS: Mutex<Option<Child>> = Mutex::new(Option::<Child>::None);
        }

        #[ctor::ctor]
        fn init() {
            let port = rand::thread_rng().gen_range(49152..65535).to_string();
            std::env::set_var("DFHACK_PORT", port);

            use std::{path::PathBuf, process::Command};
            let df_exe = PathBuf::from(std::env::var("DF_EXE").unwrap());
            let df_folder = df_exe.parent().unwrap();

            let df = Command::new(&df_exe)
                .args(["+load-save", "region1"])
                .current_dir(df_folder)
                .spawn()
                .unwrap();
            let mut process_guard = DF_PROCESS.lock().unwrap();
            process_guard.replace(df);
        }

        #[ctor::dtor]
        fn exit() {
            let mut process_guard = DF_PROCESS.lock().unwrap();
            let df = process_guard.take();
            if let Some(mut df) = df {
                df.kill().unwrap();
            }
        }

        #[test]
        fn get_version() {
            let mut client = crate::connect().unwrap();
            let version = client.core.get_df_version().unwrap();
            assert!(version.len() > 0);
        }

        #[test]
        fn pause_unpause() {
            let mut client = crate::connect().unwrap();

            let initial_pause_status = client.remote_fortress_reader.get_pause_state().unwrap();

            client
                .remote_fortress_reader
                .set_pause_state(!initial_pause_status)
                .unwrap();

            let new_pause_status = client.remote_fortress_reader.get_pause_state().unwrap();

            assert!(initial_pause_status != new_pause_status);
        }
    }
}
