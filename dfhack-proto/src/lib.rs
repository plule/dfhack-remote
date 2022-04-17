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

    /// Macro generating a request
    ///
    /// This macro assumes that it is invoked in the implementation of a plugin
    /// containing a `name` attribute, and a `protocol` attribute.
    macro_rules! make_plugin_request {
        (
            $(#[$meta:meta])*
            $func_name:ident, $method_name:literal, EmptyMessage, $response_type:path
        ) => {
            $(#[$meta])*
            pub fn $func_name(&mut self) -> Result<$response_type, E> {
                let request = crate::messages::EmptyMessage::new();
                self.protocol.borrow_mut().request(
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
            ) -> Result<$response_type, E> {
                self.protocol.borrow_mut().request(
                    self.name.to_string(),
                    $method_name.to_string(),
                    request,
                )
            }
        };
    }

    pub(crate) use make_plugin_request;
}

/// Ability to exchange protobuf messages with DFHack
pub trait ProtocolTrait<TError> {
    fn request<TRequest: protobuf::Message, TReply: protobuf::Message>(
        &mut self,
        plugin: String,
        name: String,
        request: TRequest,
    ) -> Result<TReply, TError>;
}
