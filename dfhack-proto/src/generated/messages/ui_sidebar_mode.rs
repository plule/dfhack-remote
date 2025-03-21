// This file is generated by rust-protobuf 3.7.2. Do not edit
// .proto file is parsed by pure
// @generated

// https://github.com/rust-lang/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![allow(unused_attributes)]
#![cfg_attr(rustfmt, rustfmt::skip)]

#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unused_results)]
#![allow(unused_mut)]

//! Generated file from `ui_sidebar_mode.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_2;

#[derive(Clone,Copy,PartialEq,Eq,Debug,Hash)]
// @@protoc_insertion_point(enum:proto.enums.ui_sidebar_mode.ui_sidebar_mode)
pub enum Ui_sidebar_mode {
    // @@protoc_insertion_point(enum_value:proto.enums.ui_sidebar_mode.ui_sidebar_mode.Default)
    Default = 0,
    // @@protoc_insertion_point(enum_value:proto.enums.ui_sidebar_mode.ui_sidebar_mode.Squads)
    Squads = 1,
    // @@protoc_insertion_point(enum_value:proto.enums.ui_sidebar_mode.ui_sidebar_mode.DesignateMine)
    DesignateMine = 2,
    // @@protoc_insertion_point(enum_value:proto.enums.ui_sidebar_mode.ui_sidebar_mode.DesignateRemoveRamps)
    DesignateRemoveRamps = 3,
    // @@protoc_insertion_point(enum_value:proto.enums.ui_sidebar_mode.ui_sidebar_mode.DesignateUpStair)
    DesignateUpStair = 4,
    // @@protoc_insertion_point(enum_value:proto.enums.ui_sidebar_mode.ui_sidebar_mode.DesignateDownStair)
    DesignateDownStair = 5,
    // @@protoc_insertion_point(enum_value:proto.enums.ui_sidebar_mode.ui_sidebar_mode.DesignateUpDownStair)
    DesignateUpDownStair = 6,
    // @@protoc_insertion_point(enum_value:proto.enums.ui_sidebar_mode.ui_sidebar_mode.DesignateUpRamp)
    DesignateUpRamp = 7,
    // @@protoc_insertion_point(enum_value:proto.enums.ui_sidebar_mode.ui_sidebar_mode.DesignateChannel)
    DesignateChannel = 8,
    // @@protoc_insertion_point(enum_value:proto.enums.ui_sidebar_mode.ui_sidebar_mode.DesignateGatherPlants)
    DesignateGatherPlants = 9,
    // @@protoc_insertion_point(enum_value:proto.enums.ui_sidebar_mode.ui_sidebar_mode.DesignateRemoveDesignation)
    DesignateRemoveDesignation = 10,
    // @@protoc_insertion_point(enum_value:proto.enums.ui_sidebar_mode.ui_sidebar_mode.DesignateSmooth)
    DesignateSmooth = 11,
    // @@protoc_insertion_point(enum_value:proto.enums.ui_sidebar_mode.ui_sidebar_mode.DesignateCarveTrack)
    DesignateCarveTrack = 12,
    // @@protoc_insertion_point(enum_value:proto.enums.ui_sidebar_mode.ui_sidebar_mode.DesignateEngrave)
    DesignateEngrave = 13,
    // @@protoc_insertion_point(enum_value:proto.enums.ui_sidebar_mode.ui_sidebar_mode.DesignateCarveFortification)
    DesignateCarveFortification = 14,
    // @@protoc_insertion_point(enum_value:proto.enums.ui_sidebar_mode.ui_sidebar_mode.Stockpiles)
    Stockpiles = 15,
    // @@protoc_insertion_point(enum_value:proto.enums.ui_sidebar_mode.ui_sidebar_mode.Build)
    Build = 16,
    // @@protoc_insertion_point(enum_value:proto.enums.ui_sidebar_mode.ui_sidebar_mode.QueryBuilding)
    QueryBuilding = 17,
    // @@protoc_insertion_point(enum_value:proto.enums.ui_sidebar_mode.ui_sidebar_mode.Orders)
    Orders = 18,
    // @@protoc_insertion_point(enum_value:proto.enums.ui_sidebar_mode.ui_sidebar_mode.OrdersForbid)
    OrdersForbid = 19,
    // @@protoc_insertion_point(enum_value:proto.enums.ui_sidebar_mode.ui_sidebar_mode.OrdersRefuse)
    OrdersRefuse = 20,
    // @@protoc_insertion_point(enum_value:proto.enums.ui_sidebar_mode.ui_sidebar_mode.OrdersWorkshop)
    OrdersWorkshop = 21,
    // @@protoc_insertion_point(enum_value:proto.enums.ui_sidebar_mode.ui_sidebar_mode.OrdersZone)
    OrdersZone = 22,
    // @@protoc_insertion_point(enum_value:proto.enums.ui_sidebar_mode.ui_sidebar_mode.BuildingItems)
    BuildingItems = 23,
    // @@protoc_insertion_point(enum_value:proto.enums.ui_sidebar_mode.ui_sidebar_mode.ViewUnits)
    ViewUnits = 24,
    // @@protoc_insertion_point(enum_value:proto.enums.ui_sidebar_mode.ui_sidebar_mode.LookAround)
    LookAround = 25,
    // @@protoc_insertion_point(enum_value:proto.enums.ui_sidebar_mode.ui_sidebar_mode.DesignateItemsClaim)
    DesignateItemsClaim = 26,
    // @@protoc_insertion_point(enum_value:proto.enums.ui_sidebar_mode.ui_sidebar_mode.DesignateItemsForbid)
    DesignateItemsForbid = 27,
    // @@protoc_insertion_point(enum_value:proto.enums.ui_sidebar_mode.ui_sidebar_mode.DesignateItemsMelt)
    DesignateItemsMelt = 28,
    // @@protoc_insertion_point(enum_value:proto.enums.ui_sidebar_mode.ui_sidebar_mode.DesignateItemsUnmelt)
    DesignateItemsUnmelt = 29,
    // @@protoc_insertion_point(enum_value:proto.enums.ui_sidebar_mode.ui_sidebar_mode.DesignateItemsDump)
    DesignateItemsDump = 30,
    // @@protoc_insertion_point(enum_value:proto.enums.ui_sidebar_mode.ui_sidebar_mode.DesignateItemsUndump)
    DesignateItemsUndump = 31,
    // @@protoc_insertion_point(enum_value:proto.enums.ui_sidebar_mode.ui_sidebar_mode.DesignateItemsHide)
    DesignateItemsHide = 32,
    // @@protoc_insertion_point(enum_value:proto.enums.ui_sidebar_mode.ui_sidebar_mode.DesignateItemsUnhide)
    DesignateItemsUnhide = 33,
    // @@protoc_insertion_point(enum_value:proto.enums.ui_sidebar_mode.ui_sidebar_mode.DesignateChopTrees)
    DesignateChopTrees = 34,
    // @@protoc_insertion_point(enum_value:proto.enums.ui_sidebar_mode.ui_sidebar_mode.DesignateToggleEngravings)
    DesignateToggleEngravings = 35,
    // @@protoc_insertion_point(enum_value:proto.enums.ui_sidebar_mode.ui_sidebar_mode.DesignateToggleMarker)
    DesignateToggleMarker = 36,
    // @@protoc_insertion_point(enum_value:proto.enums.ui_sidebar_mode.ui_sidebar_mode.Hotkeys)
    Hotkeys = 37,
    // @@protoc_insertion_point(enum_value:proto.enums.ui_sidebar_mode.ui_sidebar_mode.DesignateTrafficHigh)
    DesignateTrafficHigh = 38,
    // @@protoc_insertion_point(enum_value:proto.enums.ui_sidebar_mode.ui_sidebar_mode.DesignateTrafficNormal)
    DesignateTrafficNormal = 39,
    // @@protoc_insertion_point(enum_value:proto.enums.ui_sidebar_mode.ui_sidebar_mode.DesignateTrafficLow)
    DesignateTrafficLow = 40,
    // @@protoc_insertion_point(enum_value:proto.enums.ui_sidebar_mode.ui_sidebar_mode.DesignateTrafficRestricted)
    DesignateTrafficRestricted = 41,
    // @@protoc_insertion_point(enum_value:proto.enums.ui_sidebar_mode.ui_sidebar_mode.Zones)
    Zones = 42,
    // @@protoc_insertion_point(enum_value:proto.enums.ui_sidebar_mode.ui_sidebar_mode.ZonesPenInfo)
    ZonesPenInfo = 43,
    // @@protoc_insertion_point(enum_value:proto.enums.ui_sidebar_mode.ui_sidebar_mode.ZonesPitInfo)
    ZonesPitInfo = 44,
    // @@protoc_insertion_point(enum_value:proto.enums.ui_sidebar_mode.ui_sidebar_mode.ZonesHospitalInfo)
    ZonesHospitalInfo = 45,
    // @@protoc_insertion_point(enum_value:proto.enums.ui_sidebar_mode.ui_sidebar_mode.ZonesGatherInfo)
    ZonesGatherInfo = 46,
    // @@protoc_insertion_point(enum_value:proto.enums.ui_sidebar_mode.ui_sidebar_mode.DesignateRemoveConstruction)
    DesignateRemoveConstruction = 47,
    // @@protoc_insertion_point(enum_value:proto.enums.ui_sidebar_mode.ui_sidebar_mode.DepotAccess)
    DepotAccess = 48,
    // @@protoc_insertion_point(enum_value:proto.enums.ui_sidebar_mode.ui_sidebar_mode.NotesPoints)
    NotesPoints = 49,
    // @@protoc_insertion_point(enum_value:proto.enums.ui_sidebar_mode.ui_sidebar_mode.NotesRoutes)
    NotesRoutes = 50,
    // @@protoc_insertion_point(enum_value:proto.enums.ui_sidebar_mode.ui_sidebar_mode.Burrows)
    Burrows = 51,
    // @@protoc_insertion_point(enum_value:proto.enums.ui_sidebar_mode.ui_sidebar_mode.Hauling)
    Hauling = 52,
    // @@protoc_insertion_point(enum_value:proto.enums.ui_sidebar_mode.ui_sidebar_mode.ArenaWeather)
    ArenaWeather = 53,
    // @@protoc_insertion_point(enum_value:proto.enums.ui_sidebar_mode.ui_sidebar_mode.ArenaTrees)
    ArenaTrees = 54,
}

