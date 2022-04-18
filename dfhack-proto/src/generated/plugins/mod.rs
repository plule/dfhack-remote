use std::{cell::RefCell, rc::Rc};
use crate::messages::*;
use std::marker::PhantomData;
///Generated list of DFHack plugins
pub struct Plugins<TChannel: crate::Channel<E>, E> {
    ///RPCs of the  plugin
    pub core: crate::plugins::Core<E, TChannel>,
    ///RPCs of the RemoteFortressReader plugin
    pub remote_fortress_reader: crate::plugins::RemoteFortressReader<E, TChannel>,
    ///RPCs of the isoworldremote plugin
    pub isoworldremote: crate::plugins::Isoworldremote<E, TChannel>,
    ///RPCs of the rename plugin
    pub rename: crate::plugins::Rename<E, TChannel>,
}
impl<TChannel: crate::Channel<E>, E> From<TChannel> for Plugins<TChannel, E> {
    ///Initialize all the generated plugins
    fn from(channel: TChannel) -> Self {
        let channel = std::rc::Rc::new(std::cell::RefCell::new(channel));
        Self {
            core: Core::new(std::rc::Rc::clone(&channel)),
            remote_fortress_reader: RemoteFortressReader::new(
                std::rc::Rc::clone(&channel),
            ),
            isoworldremote: Isoworldremote::new(std::rc::Rc::clone(&channel)),
            rename: Rename::new(std::rc::Rc::clone(&channel)),
        }
    }
}
///RPC for the "" plugin.
pub struct Core<E, TChannel: crate::Channel<E>> {
    ///Reference to the client to exchange messages.
    pub channel: Rc<RefCell<TChannel>>,
    ///Name of the plugin. All the RPC are attached to this name.
    pub name: String,
    phantom: PhantomData<E>,
}
impl<E, TChannel: crate::Channel<E>> Core<E, TChannel> {
    ///Instanciate a new plugin instance
    pub fn new(channel: Rc<RefCell<TChannel>>) -> Self {
        Self {
            channel,
            name: "".to_string(),
            phantom: PhantomData,
        }
    }
    ///Method `BindMethod` from the plugin ``
    pub fn bind_method(&mut self, request: CoreBindRequest) -> Result<CoreBindReply, E> {
        let _response: CoreBindReply = self
            .channel
            .borrow_mut()
            .request("".to_string(), "BindMethod".to_string(), request)?;
        Ok(_response)
    }
    ///Method `CoreResume` from the plugin ``
    pub fn core_resume(&mut self) -> Result<i32, E> {
        let request = EmptyMessage::new();
        let _response: IntMessage = self
            .channel
            .borrow_mut()
            .request("".to_string(), "CoreResume".to_string(), request)?;
        let _response = _response.get_value();
        Ok(_response)
    }
    ///Method `CoreSuspend` from the plugin ``
    pub fn core_suspend(&mut self) -> Result<i32, E> {
        let request = EmptyMessage::new();
        let _response: IntMessage = self
            .channel
            .borrow_mut()
            .request("".to_string(), "CoreSuspend".to_string(), request)?;
        let _response = _response.get_value();
        Ok(_response)
    }
    ///Method `GetDFVersion` from the plugin ``
    pub fn get_df_version(&mut self) -> Result<String, E> {
        let request = EmptyMessage::new();
        let _response: StringMessage = self
            .channel
            .borrow_mut()
            .request("".to_string(), "GetDFVersion".to_string(), request)?;
        let _response = _response.get_value().to_string();
        Ok(_response)
    }
    ///Method `GetVersion` from the plugin ``
    pub fn get_version(&mut self) -> Result<String, E> {
        let request = EmptyMessage::new();
        let _response: StringMessage = self
            .channel
            .borrow_mut()
            .request("".to_string(), "GetVersion".to_string(), request)?;
        let _response = _response.get_value().to_string();
        Ok(_response)
    }
    ///Method `GetWorldInfo` from the plugin ``
    pub fn get_world_info(&mut self) -> Result<GetWorldInfoOut, E> {
        let request = EmptyMessage::new();
        let _response: GetWorldInfoOut = self
            .channel
            .borrow_mut()
            .request("".to_string(), "GetWorldInfo".to_string(), request)?;
        Ok(_response)
    }
    ///Method `ListEnums` from the plugin ``
    pub fn list_enums(&mut self) -> Result<ListEnumsOut, E> {
        let request = EmptyMessage::new();
        let _response: ListEnumsOut = self
            .channel
            .borrow_mut()
            .request("".to_string(), "ListEnums".to_string(), request)?;
        Ok(_response)
    }
    ///Method `ListJobSkills` from the plugin ``
    pub fn list_job_skills(&mut self) -> Result<ListJobSkillsOut, E> {
        let request = EmptyMessage::new();
        let _response: ListJobSkillsOut = self
            .channel
            .borrow_mut()
            .request("".to_string(), "ListJobSkills".to_string(), request)?;
        Ok(_response)
    }
    ///Method `ListMaterials` from the plugin ``
    pub fn list_materials(
        &mut self,
        request: ListMaterialsIn,
    ) -> Result<ListMaterialsOut, E> {
        let _response: ListMaterialsOut = self
            .channel
            .borrow_mut()
            .request("".to_string(), "ListMaterials".to_string(), request)?;
        Ok(_response)
    }
    ///Method `ListSquads` from the plugin ``
    pub fn list_squads(&mut self, request: ListSquadsIn) -> Result<ListSquadsOut, E> {
        let _response: ListSquadsOut = self
            .channel
            .borrow_mut()
            .request("".to_string(), "ListSquads".to_string(), request)?;
        Ok(_response)
    }
    ///Method `ListUnits` from the plugin ``
    pub fn list_units(&mut self, request: ListUnitsIn) -> Result<ListUnitsOut, E> {
        let _response: ListUnitsOut = self
            .channel
            .borrow_mut()
            .request("".to_string(), "ListUnits".to_string(), request)?;
        Ok(_response)
    }
    ///Method `RunCommand` from the plugin ``
    pub fn run_command(&mut self, request: CoreRunCommandRequest) -> Result<(), E> {
        let _response: EmptyMessage = self
            .channel
            .borrow_mut()
            .request("".to_string(), "RunCommand".to_string(), request)?;
        let _response = ();
        Ok(_response)
    }
    ///Method `RunLua` from the plugin ``
    pub fn run_lua(
        &mut self,
        request: CoreRunLuaRequest,
    ) -> Result<StringListMessage, E> {
        let _response: StringListMessage = self
            .channel
            .borrow_mut()
            .request("".to_string(), "RunLua".to_string(), request)?;
        Ok(_response)
    }
    ///Method `SetUnitLabors` from the plugin ``
    pub fn set_unit_labors(&mut self, request: SetUnitLaborsIn) -> Result<(), E> {
        let _response: EmptyMessage = self
            .channel
            .borrow_mut()
            .request("".to_string(), "SetUnitLabors".to_string(), request)?;
        let _response = ();
        Ok(_response)
    }
}
///RPC for the "RemoteFortressReader" plugin.
pub struct RemoteFortressReader<E, TChannel: crate::Channel<E>> {
    ///Reference to the client to exchange messages.
    pub channel: Rc<RefCell<TChannel>>,
    ///Name of the plugin. All the RPC are attached to this name.
    pub name: String,
    phantom: PhantomData<E>,
}
impl<E, TChannel: crate::Channel<E>> RemoteFortressReader<E, TChannel> {
    ///Instanciate a new plugin instance
    pub fn new(channel: Rc<RefCell<TChannel>>) -> Self {
        Self {
            channel,
            name: "RemoteFortressReader".to_string(),
            phantom: PhantomData,
        }
    }
    ///Method `CheckHashes` from the plugin `RemoteFortressReader`
    pub fn check_hashes(&mut self) -> Result<(), E> {
        let request = EmptyMessage::new();
        let _response: EmptyMessage = self
            .channel
            .borrow_mut()
            .request(
                "RemoteFortressReader".to_string(),
                "CheckHashes".to_string(),
                request,
            )?;
        let _response = ();
        Ok(_response)
    }
    ///Method `CopyScreen` from the plugin `RemoteFortressReader`
    pub fn copy_screen(&mut self) -> Result<ScreenCapture, E> {
        let request = EmptyMessage::new();
        let _response: ScreenCapture = self
            .channel
            .borrow_mut()
            .request(
                "RemoteFortressReader".to_string(),
                "CopyScreen".to_string(),
                request,
            )?;
        Ok(_response)
    }
    ///Method `GetBlockList` from the plugin `RemoteFortressReader`
    pub fn get_block_list(&mut self, request: BlockRequest) -> Result<BlockList, E> {
        let _response: BlockList = self
            .channel
            .borrow_mut()
            .request(
                "RemoteFortressReader".to_string(),
                "GetBlockList".to_string(),
                request,
            )?;
        Ok(_response)
    }
    ///Method `GetBuildingDefList` from the plugin `RemoteFortressReader`
    pub fn get_building_def_list(&mut self) -> Result<BuildingList, E> {
        let request = EmptyMessage::new();
        let _response: BuildingList = self
            .channel
            .borrow_mut()
            .request(
                "RemoteFortressReader".to_string(),
                "GetBuildingDefList".to_string(),
                request,
            )?;
        Ok(_response)
    }
    ///Method `GetCreatureRaws` from the plugin `RemoteFortressReader`
    pub fn get_creature_raws(&mut self) -> Result<CreatureRawList, E> {
        let request = EmptyMessage::new();
        let _response: CreatureRawList = self
            .channel
            .borrow_mut()
            .request(
                "RemoteFortressReader".to_string(),
                "GetCreatureRaws".to_string(),
                request,
            )?;
        Ok(_response)
    }
    ///Method `GetGameValidity` from the plugin `RemoteFortressReader`
    pub fn get_game_validity(&mut self) -> Result<bool, E> {
        let request = EmptyMessage::new();
        let _response: SingleBool = self
            .channel
            .borrow_mut()
            .request(
                "RemoteFortressReader".to_string(),
                "GetGameValidity".to_string(),
                request,
            )?;
        let _response = _response.get_Value();
        Ok(_response)
    }
    ///Method `GetGrowthList` from the plugin `RemoteFortressReader`
    pub fn get_growth_list(&mut self) -> Result<MaterialList, E> {
        let request = EmptyMessage::new();
        let _response: MaterialList = self
            .channel
            .borrow_mut()
            .request(
                "RemoteFortressReader".to_string(),
                "GetGrowthList".to_string(),
                request,
            )?;
        Ok(_response)
    }
    ///Method `GetItemList` from the plugin `RemoteFortressReader`
    pub fn get_item_list(&mut self) -> Result<MaterialList, E> {
        let request = EmptyMessage::new();
        let _response: MaterialList = self
            .channel
            .borrow_mut()
            .request(
                "RemoteFortressReader".to_string(),
                "GetItemList".to_string(),
                request,
            )?;
        Ok(_response)
    }
    ///Method `GetLanguage` from the plugin `RemoteFortressReader`
    pub fn get_language(&mut self) -> Result<Language, E> {
        let request = EmptyMessage::new();
        let _response: Language = self
            .channel
            .borrow_mut()
            .request(
                "RemoteFortressReader".to_string(),
                "GetLanguage".to_string(),
                request,
            )?;
        Ok(_response)
    }
    ///Method `GetMapInfo` from the plugin `RemoteFortressReader`
    pub fn get_map_info(&mut self) -> Result<MapInfo, E> {
        let request = EmptyMessage::new();
        let _response: MapInfo = self
            .channel
            .borrow_mut()
            .request(
                "RemoteFortressReader".to_string(),
                "GetMapInfo".to_string(),
                request,
            )?;
        Ok(_response)
    }
    ///Method `GetMaterialList` from the plugin `RemoteFortressReader`
    pub fn get_material_list(&mut self) -> Result<MaterialList, E> {
        let request = EmptyMessage::new();
        let _response: MaterialList = self
            .channel
            .borrow_mut()
            .request(
                "RemoteFortressReader".to_string(),
                "GetMaterialList".to_string(),
                request,
            )?;
        Ok(_response)
    }
    ///Method `GetPartialCreatureRaws` from the plugin `RemoteFortressReader`
    pub fn get_partial_creature_raws(
        &mut self,
        request: ListRequest,
    ) -> Result<CreatureRawList, E> {
        let _response: CreatureRawList = self
            .channel
            .borrow_mut()
            .request(
                "RemoteFortressReader".to_string(),
                "GetPartialCreatureRaws".to_string(),
                request,
            )?;
        Ok(_response)
    }
    ///Method `GetPartialPlantRaws` from the plugin `RemoteFortressReader`
    pub fn get_partial_plant_raws(
        &mut self,
        request: ListRequest,
    ) -> Result<PlantRawList, E> {
        let _response: PlantRawList = self
            .channel
            .borrow_mut()
            .request(
                "RemoteFortressReader".to_string(),
                "GetPartialPlantRaws".to_string(),
                request,
            )?;
        Ok(_response)
    }
    ///Method `GetPauseState` from the plugin `RemoteFortressReader`
    pub fn get_pause_state(&mut self) -> Result<bool, E> {
        let request = EmptyMessage::new();
        let _response: SingleBool = self
            .channel
            .borrow_mut()
            .request(
                "RemoteFortressReader".to_string(),
                "GetPauseState".to_string(),
                request,
            )?;
        let _response = _response.get_Value();
        Ok(_response)
    }
    ///Method `GetPlantList` from the plugin `RemoteFortressReader`
    pub fn get_plant_list(&mut self, request: BlockRequest) -> Result<PlantList, E> {
        let _response: PlantList = self
            .channel
            .borrow_mut()
            .request(
                "RemoteFortressReader".to_string(),
                "GetPlantList".to_string(),
                request,
            )?;
        Ok(_response)
    }
    ///Method `GetPlantRaws` from the plugin `RemoteFortressReader`
    pub fn get_plant_raws(&mut self) -> Result<PlantRawList, E> {
        let request = EmptyMessage::new();
        let _response: PlantRawList = self
            .channel
            .borrow_mut()
            .request(
                "RemoteFortressReader".to_string(),
                "GetPlantRaws".to_string(),
                request,
            )?;
        Ok(_response)
    }
    ///Method `GetRegionMaps` from the plugin `RemoteFortressReader`
    pub fn get_region_maps(&mut self) -> Result<RegionMaps, E> {
        let request = EmptyMessage::new();
        let _response: RegionMaps = self
            .channel
            .borrow_mut()
            .request(
                "RemoteFortressReader".to_string(),
                "GetRegionMaps".to_string(),
                request,
            )?;
        Ok(_response)
    }
    ///Method `GetRegionMapsNew` from the plugin `RemoteFortressReader`
    pub fn get_region_maps_new(&mut self) -> Result<RegionMaps, E> {
        let request = EmptyMessage::new();
        let _response: RegionMaps = self
            .channel
            .borrow_mut()
            .request(
                "RemoteFortressReader".to_string(),
                "GetRegionMapsNew".to_string(),
                request,
            )?;
        Ok(_response)
    }
    ///Method `GetReports` from the plugin `RemoteFortressReader`
    pub fn get_reports(&mut self) -> Result<Status, E> {
        let request = EmptyMessage::new();
        let _response: Status = self
            .channel
            .borrow_mut()
            .request(
                "RemoteFortressReader".to_string(),
                "GetReports".to_string(),
                request,
            )?;
        Ok(_response)
    }
    ///Method `GetSideMenu` from the plugin `RemoteFortressReader`
    pub fn get_side_menu(&mut self) -> Result<SidebarState, E> {
        let request = EmptyMessage::new();
        let _response: SidebarState = self
            .channel
            .borrow_mut()
            .request(
                "RemoteFortressReader".to_string(),
                "GetSideMenu".to_string(),
                request,
            )?;
        Ok(_response)
    }
    ///Method `GetTiletypeList` from the plugin `RemoteFortressReader`
    pub fn get_tiletype_list(&mut self) -> Result<TiletypeList, E> {
        let request = EmptyMessage::new();
        let _response: TiletypeList = self
            .channel
            .borrow_mut()
            .request(
                "RemoteFortressReader".to_string(),
                "GetTiletypeList".to_string(),
                request,
            )?;
        Ok(_response)
    }
    ///Method `GetUnitList` from the plugin `RemoteFortressReader`
    pub fn get_unit_list(&mut self) -> Result<UnitList, E> {
        let request = EmptyMessage::new();
        let _response: UnitList = self
            .channel
            .borrow_mut()
            .request(
                "RemoteFortressReader".to_string(),
                "GetUnitList".to_string(),
                request,
            )?;
        Ok(_response)
    }
    ///Method `GetUnitListInside` from the plugin `RemoteFortressReader`
    pub fn get_unit_list_inside(
        &mut self,
        request: BlockRequest,
    ) -> Result<UnitList, E> {
        let _response: UnitList = self
            .channel
            .borrow_mut()
            .request(
                "RemoteFortressReader".to_string(),
                "GetUnitListInside".to_string(),
                request,
            )?;
        Ok(_response)
    }
    ///Method `GetVersionInfo` from the plugin `RemoteFortressReader`
    pub fn get_version_info(&mut self) -> Result<VersionInfo, E> {
        let request = EmptyMessage::new();
        let _response: VersionInfo = self
            .channel
            .borrow_mut()
            .request(
                "RemoteFortressReader".to_string(),
                "GetVersionInfo".to_string(),
                request,
            )?;
        Ok(_response)
    }
    ///Method `GetViewInfo` from the plugin `RemoteFortressReader`
    pub fn get_view_info(&mut self) -> Result<ViewInfo, E> {
        let request = EmptyMessage::new();
        let _response: ViewInfo = self
            .channel
            .borrow_mut()
            .request(
                "RemoteFortressReader".to_string(),
                "GetViewInfo".to_string(),
                request,
            )?;
        Ok(_response)
    }
    ///Method `GetWorldMap` from the plugin `RemoteFortressReader`
    pub fn get_world_map(&mut self) -> Result<WorldMap, E> {
        let request = EmptyMessage::new();
        let _response: WorldMap = self
            .channel
            .borrow_mut()
            .request(
                "RemoteFortressReader".to_string(),
                "GetWorldMap".to_string(),
                request,
            )?;
        Ok(_response)
    }
    ///Method `GetWorldMapCenter` from the plugin `RemoteFortressReader`
    pub fn get_world_map_center(&mut self) -> Result<WorldMap, E> {
        let request = EmptyMessage::new();
        let _response: WorldMap = self
            .channel
            .borrow_mut()
            .request(
                "RemoteFortressReader".to_string(),
                "GetWorldMapCenter".to_string(),
                request,
            )?;
        Ok(_response)
    }
    ///Method `GetWorldMapNew` from the plugin `RemoteFortressReader`
    pub fn get_world_map_new(&mut self) -> Result<WorldMap, E> {
        let request = EmptyMessage::new();
        let _response: WorldMap = self
            .channel
            .borrow_mut()
            .request(
                "RemoteFortressReader".to_string(),
                "GetWorldMapNew".to_string(),
                request,
            )?;
        Ok(_response)
    }
    ///Method `JumpCommand` from the plugin `RemoteFortressReader`
    pub fn jump_command(&mut self, request: MoveCommandParams) -> Result<(), E> {
        let _response: EmptyMessage = self
            .channel
            .borrow_mut()
            .request(
                "RemoteFortressReader".to_string(),
                "JumpCommand".to_string(),
                request,
            )?;
        let _response = ();
        Ok(_response)
    }
    ///Method `MenuQuery` from the plugin `RemoteFortressReader`
    pub fn menu_query(&mut self) -> Result<MenuContents, E> {
        let request = EmptyMessage::new();
        let _response: MenuContents = self
            .channel
            .borrow_mut()
            .request(
                "RemoteFortressReader".to_string(),
                "MenuQuery".to_string(),
                request,
            )?;
        Ok(_response)
    }
    ///Method `MiscMoveCommand` from the plugin `RemoteFortressReader`
    pub fn misc_move_command(&mut self, request: MiscMoveParams) -> Result<(), E> {
        let _response: EmptyMessage = self
            .channel
            .borrow_mut()
            .request(
                "RemoteFortressReader".to_string(),
                "MiscMoveCommand".to_string(),
                request,
            )?;
        let _response = ();
        Ok(_response)
    }
    ///Method `MoveCommand` from the plugin `RemoteFortressReader`
    pub fn move_command(&mut self, request: MoveCommandParams) -> Result<(), E> {
        let _response: EmptyMessage = self
            .channel
            .borrow_mut()
            .request(
                "RemoteFortressReader".to_string(),
                "MoveCommand".to_string(),
                request,
            )?;
        let _response = ();
        Ok(_response)
    }
    ///Method `MovementSelectCommand` from the plugin `RemoteFortressReader`
    pub fn movement_select_command(&mut self, value: i32) -> Result<(), E> {
        let mut request = IntMessage::new();
        request.set_value(value);
        let _response: EmptyMessage = self
            .channel
            .borrow_mut()
            .request(
                "RemoteFortressReader".to_string(),
                "MovementSelectCommand".to_string(),
                request,
            )?;
        let _response = ();
        Ok(_response)
    }
    ///Method `PassKeyboardEvent` from the plugin `RemoteFortressReader`
    pub fn pass_keyboard_event(&mut self, request: KeyboardEvent) -> Result<(), E> {
        let _response: EmptyMessage = self
            .channel
            .borrow_mut()
            .request(
                "RemoteFortressReader".to_string(),
                "PassKeyboardEvent".to_string(),
                request,
            )?;
        let _response = ();
        Ok(_response)
    }
    ///Method `ResetMapHashes` from the plugin `RemoteFortressReader`
    pub fn reset_map_hashes(&mut self) -> Result<(), E> {
        let request = EmptyMessage::new();
        let _response: EmptyMessage = self
            .channel
            .borrow_mut()
            .request(
                "RemoteFortressReader".to_string(),
                "ResetMapHashes".to_string(),
                request,
            )?;
        let _response = ();
        Ok(_response)
    }
    ///Method `SendDigCommand` from the plugin `RemoteFortressReader`
    pub fn send_dig_command(&mut self, request: DigCommand) -> Result<(), E> {
        let _response: EmptyMessage = self
            .channel
            .borrow_mut()
            .request(
                "RemoteFortressReader".to_string(),
                "SendDigCommand".to_string(),
                request,
            )?;
        let _response = ();
        Ok(_response)
    }
    ///Method `SetPauseState` from the plugin `RemoteFortressReader`
    pub fn set_pause_state(&mut self, value: bool) -> Result<(), E> {
        let mut request = SingleBool::new();
        request.set_Value(value);
        let _response: EmptyMessage = self
            .channel
            .borrow_mut()
            .request(
                "RemoteFortressReader".to_string(),
                "SetPauseState".to_string(),
                request,
            )?;
        let _response = ();
        Ok(_response)
    }
    ///Method `SetSideMenu` from the plugin `RemoteFortressReader`
    pub fn set_side_menu(&mut self, request: SidebarCommand) -> Result<(), E> {
        let _response: EmptyMessage = self
            .channel
            .borrow_mut()
            .request(
                "RemoteFortressReader".to_string(),
                "SetSideMenu".to_string(),
                request,
            )?;
        let _response = ();
        Ok(_response)
    }
}
///RPC for the "isoworldremote" plugin.
pub struct Isoworldremote<E, TChannel: crate::Channel<E>> {
    ///Reference to the client to exchange messages.
    pub channel: Rc<RefCell<TChannel>>,
    ///Name of the plugin. All the RPC are attached to this name.
    pub name: String,
    phantom: PhantomData<E>,
}
impl<E, TChannel: crate::Channel<E>> Isoworldremote<E, TChannel> {
    ///Instanciate a new plugin instance
    pub fn new(channel: Rc<RefCell<TChannel>>) -> Self {
        Self {
            channel,
            name: "isoworldremote".to_string(),
            phantom: PhantomData,
        }
    }
    ///Method `GetEmbarkInfo` from the plugin `isoworldremote`
    pub fn get_embark_info(&mut self, request: MapRequest) -> Result<MapReply, E> {
        let _response: MapReply = self
            .channel
            .borrow_mut()
            .request(
                "isoworldremote".to_string(),
                "GetEmbarkInfo".to_string(),
                request,
            )?;
        Ok(_response)
    }
    ///Method `GetEmbarkTile` from the plugin `isoworldremote`
    pub fn get_embark_tile(&mut self, request: TileRequest) -> Result<EmbarkTile, E> {
        let _response: EmbarkTile = self
            .channel
            .borrow_mut()
            .request(
                "isoworldremote".to_string(),
                "GetEmbarkTile".to_string(),
                request,
            )?;
        Ok(_response)
    }
    ///Method `GetRawNames` from the plugin `isoworldremote`
    pub fn get_raw_names(&mut self, request: MapRequest) -> Result<RawNames, E> {
        let _response: RawNames = self
            .channel
            .borrow_mut()
            .request("isoworldremote".to_string(), "GetRawNames".to_string(), request)?;
        Ok(_response)
    }
}
///RPC for the "rename" plugin.
pub struct Rename<E, TChannel: crate::Channel<E>> {
    ///Reference to the client to exchange messages.
    pub channel: Rc<RefCell<TChannel>>,
    ///Name of the plugin. All the RPC are attached to this name.
    pub name: String,
    phantom: PhantomData<E>,
}
impl<E, TChannel: crate::Channel<E>> Rename<E, TChannel> {
    ///Instanciate a new plugin instance
    pub fn new(channel: Rc<RefCell<TChannel>>) -> Self {
        Self {
            channel,
            name: "rename".to_string(),
            phantom: PhantomData,
        }
    }
    ///Method `RenameBuilding` from the plugin `rename`
    pub fn rename_building(&mut self, request: RenameBuildingIn) -> Result<(), E> {
        let _response: EmptyMessage = self
            .channel
            .borrow_mut()
            .request("rename".to_string(), "RenameBuilding".to_string(), request)?;
        let _response = ();
        Ok(_response)
    }
    ///Method `RenameSquad` from the plugin `rename`
    pub fn rename_squad(&mut self, request: RenameSquadIn) -> Result<(), E> {
        let _response: EmptyMessage = self
            .channel
            .borrow_mut()
            .request("rename".to_string(), "RenameSquad".to_string(), request)?;
        let _response = ();
        Ok(_response)
    }
    ///Method `RenameUnit` from the plugin `rename`
    pub fn rename_unit(&mut self, request: RenameUnitIn) -> Result<(), E> {
        let _response: EmptyMessage = self
            .channel
            .borrow_mut()
            .request("rename".to_string(), "RenameUnit".to_string(), request)?;
        let _response = ();
        Ok(_response)
    }
}
