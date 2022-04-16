use std::{cell::RefCell, rc::Rc};

use crate::isoworldremote::*;

use crate::protocol::Protocol;

pub struct Isoworld {
    pub client: Rc<RefCell<Protocol>>,
    pub name: String,
}

impl Isoworld {
    pub fn new(client: Rc<RefCell<Protocol>>) -> Self {
        Self {
            client,
            name: "isoworldremote".to_string(),
        }
    }
    super::make_plugin_request!(get_embark_tile, "GetEmbarkTile", TileRequest, EmbarkTile);
    super::make_plugin_request!(get_embark_info, "GetEmbarkInfo", MapRequest, MapReply);
    super::make_plugin_request!(get_raw_names, "GetRawNames", MapRequest, RawNames);
}
