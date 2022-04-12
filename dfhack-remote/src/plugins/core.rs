use std::{cell::RefCell, rc::Rc};

use dfhack_proto::{BasicApi::*, CoreProtocol::*};

use crate::protocol::Protocol;

pub struct Core {
    pub client: Rc<RefCell<Protocol>>,
    pub name: String,
}

impl Core {
    pub fn new(client: Rc<RefCell<Protocol>>) -> Self {
        Self {
            client,
            name: "".to_string(),
        }
    }
    super::make_plugin_getter_request!(get_version, "GetVersion", StringMessage);
    super::make_plugin_getter_request!(get_df_version, "GetDFVersion", StringMessage);
    super::make_plugin_getter_request!(get_world_info, "GetWorldInfo", GetWorldInfoOut);
    super::make_plugin_getter_request!(list_enums, "ListEnums", ListEnumsOut);
    super::make_plugin_getter_request!(list_job_skills, "ListJobSkills", ListJobSkillsOut);
    super::make_plugin_getter_request!(core_suspends, "CoreSuspend", IntMessage);
    super::make_plugin_getter_request!(core_resume, "CoreResume", IntMessage);
    super::make_plugin_request!(
        list_materials,
        "ListMaterials",
        ListMaterialsIn,
        ListMaterialsOut
    );
    super::make_plugin_request!(list_units, "ListUnits", ListUnitsIn, ListUnitsOut);
    super::make_plugin_request!(list_squads, "ListSquads", ListSquadsIn, ListSquadsOut);
    super::make_plugin_request!(
        set_unit_labors,
        "SetUnitLabors",
        SetUnitLaborsIn,
        EmptyMessage
    );
    super::make_plugin_request!(bind_method, "BindMethod", CoreBindRequest, CoreBindReply);
    super::make_plugin_request!(
        run_command,
        "RunCommand",
        CoreRunCommandRequest,
        EmptyMessage
    );
    super::make_plugin_request!(run_lua, "RunLua", CoreRunLuaRequest, StringListMessage);
}
