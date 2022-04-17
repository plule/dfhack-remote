/// Generated code from this crate
mod generated {
    pub mod messages;
}

/// Raw protobuf messages
pub mod messages {
    pub use crate::generated::messages::*;
}

/// Ability to exchange protobuf messages with DFHack
pub trait DFHackRequest<TError> {
    fn request<TRequest: protobuf::Message, TReply: protobuf::Message>(
        &mut self,
        plugin: String,
        name: String,
        request: TRequest,
    ) -> Result<TReply, TError>;
}
