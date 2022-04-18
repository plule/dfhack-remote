#![warn(missing_docs)]
#![doc = include_str!("../README.md")]

/// Raw protobuf messages
pub mod messages {
    pub use crate::generated::messages::*;
}

/// Stubs exposing the feature of the DFHack remote API.
///
/// Each stub is generated from a DFHack plugin.
/// This module is auto-generated from DFHack sources.
pub mod stubs {
    pub use crate::generated::stubs::*;
}

/// The `Channel` is the low-level exchange implementation.
///
/// It is in charge to serialize/deserialize messages, and exchange
/// them with Dwarf Fortress. It is not meant to be used as is, but to be passed to
/// It is analoguous to the gRPC channel.
pub trait Channel<TError> {
    /// Send a request to DFHack, and return its reply.
    ///
    /// # Errors
    ///
    /// The error type is defined by the channel implementation
    ///
    /// # Arguments
    ///
    /// * `plugin` - Name of the plugin implementing the request. Example: "RemoteFortressReader". Empty for core messages.
    /// * `name` - Name of the method. Example: "GetDFVersion"
    /// * `request` - Input of the method.
    ///
    /// # Returns
    ///
    /// A protobuf result type.
    ///
    fn request<TRequest: protobuf::Message, TReply: protobuf::Message>(
        &mut self,
        plugin: String,
        name: String,
        request: TRequest,
    ) -> Result<TReply, TError>;
}

#[cfg(feature = "reflection")]
/// Reflection for runtime inspection of the stubs.
mod reflection {
    /// Descriptor of a remote procedure call
    ///
    /// These are all the needed information to make a call
    pub struct RemoteProcedureDescriptor {
        /// Name of the RPC
        pub name: String,

        /// Plugin implementing the RPC
        ///
        /// An empty string means the core API
        pub plugin_name: String,

        /// Input type
        ///
        /// This is the full name of the protobuf message
        pub input_type: String,

        /// Output type
        ///
        /// This is the full name of the protobuf message
        pub output_type: String,
    }

    /// Ability for a stub to list its supported methods
    ///
    /// This is mostly useful for testing purpose.
    pub trait StubReflection {
        /// List the supported remote calls
        fn list_methods() -> Vec<RemoteProcedureDescriptor>;
    }
}

/// Generated code from this crate
mod generated {
    pub mod messages;
    pub mod stubs;
}
