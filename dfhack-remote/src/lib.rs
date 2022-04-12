use std::{collections::HashMap, fmt};

use dfhack_proto::isoworldremote::*;
use dfhack_proto::rename::*;
use dfhack_proto::AdventureControl::*;
use dfhack_proto::BasicApi::*;
use dfhack_proto::CoreProtocol::*;
use dfhack_proto::DwarfControl::*;
use dfhack_proto::RemoteFortressReader::*;
use message::{Receive, Send};
use num_enum::TryFromPrimitiveError;

pub mod message;

#[derive(PartialEq, Eq, Hash)]
pub struct Method {
    pub plugin: String,
    pub name: String,
}

impl Method {
    fn new(plugin: String, name: String) -> Self {
        Method { plugin, name }
    }
}

pub struct DfClient {
    stream: std::net::TcpStream,
    bindings: HashMap<Method, i16>,
}

const MAGIC_QUERY: &str = "DFHack?\n";
const MAGIC_REPLY: &str = "DFHack!\n";
const VERSION: i32 = 1;

const BIND_METHOD_ID: i16 = 0;
const RUN_COMMAND_ID: i16 = 1;

macro_rules! make_request {
    ($func_name:ident, $plugin_name:literal, $method_name:literal, $request_type:path, $response_type:path) => {
        pub fn $func_name(&mut self, request: $request_type) -> Result<$response_type> {
            self.request($plugin_name.to_string(), $method_name.to_string(), request)
        }
    };
    ($func_name:ident, $method_name:literal, $request_type:path, $response_type:path) => {
        pub fn $func_name(&mut self, request: $request_type) -> Result<$response_type> {
            self.request("".to_string(), $method_name.to_string(), request)
        }
    };
}

macro_rules! make_getter_request {
    ($func_name:ident, $plugin_name:literal, $method_name:literal, $response_type:path) => {
        pub fn $func_name(&mut self) -> Result<$response_type> {
            let request = dfhack_proto::CoreProtocol::EmptyMessage::new();
            self.request($plugin_name.to_string(), $method_name.to_string(), request)
        }
    };
    ($func_name:ident, $method_name:literal, $response_type:path) => {
        pub fn $func_name(&mut self) -> Result<$response_type> {
            let request = dfhack_proto::CoreProtocol::EmptyMessage::new();
            self.request("".to_string(), $method_name.to_string(), request)
        }
    };
}

impl DfClient {
    pub fn connect() -> Result<DfClient> {
        let mut client = DfClient {
            stream: std::net::TcpStream::connect("127.0.0.1:5000")?,
            bindings: HashMap::new(),
        };

        client.bindings.insert(
            Method::new("".to_string(), "BindMethod".to_string()),
            BIND_METHOD_ID,
        );
        client.bindings.insert(
            Method::new("".to_string(), "RunCommand".to_string()),
            RUN_COMMAND_ID,
        );

        let handshake_request = message::Handshake::new(MAGIC_QUERY.to_string(), VERSION);
        handshake_request.send(&mut client.stream)?;
        let handshake_reply = message::Handshake::receive(&mut client.stream)?;

        if handshake_reply.magic != MAGIC_REPLY {
            return Err(DfRemoteError::BadMagicFailure(handshake_reply.magic));
        }

        if handshake_reply.version != VERSION {
            return Err(DfRemoteError::BadVersionFailure(handshake_reply.version));
        }

        Ok(client)
    }

    // core
    make_getter_request!(get_version, "GetVersion", StringMessage);
    make_getter_request!(get_df_version, "GetDFVersion", StringMessage);
    make_getter_request!(get_world_info, "GetWorldInfo", GetWorldInfoOut);
    make_getter_request!(list_enums, "ListEnums", ListEnumsOut);
    make_getter_request!(list_job_skills, "ListJobSkills", ListJobSkillsOut);
    make_getter_request!(core_suspends, "CoreSuspend", IntMessage);
    make_getter_request!(core_resume, "CoreResume", IntMessage);
    make_request!(
        list_materials,
        "ListMaterials",
        ListMaterialsIn,
        ListMaterialsOut
    );
    make_request!(list_units, "ListUnits", ListUnitsIn, ListUnitsOut);
    make_request!(list_squads, "ListSquads", ListSquadsIn, ListSquadsOut);
    make_request!(
        set_unit_labors,
        "SetUnitLabors",
        SetUnitLaborsIn,
        EmptyMessage
    );
    //make_request!(bind_method, "BindMethod", CoreBindRequest, CoreBindReply);
    make_request!(
        run_command,
        "RunCommand",
        CoreRunCommandRequest,
        EmptyMessage
    );
    make_request!(run_lua, "RunLua", CoreRunLuaRequest, StringListMessage);

    // isoworld
    make_request!(
        get_embark_tile,
        "isoworldremote",
        "GetEmbarkTile",
        TileRequest,
        EmbarkTile
    );
    make_request!(
        get_embark_info,
        "isoworldremote",
        "GetEmbarkInfo",
        MapRequest,
        MapReply
    );
    make_request!(
        get_raw_names,
        "isoworldremote",
        "GetRawNames",
        MapRequest,
        RawNames
    );

