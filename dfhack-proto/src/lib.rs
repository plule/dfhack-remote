#![warn(missing_docs)]
#![doc = include_str!("../README.md")]

/// Raw protobuf messages
pub mod messages {
    pub use crate::generated::messages::*;
}

/// Plugins exposing the feature of the DFHack remote API.
///
/// This module is auto-generated from DFHack sources.
pub mod plugins {
    pub use crate::generated::plugins::*;
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

/// Generated code from this crate
mod generated {
    pub mod messages;
    pub mod plugins;
}