impl ::protobuf::Enum for Ui_sidebar_mode {
    const NAME: &'static str = "ui_sidebar_mode";

    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<Ui_sidebar_mode> {
        match value {
            0 => ::std::option::Option::Some(Ui_sidebar_mode::Default),
            1 => ::std::option::Option::Some(Ui_sidebar_mode::Squads),
            2 => ::std::option::Option::Some(Ui_sidebar_mode::DesignateMine),
            3 => ::std::option::Option::Some(Ui_sidebar_mode::DesignateRemoveRamps),
            4 => ::std::option::Option::Some(Ui_sidebar_mode::DesignateUpStair),
            5 => ::std::option::Option::Some(Ui_sidebar_mode::DesignateDownStair),
            6 => ::std::option::Option::Some(Ui_sidebar_mode::DesignateUpDownStair),
            7 => ::std::option::Option::Some(Ui_sidebar_mode::DesignateUpRamp),
            8 => ::std::option::Option::Some(Ui_sidebar_mode::DesignateChannel),
            9 => ::std::option::Option::Some(Ui_sidebar_mode::DesignateGatherPlants),
            10 => ::std::option::Option::Some(Ui_sidebar_mode::DesignateRemoveDesignation),
            11 => ::std::option::Option::Some(Ui_sidebar_mode::DesignateSmooth),
            12 => ::std::option::Option::Some(Ui_sidebar_mode::DesignateCarveTrack),
            13 => ::std::option::Option::Some(Ui_sidebar_mode::DesignateEngrave),
            14 => ::std::option::Option::Some(Ui_sidebar_mode::DesignateCarveFortification),
            15 => ::std::option::Option::Some(Ui_sidebar_mode::Stockpiles),
            16 => ::std::option::Option::Some(Ui_sidebar_mode::Build),
            17 => ::std::option::Option::Some(Ui_sidebar_mode::QueryBuilding),
            18 => ::std::option::Option::Some(Ui_sidebar_mode::Orders),
            19 => ::std::option::Option::Some(Ui_sidebar_mode::OrdersForbid),
            20 => ::std::option::Option::Some(Ui_sidebar_mode::OrdersRefuse),
            21 => ::std::option::Option::Some(Ui_sidebar_mode::OrdersWorkshop),
            22 => ::std::option::Option::Some(Ui_sidebar_mode::OrdersZone),
            23 => ::std::option::Option::Some(Ui_sidebar_mode::BuildingItems),
            24 => ::std::option::Option::Some(Ui_sidebar_mode::ViewUnits),
            25 => ::std::option::Option::Some(Ui_sidebar_mode::LookAround),
            26 => ::std::option::Option::Some(Ui_sidebar_mode::DesignateItemsClaim),
            27 => ::std::option::Option::Some(Ui_sidebar_mode::DesignateItemsForbid),
            28 => ::std::option::Option::Some(Ui_sidebar_mode::DesignateItemsMelt),
            29 => ::std::option::Option::Some(Ui_sidebar_mode::DesignateItemsUnmelt),
            30 => ::std::option::Option::Some(Ui_sidebar_mode::DesignateItemsDump),
            31 => ::std::option::Option::Some(Ui_sidebar_mode::DesignateItemsUndump),
            32 => ::std::option::Option::Some(Ui_sidebar_mode::DesignateItemsHide),
            33 => ::std::option::Option::Some(Ui_sidebar_mode::DesignateItemsUnhide),
            34 => ::std::option::Option::Some(Ui_sidebar_mode::DesignateChopTrees),
            35 => ::std::option::Option::Some(Ui_sidebar_mode::DesignateToggleEngravings),
            36 => ::std::option::Option::Some(Ui_sidebar_mode::DesignateToggleMarker),
            37 => ::std::option::Option::Some(Ui_sidebar_mode::Hotkeys),
            38 => ::std::option::Option::Some(Ui_sidebar_mode::DesignateTrafficHigh),
            39 => ::std::option::Option::Some(Ui_sidebar_mode::DesignateTrafficNormal),
            40 => ::std::option::Option::Some(Ui_sidebar_mode::DesignateTrafficLow),
            41 => ::std::option::Option::Some(Ui_sidebar_mode::DesignateTrafficRestricted),
            42 => ::std::option::Option::Some(Ui_sidebar_mode::Zones),
            43 => ::std::option::Option::Some(Ui_sidebar_mode::ZonesPenInfo),
            44 => ::std::option::Option::Some(Ui_sidebar_mode::ZonesPitInfo),
            45 => ::std::option::Option::Some(Ui_sidebar_mode::ZonesHospitalInfo),
            46 => ::std::option::Option::Some(Ui_sidebar_mode::ZonesGatherInfo),
            47 => ::std::option::Option::Some(Ui_sidebar_mode::DesignateRemoveConstruction),
            48 => ::std::option::Option::Some(Ui_sidebar_mode::DepotAccess),
            49 => ::std::option::Option::Some(Ui_sidebar_mode::NotesPoints),
            50 => ::std::option::Option::Some(Ui_sidebar_mode::NotesRoutes),
            51 => ::std::option::Option::Some(Ui_sidebar_mode::Burrows),
            52 => ::std::option::Option::Some(Ui_sidebar_mode::Hauling),
            53 => ::std::option::Option::Some(Ui_sidebar_mode::ArenaWeather),
            54 => ::std::option::Option::Some(Ui_sidebar_mode::ArenaTrees),
            _ => ::std::option::Option::None
        }
    }