    // rename
    make_request!(
        rename_squad,
        "rename",
        "RenameSquad",
        RenameSquadIn,
        EmptyMessage
    );
    make_request!(
        rename_unit,
        "rename",
        "RenameUnit",
        RenameUnitIn,
        EmptyMessage
    );
    make_request!(
        rename_building,
        "rename",
        "RenameBuilding",
        RenameBuildingIn,
        EmptyMessage
    );

    // RemoteFortressReader
    make_request!(
        set_side_menu,
        "RemoteFortressReader",
        "SetSideMenu",
        SidebarCommand,
        EmptyMessage
    );
    make_request!(
        get_block_list,
        "RemoteFortressReader",
        "GetBlockList",
        BlockRequest,
        BlockList
    );
    make_request!(
        get_plant_list,
        "RemoteFortressReader",
        "GetPlantList",
        BlockRequest,
        PlantList
    );
    make_request!(
        get_unit_list_inside,
        "RemoteFortressReader",
        "GetUnitListInside",
        BlockRequest,
        UnitList
    );
    make_request!(
        get_partial_creature_raws,
        "RemoteFortressReader",
        "GetPartialCreatureRaws",
        ListRequest,
        CreatureRawList
    );
    make_request!(
        get_partial_plant_raws,
        "RemoteFortressReader",
        "GetPartialPlantRaws",
        ListRequest,
        PlantRawList
    );
    make_request!(
        pass_keyboard_event,
        "RemoteFortressReader",
        "PassKeyboardEvent",
        KeyboardEvent,
        EmptyMessage
    );
    make_request!(
        send_dig_command,
        "RemoteFortressReader",
        "SendDigCommand",
        DigCommand,
        EmptyMessage
    );
    make_request!(
        set_pause_state,
        "RemoteFortressReader",
        "SetPauseState",
        SingleBool,
        EmptyMessage
    );
    make_request!(
        move_command,
        "RemoteFortressReader",
        "MoveCommand",
        MoveCommandParams,
        EmptyMessage
    );
    make_request!(
        jump_command,
        "RemoteFortressReader",
        "JumpCommand",
        MoveCommandParams,
        EmptyMessage
    );
    make_request!(
        movement_select_command,
        "RemoteFortressReader",
        "MovementSelectCommand",
        IntMessage,
        EmptyMessage
    );
    make_request!(
        misc_move_command,
        "RemoteFortressReader",
        "MiscMoveCommand",
        MiscMoveParams,
        EmptyMessage
    );
    make_getter_request!(
        get_side_menu,
        "RemoteFortressReader",
        "GetSideMenu",
        SidebarState
    );
    make_getter_request!(
        get_material_list,
        "RemoteFortressReader",
        "GetMaterialList",
        MaterialList
    );
    make_getter_request!(
        get_growth_list,
        "RemoteFortressReader",
        "GetGrowthList",
        MaterialList
    );
    make_getter_request!(
        check_hashes,
        "RemoteFortressReader",
        "CheckHashes",
        EmptyMessage
    );
    make_getter_request!(
        get_tiletype_list,
        "RemoteFortressReader",
        "GetTiletypeList",
        TiletypeList
    );
    make_getter_request!(
        get_unit_list,
        "RemoteFortressReader",
        "GetUnitList",
        UnitList
    );
    make_getter_request!(
        get_view_info,
        "RemoteFortressReader",
        "GetViewInfo",
        ViewInfo
    );
    make_getter_request!(get_map_info, "RemoteFortressReader", "GetMapInfo", MapInfo);
    make_getter_request!(
        reset_map_hashes,
        "RemoteFortressReader",
        "ResetMapHashes",
        EmptyMessage
    );
    make_getter_request!(
        get_item_list,
        "RemoteFortressReader",
        "GetItemList",
        MaterialList
    );
    make_getter_request!(
        get_building_def_list,
        "RemoteFortressReader",
        "GetBuildingDefList",
        BuildingList
    );
    make_getter_request!(
        get_world_map,
        "RemoteFortressReader",
        "GetWorldMap",
        WorldMap
    );
    make_getter_request!(
        get_world_map_new,
        "RemoteFortressReader",
        "GetWorldMapNew",
        WorldMap
    );
    make_getter_request!(
        get_region_maps,
        "RemoteFortressReader",
        "GetRegionMaps",
        RegionMaps
    );
    make_getter_request!(
        get_region_maps_new,
        "RemoteFortressReader",
        "GetRegionMapsNew",
        RegionMaps
    );
    make_getter_request!(
        get_creature_raws,
        "RemoteFortressReader",
        "GetCreatureRaws",
        CreatureRawList
    );
    make_getter_request!(
        get_world_map_center,
        "RemoteFortressReader",
        "GetWorldMapCenter",
        WorldMap
    );
    make_getter_request!(
        get_plant_raws,
        "RemoteFortressReader",
        "GetPlantRaws",
        PlantRawList
    );
    make_getter_request!(
        copy_screen,
        "RemoteFortressReader",
        "CopyScreen",
        ScreenCapture
    );
    make_getter_request!(
        get_pause_state,
        "RemoteFortressReader",
        "GetPauseState",
        SingleBool
    );
    make_getter_request!(
        get_version_info,
        "RemoteFortressReader",
        "GetVersionInfo",
        VersionInfo
    );
    make_getter_request!(get_reports, "RemoteFortressReader", "GetReports", Status);
    make_getter_request!(
        menu_query,
        "RemoteFortressReader",
        "MenuQuery",
        MenuContents
    );
    make_getter_request!(
        get_language,
        "RemoteFortressReader",
        "GetLanguage",
        Language
    );
    make_getter_request!(
        get_game_validity,
        "RemoteFortressReader",
        "GetGameValidity",
        SingleBool
    );

