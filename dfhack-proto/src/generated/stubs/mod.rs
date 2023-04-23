use crate::messages::*;
#[cfg(feature = "reflection")]
use protobuf::MessageFull;
///Generated list of DFHack stubs. Each stub communicates with a plugin.
pub struct Stubs<TChannel: crate::Channel> {
    channel: TChannel,
}
impl<TChannel: crate::Channel> From<TChannel> for Stubs<TChannel> {
    ///Initialize all the generated stubs.
    fn from(channel: TChannel) -> Self {
        Self { channel }
    }
}
impl<TChannel: crate::Channel> Stubs<TChannel> {
    ///RPCs of the  plugin
    pub fn core(&mut self) -> crate::stubs::Core<TChannel> {
        crate::stubs::Core::new(&mut self.channel)
    }
    ///RPCs of the RemoteFortressReader plugin
    pub fn remote_fortress_reader(
        &mut self,
    ) -> crate::stubs::RemoteFortressReader<TChannel> {
        crate::stubs::RemoteFortressReader::new(&mut self.channel)
    }
    ///RPCs of the isoworldremote plugin
    pub fn isoworldremote(&mut self) -> crate::stubs::Isoworldremote<TChannel> {
        crate::stubs::Isoworldremote::new(&mut self.channel)
    }
    ///RPCs of the rename plugin
    pub fn rename(&mut self) -> crate::stubs::Rename<TChannel> {
        crate::stubs::Rename::new(&mut self.channel)
    }
}
#[cfg(feature = "reflection")]
impl<TChannel: crate::Channel> crate::reflection::StubReflection for Stubs<TChannel> {
    fn list_methods() -> Vec<crate::reflection::RemoteProcedureDescriptor> {
        let mut methods = Vec::new();
        methods.extend(Core::<TChannel>::list_methods());
        methods.extend(Core::<TChannel>::list_methods());
        methods.extend(Core::<TChannel>::list_methods());
        methods.extend(Core::<TChannel>::list_methods());
        methods
    }
}
///RPC for the "" plugin.
pub struct Core<'a, TChannel: crate::Channel> {
    ///Reference to the client to exchange messages.
    pub channel: &'a mut TChannel,
}
impl<'a, TChannel: crate::Channel> Core<'a, TChannel> {
    ///Initialize the plugin from a channel to DFHack.
    pub fn new(channel: &'a mut TChannel) -> Self {
        Self { channel }
    }
    ///Method `BindMethod` from the plugin ``
    pub fn bind_method(
        &mut self,
        request: CoreBindRequest,
    ) -> Result<CoreBindReply, TChannel::TError> {
        let _response: CoreBindReply = self
            .channel
            .request("".to_string(), "BindMethod".to_string(), request)?;
        Ok(_response)
    }
    ///Method `CoreResume` from the plugin ``
    pub fn core_resume(&mut self) -> Result<i32, TChannel::TError> {
        let request = EmptyMessage::new();
        let _response: IntMessage = self
            .channel
            .request("".to_string(), "CoreResume".to_string(), request)?;
        let _response = _response.value();
        Ok(_response)
    }
    ///Method `CoreSuspend` from the plugin ``
    pub fn core_suspend(&mut self) -> Result<i32, TChannel::TError> {
        let request = EmptyMessage::new();
        let _response: IntMessage = self
            .channel
            .request("".to_string(), "CoreSuspend".to_string(), request)?;
        let _response = _response.value();
        Ok(_response)
    }
    ///Method `GetDFVersion` from the plugin ``
    pub fn get_df_version(&mut self) -> Result<String, TChannel::TError> {
        let request = EmptyMessage::new();
        let _response: StringMessage = self
            .channel
            .request("".to_string(), "GetDFVersion".to_string(), request)?;
        let _response = _response.value().to_string();
        Ok(_response)
    }
    ///Method `GetVersion` from the plugin ``
    pub fn get_version(&mut self) -> Result<String, TChannel::TError> {
        let request = EmptyMessage::new();
        let _response: StringMessage = self
            .channel
            .request("".to_string(), "GetVersion".to_string(), request)?;
        let _response = _response.value().to_string();
        Ok(_response)
    }
    ///Method `GetWorldInfo` from the plugin ``
    pub fn get_world_info(&mut self) -> Result<GetWorldInfoOut, TChannel::TError> {
        let request = EmptyMessage::new();
        let _response: GetWorldInfoOut = self
            .channel
            .request("".to_string(), "GetWorldInfo".to_string(), request)?;
        Ok(_response)
    }
    ///Method `ListEnums` from the plugin ``
    pub fn list_enums(&mut self) -> Result<ListEnumsOut, TChannel::TError> {
        let request = EmptyMessage::new();
        let _response: ListEnumsOut = self
            .channel
            .request("".to_string(), "ListEnums".to_string(), request)?;
        Ok(_response)
    }
    ///Method `ListJobSkills` from the plugin ``
    pub fn list_job_skills(&mut self) -> Result<ListJobSkillsOut, TChannel::TError> {
        let request = EmptyMessage::new();
        let _response: ListJobSkillsOut = self
            .channel
            .request("".to_string(), "ListJobSkills".to_string(), request)?;
        Ok(_response)
    }
    ///Method `ListMaterials` from the plugin ``
    pub fn list_materials(
        &mut self,
        request: ListMaterialsIn,
    ) -> Result<ListMaterialsOut, TChannel::TError> {
        let _response: ListMaterialsOut = self
            .channel
            .request("".to_string(), "ListMaterials".to_string(), request)?;
        Ok(_response)
    }
    ///Method `ListSquads` from the plugin ``
    pub fn list_squads(
        &mut self,
        request: ListSquadsIn,
    ) -> Result<ListSquadsOut, TChannel::TError> {
        let _response: ListSquadsOut = self
            .channel
            .request("".to_string(), "ListSquads".to_string(), request)?;
        Ok(_response)
    }
    ///Method `ListUnits` from the plugin ``
    pub fn list_units(
        &mut self,
        request: ListUnitsIn,
    ) -> Result<ListUnitsOut, TChannel::TError> {
        let _response: ListUnitsOut = self
            .channel
            .request("".to_string(), "ListUnits".to_string(), request)?;
        Ok(_response)
    }
    ///Method `RunCommand` from the plugin ``
    pub fn run_command(
        &mut self,
        request: CoreRunCommandRequest,
    ) -> Result<(), TChannel::TError> {
        let _response: EmptyMessage = self
            .channel
            .request("".to_string(), "RunCommand".to_string(), request)?;
        let _response = ();
        Ok(_response)
    }
    ///Method `RunLua` from the plugin ``
    pub fn run_lua(
        &mut self,
        request: CoreRunLuaRequest,
    ) -> Result<StringListMessage, TChannel::TError> {
        let _response: StringListMessage = self
            .channel
            .request("".to_string(), "RunLua".to_string(), request)?;
        Ok(_response)
    }
    ///Method `SetUnitLabors` from the plugin ``
    pub fn set_unit_labors(
        &mut self,
        request: SetUnitLaborsIn,
    ) -> Result<(), TChannel::TError> {
        let _response: EmptyMessage = self
            .channel
            .request("".to_string(), "SetUnitLabors".to_string(), request)?;
        let _response = ();
        Ok(_response)
    }
}
#[cfg(feature = "reflection")]
impl<TChannel: crate::Channel> crate::reflection::StubReflection for Core<'_, TChannel> {
    fn list_methods() -> Vec<crate::reflection::RemoteProcedureDescriptor> {
        vec![
            crate ::reflection::RemoteProcedureDescriptor { name : "BindMethod"
            .to_string(), plugin_name : "".to_string(), input_type :
            CoreBindRequest::descriptor().full_name().to_string(), output_type :
            CoreBindReply::descriptor().full_name().to_string(), }, crate
            ::reflection::RemoteProcedureDescriptor { name : "CoreResume".to_string(),
            plugin_name : "".to_string(), input_type : EmptyMessage::descriptor()
            .full_name().to_string(), output_type : IntMessage::descriptor().full_name()
            .to_string(), }, crate ::reflection::RemoteProcedureDescriptor { name :
            "CoreSuspend".to_string(), plugin_name : "".to_string(), input_type :
            EmptyMessage::descriptor().full_name().to_string(), output_type :
            IntMessage::descriptor().full_name().to_string(), }, crate
            ::reflection::RemoteProcedureDescriptor { name : "GetDFVersion".to_string(),
            plugin_name : "".to_string(), input_type : EmptyMessage::descriptor()
            .full_name().to_string(), output_type : StringMessage::descriptor()
            .full_name().to_string(), }, crate ::reflection::RemoteProcedureDescriptor {
            name : "GetVersion".to_string(), plugin_name : "".to_string(), input_type :
            EmptyMessage::descriptor().full_name().to_string(), output_type :
            StringMessage::descriptor().full_name().to_string(), }, crate
            ::reflection::RemoteProcedureDescriptor { name : "GetWorldInfo".to_string(),
            plugin_name : "".to_string(), input_type : EmptyMessage::descriptor()
            .full_name().to_string(), output_type : GetWorldInfoOut::descriptor()
            .full_name().to_string(), }, crate ::reflection::RemoteProcedureDescriptor {
            name : "ListEnums".to_string(), plugin_name : "".to_string(), input_type :
            EmptyMessage::descriptor().full_name().to_string(), output_type :
            ListEnumsOut::descriptor().full_name().to_string(), }, crate
            ::reflection::RemoteProcedureDescriptor { name : "ListJobSkills".to_string(),
            plugin_name : "".to_string(), input_type : EmptyMessage::descriptor()
            .full_name().to_string(), output_type : ListJobSkillsOut::descriptor()
            .full_name().to_string(), }, crate ::reflection::RemoteProcedureDescriptor {
            name : "ListMaterials".to_string(), plugin_name : "".to_string(), input_type
            : ListMaterialsIn::descriptor().full_name().to_string(), output_type :
            ListMaterialsOut::descriptor().full_name().to_string(), }, crate
            ::reflection::RemoteProcedureDescriptor { name : "ListSquads".to_string(),
            plugin_name : "".to_string(), input_type : ListSquadsIn::descriptor()
            .full_name().to_string(), output_type : ListSquadsOut::descriptor()
            .full_name().to_string(), }, crate ::reflection::RemoteProcedureDescriptor {
            name : "ListUnits".to_string(), plugin_name : "".to_string(), input_type :
            ListUnitsIn::descriptor().full_name().to_string(), output_type :
            ListUnitsOut::descriptor().full_name().to_string(), }, crate
            ::reflection::RemoteProcedureDescriptor { name : "RunCommand".to_string(),
            plugin_name : "".to_string(), input_type :
            CoreRunCommandRequest::descriptor().full_name().to_string(), output_type :
            EmptyMessage::descriptor().full_name().to_string(), }, crate
            ::reflection::RemoteProcedureDescriptor { name : "RunLua".to_string(),
            plugin_name : "".to_string(), input_type : CoreRunLuaRequest::descriptor()
            .full_name().to_string(), output_type : StringListMessage::descriptor()
            .full_name().to_string(), }, crate ::reflection::RemoteProcedureDescriptor {
            name : "SetUnitLabors".to_string(), plugin_name : "".to_string(), input_type
            : SetUnitLaborsIn::descriptor().full_name().to_string(), output_type :
            EmptyMessage::descriptor().full_name().to_string(), },
        ]
    }
}
///RPC for the "RemoteFortressReader" plugin.
pub struct RemoteFortressReader<'a, TChannel: crate::Channel> {
    ///Reference to the client to exchange messages.
    pub channel: &'a mut TChannel,
}
impl<'a, TChannel: crate::Channel> RemoteFortressReader<'a, TChannel> {
    ///Initialize the plugin from a channel to DFHack.
    pub fn new(channel: &'a mut TChannel) -> Self {
        Self { channel }
    }
    ///Method `CheckHashes` from the plugin `RemoteFortressReader`
    pub fn check_hashes(&mut self) -> Result<(), TChannel::TError> {
        let request = EmptyMessage::new();
        let _response: EmptyMessage = self
            .channel
            .request(
                "RemoteFortressReader".to_string(),
                "CheckHashes".to_string(),
                request,
            )?;
        let _response = ();
        Ok(_response)
    }
    ///Method `CopyScreen` from the plugin `RemoteFortressReader`
    pub fn copy_screen(&mut self) -> Result<ScreenCapture, TChannel::TError> {
        let request = EmptyMessage::new();
        let _response: ScreenCapture = self
            .channel
            .request(
                "RemoteFortressReader".to_string(),
                "CopyScreen".to_string(),
                request,
            )?;
        Ok(_response)
    }
    ///Method `GetBlockList` from the plugin `RemoteFortressReader`
    pub fn get_block_list(
        &mut self,
        request: BlockRequest,
    ) -> Result<BlockList, TChannel::TError> {
        let _response: BlockList = self
            .channel
            .request(
                "RemoteFortressReader".to_string(),
                "GetBlockList".to_string(),
                request,
            )?;
        Ok(_response)
    }
    ///Method `GetBuildingDefList` from the plugin `RemoteFortressReader`
    pub fn get_building_def_list(&mut self) -> Result<BuildingList, TChannel::TError> {
        let request = EmptyMessage::new();
        let _response: BuildingList = self
            .channel
            .request(
                "RemoteFortressReader".to_string(),
                "GetBuildingDefList".to_string(),
                request,
            )?;
        Ok(_response)
    }
    ///Method `GetCreatureRaws` from the plugin `RemoteFortressReader`
    pub fn get_creature_raws(&mut self) -> Result<CreatureRawList, TChannel::TError> {
        let request = EmptyMessage::new();
        let _response: CreatureRawList = self
            .channel
            .request(
                "RemoteFortressReader".to_string(),
                "GetCreatureRaws".to_string(),
                request,
            )?;
        Ok(_response)
    }
    ///Method `GetGameValidity` from the plugin `RemoteFortressReader`
    pub fn get_game_validity(&mut self) -> Result<bool, TChannel::TError> {
        let request = EmptyMessage::new();
        let _response: SingleBool = self
            .channel
            .request(
                "RemoteFortressReader".to_string(),
                "GetGameValidity".to_string(),
                request,
            )?;
        let _response = _response.Value();
        Ok(_response)
    }
    ///Method `GetGrowthList` from the plugin `RemoteFortressReader`
    pub fn get_growth_list(&mut self) -> Result<MaterialList, TChannel::TError> {
        let request = EmptyMessage::new();
        let _response: MaterialList = self
            .channel
            .request(
                "RemoteFortressReader".to_string(),
                "GetGrowthList".to_string(),
                request,
            )?;
        Ok(_response)
    }
    ///Method `GetItemList` from the plugin `RemoteFortressReader`
    pub fn get_item_list(&mut self) -> Result<MaterialList, TChannel::TError> {
        let request = EmptyMessage::new();
        let _response: MaterialList = self
            .channel
            .request(
                "RemoteFortressReader".to_string(),
                "GetItemList".to_string(),
                request,
            )?;
        Ok(_response)
    }
    ///Method `GetLanguage` from the plugin `RemoteFortressReader`
    pub fn get_language(&mut self) -> Result<Language, TChannel::TError> {
        let request = EmptyMessage::new();
        let _response: Language = self
            .channel
            .request(
                "RemoteFortressReader".to_string(),
                "GetLanguage".to_string(),
                request,
            )?;
        Ok(_response)
    }
    ///Method `GetMapInfo` from the plugin `RemoteFortressReader`
    pub fn get_map_info(&mut self) -> Result<MapInfo, TChannel::TError> {
        let request = EmptyMessage::new();
        let _response: MapInfo = self
            .channel
            .request(
                "RemoteFortressReader".to_string(),
                "GetMapInfo".to_string(),
                request,
            )?;
        Ok(_response)
    }
    ///Method `GetMaterialList` from the plugin `RemoteFortressReader`
    pub fn get_material_list(&mut self) -> Result<MaterialList, TChannel::TError> {
        let request = EmptyMessage::new();
        let _response: MaterialList = self
            .channel
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
    ) -> Result<CreatureRawList, TChannel::TError> {
        let _response: CreatureRawList = self
            .channel
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
    ) -> Result<PlantRawList, TChannel::TError> {
        let _response: PlantRawList = self
            .channel
            .request(
                "RemoteFortressReader".to_string(),
                "GetPartialPlantRaws".to_string(),
                request,
            )?;
        Ok(_response)
    }
    ///Method `GetPauseState` from the plugin `RemoteFortressReader`
    pub fn get_pause_state(&mut self) -> Result<bool, TChannel::TError> {
        let request = EmptyMessage::new();
        let _response: SingleBool = self
            .channel
            .request(
                "RemoteFortressReader".to_string(),
                "GetPauseState".to_string(),
                request,
            )?;
        let _response = _response.Value();
        Ok(_response)
    }
    ///Method `GetPlantList` from the plugin `RemoteFortressReader`
    pub fn get_plant_list(
        &mut self,
        request: BlockRequest,
    ) -> Result<PlantList, TChannel::TError> {
        let _response: PlantList = self
            .channel
            .request(
                "RemoteFortressReader".to_string(),
                "GetPlantList".to_string(),
                request,
            )?;
        Ok(_response)
    }
    ///Method `GetPlantRaws` from the plugin `RemoteFortressReader`
    pub fn get_plant_raws(&mut self) -> Result<PlantRawList, TChannel::TError> {
        let request = EmptyMessage::new();
        let _response: PlantRawList = self
            .channel
            .request(
                "RemoteFortressReader".to_string(),
                "GetPlantRaws".to_string(),
                request,
            )?;
        Ok(_response)
    }
    ///Method `GetRegionMaps` from the plugin `RemoteFortressReader`
    pub fn get_region_maps(&mut self) -> Result<RegionMaps, TChannel::TError> {
        let request = EmptyMessage::new();
        let _response: RegionMaps = self
            .channel
            .request(
                "RemoteFortressReader".to_string(),
                "GetRegionMaps".to_string(),
                request,
            )?;
        Ok(_response)
    }
    ///Method `GetRegionMapsNew` from the plugin `RemoteFortressReader`
    pub fn get_region_maps_new(&mut self) -> Result<RegionMaps, TChannel::TError> {
        let request = EmptyMessage::new();
        let _response: RegionMaps = self
            .channel
            .request(
                "RemoteFortressReader".to_string(),
                "GetRegionMapsNew".to_string(),
                request,
            )?;
        Ok(_response)
    }
    ///Method `GetReports` from the plugin `RemoteFortressReader`
    pub fn get_reports(&mut self) -> Result<Status, TChannel::TError> {
        let request = EmptyMessage::new();
        let _response: Status = self
            .channel
            .request(
                "RemoteFortressReader".to_string(),
                "GetReports".to_string(),
                request,
            )?;
        Ok(_response)
    }
    ///Method `GetSideMenu` from the plugin `RemoteFortressReader`
    pub fn get_side_menu(&mut self) -> Result<SidebarState, TChannel::TError> {
        let request = EmptyMessage::new();
        let _response: SidebarState = self
            .channel
            .request(
                "RemoteFortressReader".to_string(),
                "GetSideMenu".to_string(),
                request,
            )?;
        Ok(_response)
    }
    ///Method `GetTiletypeList` from the plugin `RemoteFortressReader`
    pub fn get_tiletype_list(&mut self) -> Result<TiletypeList, TChannel::TError> {
        let request = EmptyMessage::new();
        let _response: TiletypeList = self
            .channel
            .request(
                "RemoteFortressReader".to_string(),
                "GetTiletypeList".to_string(),
                request,
            )?;
        Ok(_response)
    }
    ///Method `GetUnitList` from the plugin `RemoteFortressReader`
    pub fn get_unit_list(&mut self) -> Result<UnitList, TChannel::TError> {
        let request = EmptyMessage::new();
        let _response: UnitList = self
            .channel
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
    ) -> Result<UnitList, TChannel::TError> {
        let _response: UnitList = self
            .channel
            .request(
                "RemoteFortressReader".to_string(),
                "GetUnitListInside".to_string(),
                request,
            )?;
        Ok(_response)
    }
    ///Method `GetVersionInfo` from the plugin `RemoteFortressReader`
    pub fn get_version_info(&mut self) -> Result<VersionInfo, TChannel::TError> {
        let request = EmptyMessage::new();
        let _response: VersionInfo = self
            .channel
            .request(
                "RemoteFortressReader".to_string(),
                "GetVersionInfo".to_string(),
                request,
            )?;
        Ok(_response)
    }
    ///Method `GetViewInfo` from the plugin `RemoteFortressReader`
    pub fn get_view_info(&mut self) -> Result<ViewInfo, TChannel::TError> {
        let request = EmptyMessage::new();
        let _response: ViewInfo = self
            .channel
            .request(
                "RemoteFortressReader".to_string(),
                "GetViewInfo".to_string(),
                request,
            )?;
        Ok(_response)
    }
    ///Method `GetWorldMap` from the plugin `RemoteFortressReader`
    pub fn get_world_map(&mut self) -> Result<WorldMap, TChannel::TError> {
        let request = EmptyMessage::new();
        let _response: WorldMap = self
            .channel
            .request(
                "RemoteFortressReader".to_string(),
                "GetWorldMap".to_string(),
                request,
            )?;
        Ok(_response)
    }
    ///Method `GetWorldMapCenter` from the plugin `RemoteFortressReader`
    pub fn get_world_map_center(&mut self) -> Result<WorldMap, TChannel::TError> {
        let request = EmptyMessage::new();
        let _response: WorldMap = self
            .channel
            .request(
                "RemoteFortressReader".to_string(),
                "GetWorldMapCenter".to_string(),
                request,
            )?;
        Ok(_response)
    }
    ///Method `GetWorldMapNew` from the plugin `RemoteFortressReader`
    pub fn get_world_map_new(&mut self) -> Result<WorldMap, TChannel::TError> {
        let request = EmptyMessage::new();
        let _response: WorldMap = self
            .channel
            .request(
                "RemoteFortressReader".to_string(),
                "GetWorldMapNew".to_string(),
                request,
            )?;
        Ok(_response)
    }
    ///Method `JumpCommand` from the plugin `RemoteFortressReader`
    pub fn jump_command(
        &mut self,
        request: MoveCommandParams,
    ) -> Result<(), TChannel::TError> {
        let _response: EmptyMessage = self
            .channel
            .request(
                "RemoteFortressReader".to_string(),
                "JumpCommand".to_string(),
                request,
            )?;
        let _response = ();
        Ok(_response)
    }
    ///Method `MenuQuery` from the plugin `RemoteFortressReader`
    pub fn menu_query(&mut self) -> Result<MenuContents, TChannel::TError> {
        let request = EmptyMessage::new();
        let _response: MenuContents = self
            .channel
            .request(
                "RemoteFortressReader".to_string(),
                "MenuQuery".to_string(),
                request,
            )?;
        Ok(_response)
    }
    ///Method `MiscMoveCommand` from the plugin `RemoteFortressReader`
    pub fn misc_move_command(
        &mut self,
        request: MiscMoveParams,
    ) -> Result<(), TChannel::TError> {
        let _response: EmptyMessage = self
            .channel
            .request(
                "RemoteFortressReader".to_string(),
                "MiscMoveCommand".to_string(),
                request,
            )?;
        let _response = ();
        Ok(_response)
    }
    ///Method `MoveCommand` from the plugin `RemoteFortressReader`
    pub fn move_command(
        &mut self,
        request: MoveCommandParams,
    ) -> Result<(), TChannel::TError> {
        let _response: EmptyMessage = self
            .channel
            .request(
                "RemoteFortressReader".to_string(),
                "MoveCommand".to_string(),
                request,
            )?;
        let _response = ();
        Ok(_response)
    }
    ///Method `MovementSelectCommand` from the plugin `RemoteFortressReader`
    pub fn movement_select_command(
        &mut self,
        value: i32,
    ) -> Result<(), TChannel::TError> {
        let mut request = IntMessage::new();
        request.set_value(value);
        let _response: EmptyMessage = self
            .channel
            .request(
                "RemoteFortressReader".to_string(),
                "MovementSelectCommand".to_string(),
                request,
            )?;
        let _response = ();
        Ok(_response)
    }
    ///Method `PassKeyboardEvent` from the plugin `RemoteFortressReader`
    pub fn pass_keyboard_event(
        &mut self,
        request: KeyboardEvent,
    ) -> Result<(), TChannel::TError> {
        let _response: EmptyMessage = self
            .channel
            .request(
                "RemoteFortressReader".to_string(),
                "PassKeyboardEvent".to_string(),
                request,
            )?;
        let _response = ();
        Ok(_response)
    }
    ///Method `ResetMapHashes` from the plugin `RemoteFortressReader`
    pub fn reset_map_hashes(&mut self) -> Result<(), TChannel::TError> {
        let request = EmptyMessage::new();
        let _response: EmptyMessage = self
            .channel
            .request(
                "RemoteFortressReader".to_string(),
                "ResetMapHashes".to_string(),
                request,
            )?;
        let _response = ();
        Ok(_response)
    }
    ///Method `SendDigCommand` from the plugin `RemoteFortressReader`
    pub fn send_dig_command(
        &mut self,
        request: DigCommand,
    ) -> Result<(), TChannel::TError> {
        let _response: EmptyMessage = self
            .channel
            .request(
                "RemoteFortressReader".to_string(),
                "SendDigCommand".to_string(),
                request,
            )?;
        let _response = ();
        Ok(_response)
    }
    ///Method `SetPauseState` from the plugin `RemoteFortressReader`
    pub fn set_pause_state(&mut self, value: bool) -> Result<(), TChannel::TError> {
        let mut request = SingleBool::new();
        request.set_Value(value);
        let _response: EmptyMessage = self
            .channel
            .request(
                "RemoteFortressReader".to_string(),
                "SetPauseState".to_string(),
                request,
            )?;
        let _response = ();
        Ok(_response)
    }
    ///Method `SetSideMenu` from the plugin `RemoteFortressReader`
    pub fn set_side_menu(
        &mut self,
        request: SidebarCommand,
    ) -> Result<(), TChannel::TError> {
        let _response: EmptyMessage = self
            .channel
            .request(
                "RemoteFortressReader".to_string(),
                "SetSideMenu".to_string(),
                request,
            )?;
        let _response = ();
        Ok(_response)
    }
}
#[cfg(feature = "reflection")]
impl<TChannel: crate::Channel> crate::reflection::StubReflection
for RemoteFortressReader<'_, TChannel> {
    fn list_methods() -> Vec<crate::reflection::RemoteProcedureDescriptor> {
        vec![
            crate ::reflection::RemoteProcedureDescriptor { name : "CheckHashes"
            .to_string(), plugin_name : "RemoteFortressReader".to_string(), input_type :
            EmptyMessage::descriptor().full_name().to_string(), output_type :
            EmptyMessage::descriptor().full_name().to_string(), }, crate
            ::reflection::RemoteProcedureDescriptor { name : "CopyScreen".to_string(),
            plugin_name : "RemoteFortressReader".to_string(), input_type :
            EmptyMessage::descriptor().full_name().to_string(), output_type :
            ScreenCapture::descriptor().full_name().to_string(), }, crate
            ::reflection::RemoteProcedureDescriptor { name : "GetBlockList".to_string(),
            plugin_name : "RemoteFortressReader".to_string(), input_type :
            BlockRequest::descriptor().full_name().to_string(), output_type :
            BlockList::descriptor().full_name().to_string(), }, crate
            ::reflection::RemoteProcedureDescriptor { name : "GetBuildingDefList"
            .to_string(), plugin_name : "RemoteFortressReader".to_string(), input_type :
            EmptyMessage::descriptor().full_name().to_string(), output_type :
            BuildingList::descriptor().full_name().to_string(), }, crate
            ::reflection::RemoteProcedureDescriptor { name : "GetCreatureRaws"
            .to_string(), plugin_name : "RemoteFortressReader".to_string(), input_type :
            EmptyMessage::descriptor().full_name().to_string(), output_type :
            CreatureRawList::descriptor().full_name().to_string(), }, crate
            ::reflection::RemoteProcedureDescriptor { name : "GetGameValidity"
            .to_string(), plugin_name : "RemoteFortressReader".to_string(), input_type :
            EmptyMessage::descriptor().full_name().to_string(), output_type :
            SingleBool::descriptor().full_name().to_string(), }, crate
            ::reflection::RemoteProcedureDescriptor { name : "GetGrowthList".to_string(),
            plugin_name : "RemoteFortressReader".to_string(), input_type :
            EmptyMessage::descriptor().full_name().to_string(), output_type :
            MaterialList::descriptor().full_name().to_string(), }, crate
            ::reflection::RemoteProcedureDescriptor { name : "GetItemList".to_string(),
            plugin_name : "RemoteFortressReader".to_string(), input_type :
            EmptyMessage::descriptor().full_name().to_string(), output_type :
            MaterialList::descriptor().full_name().to_string(), }, crate
            ::reflection::RemoteProcedureDescriptor { name : "GetLanguage".to_string(),
            plugin_name : "RemoteFortressReader".to_string(), input_type :
            EmptyMessage::descriptor().full_name().to_string(), output_type :
            Language::descriptor().full_name().to_string(), }, crate
            ::reflection::RemoteProcedureDescriptor { name : "GetMapInfo".to_string(),
            plugin_name : "RemoteFortressReader".to_string(), input_type :
            EmptyMessage::descriptor().full_name().to_string(), output_type :
            MapInfo::descriptor().full_name().to_string(), }, crate
            ::reflection::RemoteProcedureDescriptor { name : "GetMaterialList"
            .to_string(), plugin_name : "RemoteFortressReader".to_string(), input_type :
            EmptyMessage::descriptor().full_name().to_string(), output_type :
            MaterialList::descriptor().full_name().to_string(), }, crate
            ::reflection::RemoteProcedureDescriptor { name : "GetPartialCreatureRaws"
            .to_string(), plugin_name : "RemoteFortressReader".to_string(), input_type :
            ListRequest::descriptor().full_name().to_string(), output_type :
            CreatureRawList::descriptor().full_name().to_string(), }, crate
            ::reflection::RemoteProcedureDescriptor { name : "GetPartialPlantRaws"
            .to_string(), plugin_name : "RemoteFortressReader".to_string(), input_type :
            ListRequest::descriptor().full_name().to_string(), output_type :
            PlantRawList::descriptor().full_name().to_string(), }, crate
            ::reflection::RemoteProcedureDescriptor { name : "GetPauseState".to_string(),
            plugin_name : "RemoteFortressReader".to_string(), input_type :
            EmptyMessage::descriptor().full_name().to_string(), output_type :
            SingleBool::descriptor().full_name().to_string(), }, crate
            ::reflection::RemoteProcedureDescriptor { name : "GetPlantList".to_string(),
            plugin_name : "RemoteFortressReader".to_string(), input_type :
            BlockRequest::descriptor().full_name().to_string(), output_type :
            PlantList::descriptor().full_name().to_string(), }, crate
            ::reflection::RemoteProcedureDescriptor { name : "GetPlantRaws".to_string(),
            plugin_name : "RemoteFortressReader".to_string(), input_type :
            EmptyMessage::descriptor().full_name().to_string(), output_type :
            PlantRawList::descriptor().full_name().to_string(), }, crate
            ::reflection::RemoteProcedureDescriptor { name : "GetRegionMaps".to_string(),
            plugin_name : "RemoteFortressReader".to_string(), input_type :
            EmptyMessage::descriptor().full_name().to_string(), output_type :
            RegionMaps::descriptor().full_name().to_string(), }, crate
            ::reflection::RemoteProcedureDescriptor { name : "GetRegionMapsNew"
            .to_string(), plugin_name : "RemoteFortressReader".to_string(), input_type :
            EmptyMessage::descriptor().full_name().to_string(), output_type :
            RegionMaps::descriptor().full_name().to_string(), }, crate
            ::reflection::RemoteProcedureDescriptor { name : "GetReports".to_string(),
            plugin_name : "RemoteFortressReader".to_string(), input_type :
            EmptyMessage::descriptor().full_name().to_string(), output_type :
            Status::descriptor().full_name().to_string(), }, crate
            ::reflection::RemoteProcedureDescriptor { name : "GetSideMenu".to_string(),
            plugin_name : "RemoteFortressReader".to_string(), input_type :
            EmptyMessage::descriptor().full_name().to_string(), output_type :
            SidebarState::descriptor().full_name().to_string(), }, crate
            ::reflection::RemoteProcedureDescriptor { name : "GetTiletypeList"
            .to_string(), plugin_name : "RemoteFortressReader".to_string(), input_type :
            EmptyMessage::descriptor().full_name().to_string(), output_type :
            TiletypeList::descriptor().full_name().to_string(), }, crate
            ::reflection::RemoteProcedureDescriptor { name : "GetUnitList".to_string(),
            plugin_name : "RemoteFortressReader".to_string(), input_type :
            EmptyMessage::descriptor().full_name().to_string(), output_type :
            UnitList::descriptor().full_name().to_string(), }, crate
            ::reflection::RemoteProcedureDescriptor { name : "GetUnitListInside"
            .to_string(), plugin_name : "RemoteFortressReader".to_string(), input_type :
            BlockRequest::descriptor().full_name().to_string(), output_type :
            UnitList::descriptor().full_name().to_string(), }, crate
            ::reflection::RemoteProcedureDescriptor { name : "GetVersionInfo"
            .to_string(), plugin_name : "RemoteFortressReader".to_string(), input_type :
            EmptyMessage::descriptor().full_name().to_string(), output_type :
            VersionInfo::descriptor().full_name().to_string(), }, crate
            ::reflection::RemoteProcedureDescriptor { name : "GetViewInfo".to_string(),
            plugin_name : "RemoteFortressReader".to_string(), input_type :
            EmptyMessage::descriptor().full_name().to_string(), output_type :
            ViewInfo::descriptor().full_name().to_string(), }, crate
            ::reflection::RemoteProcedureDescriptor { name : "GetWorldMap".to_string(),
            plugin_name : "RemoteFortressReader".to_string(), input_type :
            EmptyMessage::descriptor().full_name().to_string(), output_type :
            WorldMap::descriptor().full_name().to_string(), }, crate
            ::reflection::RemoteProcedureDescriptor { name : "GetWorldMapCenter"
            .to_string(), plugin_name : "RemoteFortressReader".to_string(), input_type :
            EmptyMessage::descriptor().full_name().to_string(), output_type :
            WorldMap::descriptor().full_name().to_string(), }, crate
            ::reflection::RemoteProcedureDescriptor { name : "GetWorldMapNew"
            .to_string(), plugin_name : "RemoteFortressReader".to_string(), input_type :
            EmptyMessage::descriptor().full_name().to_string(), output_type :
            WorldMap::descriptor().full_name().to_string(), }, crate
            ::reflection::RemoteProcedureDescriptor { name : "JumpCommand".to_string(),
            plugin_name : "RemoteFortressReader".to_string(), input_type :
            MoveCommandParams::descriptor().full_name().to_string(), output_type :
            EmptyMessage::descriptor().full_name().to_string(), }, crate
            ::reflection::RemoteProcedureDescriptor { name : "MenuQuery".to_string(),
            plugin_name : "RemoteFortressReader".to_string(), input_type :
            EmptyMessage::descriptor().full_name().to_string(), output_type :
            MenuContents::descriptor().full_name().to_string(), }, crate
            ::reflection::RemoteProcedureDescriptor { name : "MiscMoveCommand"
            .to_string(), plugin_name : "RemoteFortressReader".to_string(), input_type :
            MiscMoveParams::descriptor().full_name().to_string(), output_type :
            EmptyMessage::descriptor().full_name().to_string(), }, crate
            ::reflection::RemoteProcedureDescriptor { name : "MoveCommand".to_string(),
            plugin_name : "RemoteFortressReader".to_string(), input_type :
            MoveCommandParams::descriptor().full_name().to_string(), output_type :
            EmptyMessage::descriptor().full_name().to_string(), }, crate
            ::reflection::RemoteProcedureDescriptor { name : "MovementSelectCommand"
            .to_string(), plugin_name : "RemoteFortressReader".to_string(), input_type :
            IntMessage::descriptor().full_name().to_string(), output_type :
            EmptyMessage::descriptor().full_name().to_string(), }, crate
            ::reflection::RemoteProcedureDescriptor { name : "PassKeyboardEvent"
            .to_string(), plugin_name : "RemoteFortressReader".to_string(), input_type :
            KeyboardEvent::descriptor().full_name().to_string(), output_type :
            EmptyMessage::descriptor().full_name().to_string(), }, crate
            ::reflection::RemoteProcedureDescriptor { name : "ResetMapHashes"
            .to_string(), plugin_name : "RemoteFortressReader".to_string(), input_type :
            EmptyMessage::descriptor().full_name().to_string(), output_type :
            EmptyMessage::descriptor().full_name().to_string(), }, crate
            ::reflection::RemoteProcedureDescriptor { name : "SendDigCommand"
            .to_string(), plugin_name : "RemoteFortressReader".to_string(), input_type :
            DigCommand::descriptor().full_name().to_string(), output_type :
            EmptyMessage::descriptor().full_name().to_string(), }, crate
            ::reflection::RemoteProcedureDescriptor { name : "SetPauseState".to_string(),
            plugin_name : "RemoteFortressReader".to_string(), input_type :
            SingleBool::descriptor().full_name().to_string(), output_type :
            EmptyMessage::descriptor().full_name().to_string(), }, crate
            ::reflection::RemoteProcedureDescriptor { name : "SetSideMenu".to_string(),
            plugin_name : "RemoteFortressReader".to_string(), input_type :
            SidebarCommand::descriptor().full_name().to_string(), output_type :
            EmptyMessage::descriptor().full_name().to_string(), },
        ]
    }
}
///RPC for the "isoworldremote" plugin.
pub struct Isoworldremote<'a, TChannel: crate::Channel> {
    ///Reference to the client to exchange messages.
    pub channel: &'a mut TChannel,
}
impl<'a, TChannel: crate::Channel> Isoworldremote<'a, TChannel> {
    ///Initialize the plugin from a channel to DFHack.
    pub fn new(channel: &'a mut TChannel) -> Self {
        Self { channel }
    }
    ///Method `GetEmbarkInfo` from the plugin `isoworldremote`
    pub fn get_embark_info(
        &mut self,
        request: MapRequest,
    ) -> Result<MapReply, TChannel::TError> {
        let _response: MapReply = self
            .channel
            .request(
                "isoworldremote".to_string(),
                "GetEmbarkInfo".to_string(),
                request,
            )?;
        Ok(_response)
    }
    ///Method `GetEmbarkTile` from the plugin `isoworldremote`
    pub fn get_embark_tile(
        &mut self,
        request: TileRequest,
    ) -> Result<EmbarkTile, TChannel::TError> {
        let _response: EmbarkTile = self
            .channel
            .request(
                "isoworldremote".to_string(),
                "GetEmbarkTile".to_string(),
                request,
            )?;
        Ok(_response)
    }
    ///Method `GetRawNames` from the plugin `isoworldremote`
    pub fn get_raw_names(
        &mut self,
        request: MapRequest,
    ) -> Result<RawNames, TChannel::TError> {
        let _response: RawNames = self
            .channel
            .request("isoworldremote".to_string(), "GetRawNames".to_string(), request)?;
        Ok(_response)
    }
}
#[cfg(feature = "reflection")]
impl<TChannel: crate::Channel> crate::reflection::StubReflection
for Isoworldremote<'_, TChannel> {
    fn list_methods() -> Vec<crate::reflection::RemoteProcedureDescriptor> {
        vec![
            crate ::reflection::RemoteProcedureDescriptor { name : "GetEmbarkInfo"
            .to_string(), plugin_name : "isoworldremote".to_string(), input_type :
            MapRequest::descriptor().full_name().to_string(), output_type :
            MapReply::descriptor().full_name().to_string(), }, crate
            ::reflection::RemoteProcedureDescriptor { name : "GetEmbarkTile".to_string(),
            plugin_name : "isoworldremote".to_string(), input_type :
            TileRequest::descriptor().full_name().to_string(), output_type :
            EmbarkTile::descriptor().full_name().to_string(), }, crate
            ::reflection::RemoteProcedureDescriptor { name : "GetRawNames".to_string(),
            plugin_name : "isoworldremote".to_string(), input_type :
            MapRequest::descriptor().full_name().to_string(), output_type :
            RawNames::descriptor().full_name().to_string(), },
        ]
    }
}
///RPC for the "rename" plugin.
pub struct Rename<'a, TChannel: crate::Channel> {
    ///Reference to the client to exchange messages.
    pub channel: &'a mut TChannel,
}
impl<'a, TChannel: crate::Channel> Rename<'a, TChannel> {
    ///Initialize the plugin from a channel to DFHack.
    pub fn new(channel: &'a mut TChannel) -> Self {
        Self { channel }
    }
    ///Method `RenameBuilding` from the plugin `rename`
    pub fn rename_building(
        &mut self,
        request: RenameBuildingIn,
    ) -> Result<(), TChannel::TError> {
        let _response: EmptyMessage = self
            .channel
            .request("rename".to_string(), "RenameBuilding".to_string(), request)?;
        let _response = ();
        Ok(_response)
    }
    ///Method `RenameSquad` from the plugin `rename`
    pub fn rename_squad(
        &mut self,
        request: RenameSquadIn,
    ) -> Result<(), TChannel::TError> {
        let _response: EmptyMessage = self
            .channel
            .request("rename".to_string(), "RenameSquad".to_string(), request)?;
        let _response = ();
        Ok(_response)
    }
    ///Method `RenameUnit` from the plugin `rename`
    pub fn rename_unit(
        &mut self,
        request: RenameUnitIn,
    ) -> Result<(), TChannel::TError> {
        let _response: EmptyMessage = self
            .channel
            .request("rename".to_string(), "RenameUnit".to_string(), request)?;
        let _response = ();
        Ok(_response)
    }
}
#[cfg(feature = "reflection")]
impl<TChannel: crate::Channel> crate::reflection::StubReflection
for Rename<'_, TChannel> {
    fn list_methods() -> Vec<crate::reflection::RemoteProcedureDescriptor> {
        vec![
            crate ::reflection::RemoteProcedureDescriptor { name : "RenameBuilding"
            .to_string(), plugin_name : "rename".to_string(), input_type :
            RenameBuildingIn::descriptor().full_name().to_string(), output_type :
            EmptyMessage::descriptor().full_name().to_string(), }, crate
            ::reflection::RemoteProcedureDescriptor { name : "RenameSquad".to_string(),
            plugin_name : "rename".to_string(), input_type : RenameSquadIn::descriptor()
            .full_name().to_string(), output_type : EmptyMessage::descriptor()
            .full_name().to_string(), }, crate ::reflection::RemoteProcedureDescriptor {
            name : "RenameUnit".to_string(), plugin_name : "rename".to_string(),
            input_type : RenameUnitIn::descriptor().full_name().to_string(), output_type
            : EmptyMessage::descriptor().full_name().to_string(), },
        ]
    }
}
