mod core;
mod isoworld;
mod remote_fortress_reader;
mod rename;

pub use self::core::Core;
pub use self::isoworld::Isoworld;
pub use self::remote_fortress_reader::RemoteFortressReader;
pub use self::rename::Rename;

macro_rules! make_plugin_request {
    ($func_name:ident, $method_name:literal, EmptyMessage, $response_type:path) => {
        pub fn $func_name(&mut self) -> crate::DFHackResult<$response_type> {
            let request = crate::CoreProtocol::EmptyMessage::new();
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