    pub fn request<TRequest: protobuf::Message, TReply: protobuf::Message>(
        &mut self,
        plugin: String,
        name: String,
        request: TRequest,
    ) -> Result<TReply> {
        let method = Method::new(plugin, name);

        // did not manage to use the entry api due to borrow checker
        let maybe_id = self.bindings.get(&method);
        let id: i16;

        if maybe_id.is_none() {
            id = self.bind_method::<TRequest, TReply>(&method)?;
            self.bindings.insert(method, id);
        } else {
            id = *maybe_id.unwrap();
        }
        self.request_raw(id, request)
    }

    pub fn request_raw<TIN: protobuf::Message, TOUT: protobuf::Message>(
        &mut self,
        id: i16,
        message: TIN,
    ) -> Result<TOUT> {
        let request = message::Request::new(id, message);
        request.send(&mut self.stream)?;

        loop {
            let reply: message::Reply<TOUT> = message::Reply::receive(&mut self.stream)?;
            match reply {
                message::Reply::Text(text) => {
                    for fragment in text.get_fragments() {
                        println!("{}", fragment.get_text());
                    }
                }
                message::Reply::Result(result) => return Ok(result),
                message::Reply::Failure(_) => return Err(DfRemoteError::RpcError()),
            }
        }
    }

    pub fn bind_method<TIN: protobuf::Message, TOUT: protobuf::Message>(
        &mut self,
        method: &Method,
    ) -> Result<i16> {
        let mut request = dfhack_proto::CoreProtocol::CoreBindRequest::new();
        let input_msg = TIN::descriptor_static().full_name();
        let output_msg = TOUT::descriptor_static().full_name();
        request.set_method(method.name.to_owned());
        request.set_input_msg(input_msg.to_string());
        request.set_output_msg(output_msg.to_string());
        request.set_plugin(method.plugin.to_owned());
        let reply: dfhack_proto::CoreProtocol::CoreBindReply =
            self.request_raw(BIND_METHOD_ID, request)?;
        Ok(reply.get_assigned_id() as i16)
    }
}

impl Drop for DfClient {
    fn drop(&mut self) {
        let quit = message::Quit::new();
        let res = quit.send(&mut self.stream);
        if let Err(failure) = res {
            println!(
                "Warning: failed to close the connection to dfhack-remote: {}",
                failure
            );
        }
    }
}

pub type Result<T> = std::result::Result<T, DfRemoteError>;

#[derive(Debug)]
pub enum DfRemoteError {
    CommunicationFailure(std::io::Error),
    BadMagicFailure(String),
    BadVersionFailure(i32),
    ProtobufError(protobuf::ProtobufError),
    UnknownReplyCode(i16),
    RpcError(),
}

impl fmt::Display for DfRemoteError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        match self {
            DfRemoteError::BadMagicFailure(magic) => {
                write!(f, "Handshake failed: bad magic {magic}.")
            }
            DfRemoteError::BadVersionFailure(version) => {
                write!(f, "Handshake failed: unsupported version {version}.")
            }
            DfRemoteError::CommunicationFailure(error) => {
                write!(f, "Communication failure: {error}")
            }
            DfRemoteError::ProtobufError(error) => {
                write!(f, "Protobuf error: {error}")
            }
            DfRemoteError::UnknownReplyCode(code) => {
                write!(f, "Unknown DFHack reply code: {code}")
            }
            DfRemoteError::RpcError() => {
                write!(f, "RPC Error")
            }
        }
    }
}

impl From<std::io::Error> for DfRemoteError {
    fn from(err: std::io::Error) -> Self {
        Self::CommunicationFailure(err)
    }
}

impl From<protobuf::ProtobufError> for DfRemoteError {
    fn from(err: protobuf::ProtobufError) -> Self {
        Self::ProtobufError(err)
    }
}

impl From<TryFromPrimitiveError<message::RpcReplyCode>> for DfRemoteError {
    fn from(err: TryFromPrimitiveError<message::RpcReplyCode>) -> Self {
        Self::UnknownReplyCode(err.number)
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