    fn from_str(str: &str) -> ::std::option::Option<Ui_sidebar_mode> {
        match str {
            "Default" => ::std::option::Option::Some(Ui_sidebar_mode::Default),
            "Squads" => ::std::option::Option::Some(Ui_sidebar_mode::Squads),
            "DesignateMine" => ::std::option::Option::Some(Ui_sidebar_mode::DesignateMine),
            "DesignateRemoveRamps" => ::std::option::Option::Some(Ui_sidebar_mode::DesignateRemoveRamps),
            "DesignateUpStair" => ::std::option::Option::Some(Ui_sidebar_mode::DesignateUpStair),
            "DesignateDownStair" => ::std::option::Option::Some(Ui_sidebar_mode::DesignateDownStair),
            "DesignateUpDownStair" => ::std::option::Option::Some(Ui_sidebar_mode::DesignateUpDownStair),
            "DesignateUpRamp" => ::std::option::Option::Some(Ui_sidebar_mode::DesignateUpRamp),
            "DesignateChannel" => ::std::option::Option::Some(Ui_sidebar_mode::DesignateChannel),
            "DesignateGatherPlants" => ::std::option::Option::Some(Ui_sidebar_mode::DesignateGatherPlants),
            "DesignateRemoveDesignation" => ::std::option::Option::Some(Ui_sidebar_mode::DesignateRemoveDesignation),
            "DesignateSmooth" => ::std::option::Option::Some(Ui_sidebar_mode::DesignateSmooth),
            "DesignateCarveTrack" => ::std::option::Option::Some(Ui_sidebar_mode::DesignateCarveTrack),
            "DesignateEngrave" => ::std::option::Option::Some(Ui_sidebar_mode::DesignateEngrave),
            "DesignateCarveFortification" => ::std::option::Option::Some(Ui_sidebar_mode::DesignateCarveFortification),
            "Stockpiles" => ::std::option::Option::Some(Ui_sidebar_mode::Stockpiles),
            "Build" => ::std::option::Option::Some(Ui_sidebar_mode::Build),
            "QueryBuilding" => ::std::option::Option::Some(Ui_sidebar_mode::QueryBuilding),
            "Orders" => ::std::option::Option::Some(Ui_sidebar_mode::Orders),
            "OrdersForbid" => ::std::option::Option::Some(Ui_sidebar_mode::OrdersForbid),
            "OrdersRefuse" => ::std::option::Option::Some(Ui_sidebar_mode::OrdersRefuse),
            "OrdersWorkshop" => ::std::option::Option::Some(Ui_sidebar_mode::OrdersWorkshop),
            "OrdersZone" => ::std::option::Option::Some(Ui_sidebar_mode::OrdersZone),
            "BuildingItems" => ::std::option::Option::Some(Ui_sidebar_mode::BuildingItems),
            "ViewUnits" => ::std::option::Option::Some(Ui_sidebar_mode::ViewUnits),
            "LookAround" => ::std::option::Option::Some(Ui_sidebar_mode::LookAround),
            "DesignateItemsClaim" => ::std::option::Option::Some(Ui_sidebar_mode::DesignateItemsClaim),
            "DesignateItemsForbid" => ::std::option::Option::Some(Ui_sidebar_mode::DesignateItemsForbid),
            "DesignateItemsMelt" => ::std::option::Option::Some(Ui_sidebar_mode::DesignateItemsMelt),
            "DesignateItemsUnmelt" => ::std::option::Option::Some(Ui_sidebar_mode::DesignateItemsUnmelt),
            "DesignateItemsDump" => ::std::option::Option::Some(Ui_sidebar_mode::DesignateItemsDump),
            "DesignateItemsUndump" => ::std::option::Option::Some(Ui_sidebar_mode::DesignateItemsUndump),
            "DesignateItemsHide" => ::std::option::Option::Some(Ui_sidebar_mode::DesignateItemsHide),
            "DesignateItemsUnhide" => ::std::option::Option::Some(Ui_sidebar_mode::DesignateItemsUnhide),
            "DesignateChopTrees" => ::std::option::Option::Some(Ui_sidebar_mode::DesignateChopTrees),
            "DesignateToggleEngravings" => ::std::option::Option::Some(Ui_sidebar_mode::DesignateToggleEngravings),
            "DesignateToggleMarker" => ::std::option::Option::Some(Ui_sidebar_mode::DesignateToggleMarker),
            "Hotkeys" => ::std::option::Option::Some(Ui_sidebar_mode::Hotkeys),
            "DesignateTrafficHigh" => ::std::option::Option::Some(Ui_sidebar_mode::DesignateTrafficHigh),
            "DesignateTrafficNormal" => ::std::option::Option::Some(Ui_sidebar_mode::DesignateTrafficNormal),
            "DesignateTrafficLow" => ::std::option::Option::Some(Ui_sidebar_mode::DesignateTrafficLow),
            "DesignateTrafficRestricted" => ::std::option::Option::Some(Ui_sidebar_mode::DesignateTrafficRestricted),
            "Zones" => ::std::option::Option::Some(Ui_sidebar_mode::Zones),
            "ZonesPenInfo" => ::std::option::Option::Some(Ui_sidebar_mode::ZonesPenInfo),
            "ZonesPitInfo" => ::std::option::Option::Some(Ui_sidebar_mode::ZonesPitInfo),
            "ZonesHospitalInfo" => ::std::option::Option::Some(Ui_sidebar_mode::ZonesHospitalInfo),
            "ZonesGatherInfo" => ::std::option::Option::Some(Ui_sidebar_mode::ZonesGatherInfo),
            "DesignateRemoveConstruction" => ::std::option::Option::Some(Ui_sidebar_mode::DesignateRemoveConstruction),
            "DepotAccess" => ::std::option::Option::Some(Ui_sidebar_mode::DepotAccess),
            "NotesPoints" => ::std::option::Option::Some(Ui_sidebar_mode::NotesPoints),
            "NotesRoutes" => ::std::option::Option::Some(Ui_sidebar_mode::NotesRoutes),
            "Burrows" => ::std::option::Option::Some(Ui_sidebar_mode::Burrows),
            "Hauling" => ::std::option::Option::Some(Ui_sidebar_mode::Hauling),
            "ArenaWeather" => ::std::option::Option::Some(Ui_sidebar_mode::ArenaWeather),
            "ArenaTrees" => ::std::option::Option::Some(Ui_sidebar_mode::ArenaTrees),
            _ => ::std::option::Option::None
        }
    }

