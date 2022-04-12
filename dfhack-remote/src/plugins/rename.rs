use std::{cell::RefCell, rc::Rc};

use dfhack_proto::{rename::*, CoreProtocol::*};

use crate::protocol::Protocol;

pub struct Rename {
    pub client: Rc<RefCell<Protocol>>,
    pub name: String,
}

impl Rename {
    pub fn new(client: Rc<RefCell<Protocol>>) -> Self {
        Self {
            client,
            name: "rename".to_string(),
        }
    }
    super::make_plugin_request!(rename_squad, "RenameSquad", RenameSquadIn, EmptyMessage);
    super::make_plugin_request!(rename_unit, "RenameUnit", RenameUnitIn, EmptyMessage);
    super::make_plugin_request!(
        rename_building,
        "RenameBuilding",
        RenameBuildingIn,
        EmptyMessage
    );
}
