#![warn(missing_docs)]
//! # dfhack_proto
//!
//! DFHack proto contains all the generated code for interacting with DFHack remote API.
//!
//! It contains two main modules:
//!
//!  - [messages] exposes the protobuf messages. This is the standard generated protobuf.
//!  - [plugins] exposes the plugins and their RPC. DFHack is not using gRPC and this is a custom implementation
//!
//! Internally these two modules are created under the `generated` module.
//!
//! All the plugins are built from a struct implementing [ProtocolTrait]. This trait
//! should implement the actual data exchange.
//!
//! This crates generates all its code directly in a source subfolder. It would likely
//! be cleaner to use the $OUT_DIR, or macro to do this job.
//!

/// Generated code from this crate
mod generated {
    pub mod messages;
    pub mod plugins;
}

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

/// Ability to exchange protobuf messages with DFHack
pub trait ProtocolTrait<TError> {
    /// Send a request to DFHack
    ///
    /// # Errors
    ///
    /// The error type is defined by the protocol implementation
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