    const VALUES: &'static [Ui_sidebar_mode] = &[
        Ui_sidebar_mode::Default,
        Ui_sidebar_mode::Squads,
        Ui_sidebar_mode::DesignateMine,
        Ui_sidebar_mode::DesignateRemoveRamps,
        Ui_sidebar_mode::DesignateUpStair,
        Ui_sidebar_mode::DesignateDownStair,
        Ui_sidebar_mode::DesignateUpDownStair,
        Ui_sidebar_mode::DesignateUpRamp,
        Ui_sidebar_mode::DesignateChannel,
        Ui_sidebar_mode::DesignateGatherPlants,
        Ui_sidebar_mode::DesignateRemoveDesignation,
        Ui_sidebar_mode::DesignateSmooth,
        Ui_sidebar_mode::DesignateCarveTrack,
        Ui_sidebar_mode::DesignateEngrave,
        Ui_sidebar_mode::DesignateCarveFortification,
        Ui_sidebar_mode::Stockpiles,
        Ui_sidebar_mode::Build,
        Ui_sidebar_mode::QueryBuilding,
        Ui_sidebar_mode::Orders,
        Ui_sidebar_mode::OrdersForbid,
        Ui_sidebar_mode::OrdersRefuse,
        Ui_sidebar_mode::OrdersWorkshop,
        Ui_sidebar_mode::OrdersZone,
        Ui_sidebar_mode::BuildingItems,
        Ui_sidebar_mode::ViewUnits,
        Ui_sidebar_mode::LookAround,
        Ui_sidebar_mode::DesignateItemsClaim,
        Ui_sidebar_mode::DesignateItemsForbid,
        Ui_sidebar_mode::DesignateItemsMelt,
        Ui_sidebar_mode::DesignateItemsUnmelt,
        Ui_sidebar_mode::DesignateItemsDump,
        Ui_sidebar_mode::DesignateItemsUndump,
        Ui_sidebar_mode::DesignateItemsHide,
        Ui_sidebar_mode::DesignateItemsUnhide,
        Ui_sidebar_mode::DesignateChopTrees,
        Ui_sidebar_mode::DesignateToggleEngravings,
        Ui_sidebar_mode::DesignateToggleMarker,
        Ui_sidebar_mode::Hotkeys,
        Ui_sidebar_mode::DesignateTrafficHigh,
        Ui_sidebar_mode::DesignateTrafficNormal,
        Ui_sidebar_mode::DesignateTrafficLow,
        Ui_sidebar_mode::DesignateTrafficRestricted,
        Ui_sidebar_mode::Zones,
        Ui_sidebar_mode::ZonesPenInfo,
        Ui_sidebar_mode::ZonesPitInfo,
        Ui_sidebar_mode::ZonesHospitalInfo,
        Ui_sidebar_mode::ZonesGatherInfo,
        Ui_sidebar_mode::DesignateRemoveConstruction,
        Ui_sidebar_mode::DepotAccess,
        Ui_sidebar_mode::NotesPoints,
        Ui_sidebar_mode::NotesRoutes,
        Ui_sidebar_mode::Burrows,
        Ui_sidebar_mode::Hauling,
        Ui_sidebar_mode::ArenaWeather,
        Ui_sidebar_mode::ArenaTrees,
    ];
}

