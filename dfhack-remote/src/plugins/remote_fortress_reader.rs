use std::{cell::RefCell, rc::Rc};

use dfhack_proto::{
    AdventureControl::*, CoreProtocol::*, DwarfControl::*, RemoteFortressReader::*,
};

use crate::protocol::Protocol;

pub struct RemoteFortressReader {
    pub client: Rc<RefCell<Protocol>>,
    pub name: String,
}

impl RemoteFortressReader {
    pub fn new(client: Rc<RefCell<Protocol>>) -> Self {
        Self {
            client,
            name: "RemoteFortressReader".to_string(),
        }
    }

    super::make_plugin_request!(set_side_menu, "SetSideMenu", SidebarCommand, EmptyMessage);
    super::make_plugin_request!(get_block_list, "GetBlockList", BlockRequest, BlockList);
    super::make_plugin_request!(get_plant_list, "GetPlantList", BlockRequest, PlantList);
    super::make_plugin_request!(
        get_unit_list_inside,
        "GetUnitListInside",
        BlockRequest,
        UnitList
    );
    super::make_plugin_request!(
        get_partial_creature_raws,
        "GetPartialCreatureRaws",
        ListRequest,
        CreatureRawList
    );
    super::make_plugin_request!(
        get_partial_plant_raws,
        "GetPartialPlantRaws",
        ListRequest,
        PlantRawList
    );
    super::make_plugin_request!(
        pass_keyboard_event,
        "PassKeyboardEvent",
        KeyboardEvent,
        EmptyMessage
    );
    super::make_plugin_request!(send_dig_command, "SendDigCommand", DigCommand, EmptyMessage);
    super::make_plugin_request!(set_pause_state, "SetPauseState", SingleBool, EmptyMessage);
    super::make_plugin_request!(move_command, "MoveCommand", MoveCommandParams, EmptyMessage);
    super::make_plugin_request!(jump_command, "JumpCommand", MoveCommandParams, EmptyMessage);
    super::make_plugin_request!(
        movement_select_command,
        "MovementSelectCommand",
        IntMessage,
        EmptyMessage
    );
    super::make_plugin_request!(
        misc_move_command,
        "MiscMoveCommand",
        MiscMoveParams,
        EmptyMessage
    );
    super::make_plugin_getter_request!(get_side_menu, "GetSideMenu", SidebarState);
    super::make_plugin_getter_request!(get_material_list, "GetMaterialList", MaterialList);
    super::make_plugin_getter_request!(get_growth_list, "GetGrowthList", MaterialList);
    super::make_plugin_getter_request!(check_hashes, "CheckHashes", EmptyMessage);
    super::make_plugin_getter_request!(get_tiletype_list, "GetTiletypeList", TiletypeList);
    super::make_plugin_getter_request!(get_unit_list, "GetUnitList", UnitList);
    super::make_plugin_getter_request!(get_view_info, "GetViewInfo", ViewInfo);
    super::make_plugin_getter_request!(get_map_info, "GetMapInfo", MapInfo);
    super::make_plugin_getter_request!(reset_map_hashes, "ResetMapHashes", EmptyMessage);
    super::make_plugin_getter_request!(get_item_list, "GetItemList", MaterialList);
    super::make_plugin_getter_request!(get_building_def_list, "GetBuildingDefList", BuildingList);
    super::make_plugin_getter_request!(get_world_map, "GetWorldMap", WorldMap);
    super::make_plugin_getter_request!(get_world_map_new, "GetWorldMapNew", WorldMap);
    super::make_plugin_getter_request!(get_region_maps, "GetRegionMaps", RegionMaps);
    super::make_plugin_getter_request!(get_region_maps_new, "GetRegionMapsNew", RegionMaps);
    super::make_plugin_getter_request!(get_creature_raws, "GetCreatureRaws", CreatureRawList);
    super::make_plugin_getter_request!(get_world_map_center, "GetWorldMapCenter", WorldMap);
    super::make_plugin_getter_request!(get_plant_raws, "GetPlantRaws", PlantRawList);
    super::make_plugin_getter_request!(copy_screen, "CopyScreen", ScreenCapture);
    super::make_plugin_getter_request!(get_pause_state, "GetPauseState", SingleBool);
    super::make_plugin_getter_request!(get_version_info, "GetVersionInfo", VersionInfo);
    super::make_plugin_getter_request!(get_reports, "GetReports", Status);
    super::make_plugin_getter_request!(menu_query, "MenuQuery", MenuContents);
    super::make_plugin_getter_request!(get_language, "GetLanguage", Language);
    super::make_plugin_getter_request!(get_game_validity, "GetGameValidity", SingleBool);
}