impl ::protobuf::EnumFull for Ui_sidebar_mode {
    fn enum_descriptor() -> ::protobuf::reflect::EnumDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().enum_by_package_relative_name("ui_sidebar_mode").unwrap()).clone()
    }

    fn descriptor(&self) -> ::protobuf::reflect::EnumValueDescriptor {
        let index = *self as usize;
        Self::enum_descriptor().value_by_index(index)
    }
}

impl ::std::default::Default for Ui_sidebar_mode {
    fn default() -> Self {
        Ui_sidebar_mode::Default
    }
}

impl Ui_sidebar_mode {
    fn generated_enum_descriptor_data() -> ::protobuf::reflect::GeneratedEnumDescriptorData {
        ::protobuf::reflect::GeneratedEnumDescriptorData::new::<Ui_sidebar_mode>("ui_sidebar_mode")
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x15ui_sidebar_mode.proto\x12\x1bproto.enums.ui_sidebar_mode*\xa0\t\n\
    \x0fui_sidebar_mode\x12\x0b\n\x07Default\x10\0\x12\n\n\x06Squads\x10\x01\
    \x12\x11\n\rDesignateMine\x10\x02\x12\x18\n\x14DesignateRemoveRamps\x10\
    \x03\x12\x14\n\x10DesignateUpStair\x10\x04\x12\x16\n\x12DesignateDownSta\
    ir\x10\x05\x12\x18\n\x14DesignateUpDownStair\x10\x06\x12\x13\n\x0fDesign\
    ateUpRamp\x10\x07\x12\x14\n\x10DesignateChannel\x10\x08\x12\x19\n\x15Des\
    ignateGatherPlants\x10\t\x12\x1e\n\x1aDesignateRemoveDesignation\x10\n\
    \x12\x13\n\x0fDesignateSmooth\x10\x0b\x12\x17\n\x13DesignateCarveTrack\
    \x10\x0c\x12\x14\n\x10DesignateEngrave\x10\r\x12\x1f\n\x1bDesignateCarve\
    Fortification\x10\x0e\x12\x0e\n\nStockpiles\x10\x0f\x12\t\n\x05Build\x10\
    \x10\x12\x11\n\rQueryBuilding\x10\x11\x12\n\n\x06Orders\x10\x12\x12\x10\
    \n\x0cOrdersForbid\x10\x13\x12\x10\n\x0cOrdersRefuse\x10\x14\x12\x12\n\
    \x0eOrdersWorkshop\x10\x15\x12\x0e\n\nOrdersZone\x10\x16\x12\x11\n\rBuil\
    dingItems\x10\x17\x12\r\n\tViewUnits\x10\x18\x12\x0e\n\nLookAround\x10\
    \x19\x12\x17\n\x13DesignateItemsClaim\x10\x1a\x12\x18\n\x14DesignateItem\
    sForbid\x10\x1b\x12\x16\n\x12DesignateItemsMelt\x10\x1c\x12\x18\n\x14Des\
    ignateItemsUnmelt\x10\x1d\x12\x16\n\x12DesignateItemsDump\x10\x1e\x12\
    \x18\n\x14DesignateItemsUndump\x10\x1f\x12\x16\n\x12DesignateItemsHide\
    \x10\x20\x12\x18\n\x14DesignateItemsUnhide\x10!\x12\x16\n\x12DesignateCh\
    opTrees\x10\"\x12\x1d\n\x19DesignateToggleEngravings\x10#\x12\x19\n\x15D\
    esignateToggleMarker\x10$\x12\x0b\n\x07Hotkeys\x10%\x12\x18\n\x14Designa\
    teTrafficHigh\x10&\x12\x1a\n\x16DesignateTrafficNormal\x10'\x12\x17\n\
    \x13DesignateTrafficLow\x10(\x12\x1e\n\x1aDesignateTrafficRestricted\x10\
    )\x12\t\n\x05Zones\x10*\x12\x10\n\x0cZonesPenInfo\x10+\x12\x10\n\x0cZone\
    sPitInfo\x10,\x12\x15\n\x11ZonesHospitalInfo\x10-\x12\x13\n\x0fZonesGath\
    erInfo\x10.\x12\x1f\n\x1bDesignateRemoveConstruction\x10/\x12\x0f\n\x0bD\
    epotAccess\x100\x12\x0f\n\x0bNotesPoints\x101\x12\x0f\n\x0bNotesRoutes\
    \x102\x12\x0b\n\x07Burrows\x103\x12\x0b\n\x07Hauling\x104\x12\x10\n\x0cA\
    renaWeather\x105\x12\x0e\n\nArenaTrees\x106B\x02H\x03b\x06proto2\
";

/// `FileDescriptorProto` object which was a source for this generated file
fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    static file_descriptor_proto_lazy: ::protobuf::rt::Lazy<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::rt::Lazy::new();
    file_descriptor_proto_lazy.get(|| {
        ::protobuf::Message::parse_from_bytes(file_descriptor_proto_data).unwrap()
    })
}

/// `FileDescriptor` object which allows dynamic access to files
pub fn file_descriptor() -> &'static ::protobuf::reflect::FileDescriptor {
    static generated_file_descriptor_lazy: ::protobuf::rt::Lazy<::protobuf::reflect::GeneratedFileDescriptor> = ::protobuf::rt::Lazy::new();
    static file_descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::FileDescriptor> = ::protobuf::rt::Lazy::new();
    file_descriptor.get(|| {
        let generated_file_descriptor = generated_file_descriptor_lazy.get(|| {
            let mut deps = ::std::vec::Vec::with_capacity(0);
            let mut messages = ::std::vec::Vec::with_capacity(0);
            let mut enums = ::std::vec::Vec::with_capacity(1);
            enums.push(Ui_sidebar_mode::generated_enum_descriptor_data());
            ::protobuf::reflect::GeneratedFileDescriptor::new_generated(
                file_descriptor_proto(),
                deps,
                messages,
                enums,
            )
        });
        ::protobuf::reflect::FileDescriptor::new_generated_2(generated_file_descriptor)
    })
}
