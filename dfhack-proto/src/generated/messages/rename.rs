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

//! Generated file from `rename.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_2;

// @@protoc_insertion_point(message:dfproto.RenameSquadIn)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct RenameSquadIn {
    // message fields
    // @@protoc_insertion_point(field:dfproto.RenameSquadIn.squad_id)
    pub squad_id: ::std::option::Option<i32>,
    // @@protoc_insertion_point(field:dfproto.RenameSquadIn.nickname)
    pub nickname: ::std::option::Option<::std::string::String>,
    // @@protoc_insertion_point(field:dfproto.RenameSquadIn.alias)
    pub alias: ::std::option::Option<::std::string::String>,
    // special fields
    // @@protoc_insertion_point(special_field:dfproto.RenameSquadIn.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a RenameSquadIn {
    fn default() -> &'a RenameSquadIn {
        <RenameSquadIn as ::protobuf::Message>::default_instance()
    }
}

impl RenameSquadIn {
    pub fn new() -> RenameSquadIn {
        ::std::default::Default::default()
    }

    // required int32 squad_id = 1;

    pub fn squad_id(&self) -> i32 {
        self.squad_id.unwrap_or(0)
    }

    pub fn clear_squad_id(&mut self) {
        self.squad_id = ::std::option::Option::None;
    }

    pub fn has_squad_id(&self) -> bool {
        self.squad_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_squad_id(&mut self, v: i32) {
        self.squad_id = ::std::option::Option::Some(v);
    }

    // optional string nickname = 2;

    pub fn nickname(&self) -> &str {
        match self.nickname.as_ref() {
            Some(v) => v,
            None => "",
        }
    }

    pub fn clear_nickname(&mut self) {
        self.nickname = ::std::option::Option::None;
    }

    pub fn has_nickname(&self) -> bool {
        self.nickname.is_some()
    }

    // Param is passed by value, moved
    pub fn set_nickname(&mut self, v: ::std::string::String) {
        self.nickname = ::std::option::Option::Some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_nickname(&mut self) -> &mut ::std::string::String {
        if self.nickname.is_none() {
            self.nickname = ::std::option::Option::Some(::std::string::String::new());
        }
        self.nickname.as_mut().unwrap()
    }

    // Take field
    pub fn take_nickname(&mut self) -> ::std::string::String {
        self.nickname.take().unwrap_or_else(|| ::std::string::String::new())
    }

    // optional string alias = 3;

    pub fn alias(&self) -> &str {
        match self.alias.as_ref() {
            Some(v) => v,
            None => "",
        }
    }

    pub fn clear_alias(&mut self) {
        self.alias = ::std::option::Option::None;
    }

    pub fn has_alias(&self) -> bool {
        self.alias.is_some()
    }

    // Param is passed by value, moved
    pub fn set_alias(&mut self, v: ::std::string::String) {
        self.alias = ::std::option::Option::Some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_alias(&mut self) -> &mut ::std::string::String {
        if self.alias.is_none() {
            self.alias = ::std::option::Option::Some(::std::string::String::new());
        }
        self.alias.as_mut().unwrap()
    }

    // Take field
    pub fn take_alias(&mut self) -> ::std::string::String {
        self.alias.take().unwrap_or_else(|| ::std::string::String::new())
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(3);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_option_accessor::<_, _>(
            "squad_id",
            |m: &RenameSquadIn| { &m.squad_id },
            |m: &mut RenameSquadIn| { &mut m.squad_id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_option_accessor::<_, _>(
            "nickname",
            |m: &RenameSquadIn| { &m.nickname },
            |m: &mut RenameSquadIn| { &mut m.nickname },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_option_accessor::<_, _>(
            "alias",
            |m: &RenameSquadIn| { &m.alias },
            |m: &mut RenameSquadIn| { &mut m.alias },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<RenameSquadIn>(
            "RenameSquadIn",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for RenameSquadIn {
    const NAME: &'static str = "RenameSquadIn";

    fn is_initialized(&self) -> bool {
        if self.squad_id.is_none() {
            return false;
        }
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                8 => {
                    self.squad_id = ::std::option::Option::Some(is.read_int32()?);
                },
                18 => {
                    self.nickname = ::std::option::Option::Some(is.read_string()?);
                },
                26 => {
                    self.alias = ::std::option::Option::Some(is.read_string()?);
                },
                tag => {
                    ::protobuf::rt::read_unknown_or_skip_group(tag, is, self.special_fields.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u64 {
        let mut my_size = 0;
        if let Some(v) = self.squad_id {
            my_size += ::protobuf::rt::int32_size(1, v);
        }
        if let Some(v) = self.nickname.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        }
        if let Some(v) = self.alias.as_ref() {
            my_size += ::protobuf::rt::string_size(3, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if let Some(v) = self.squad_id {
            os.write_int32(1, v)?;
        }
        if let Some(v) = self.nickname.as_ref() {
            os.write_string(2, v)?;
        }
        if let Some(v) = self.alias.as_ref() {
            os.write_string(3, v)?;
        }
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &::protobuf::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::protobuf::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> RenameSquadIn {
        RenameSquadIn::new()
    }

    fn clear(&mut self) {
        self.squad_id = ::std::option::Option::None;
        self.nickname = ::std::option::Option::None;
        self.alias = ::std::option::Option::None;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static RenameSquadIn {
        static instance: RenameSquadIn = RenameSquadIn {
            squad_id: ::std::option::Option::None,
            nickname: ::std::option::Option::None,
            alias: ::std::option::Option::None,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for RenameSquadIn {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("RenameSquadIn").unwrap()).clone()
    }
}

impl ::std::fmt::Display for RenameSquadIn {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RenameSquadIn {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

// @@protoc_insertion_point(message:dfproto.RenameUnitIn)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct RenameUnitIn {
    // message fields
    // @@protoc_insertion_point(field:dfproto.RenameUnitIn.unit_id)
    pub unit_id: ::std::option::Option<i32>,
    // @@protoc_insertion_point(field:dfproto.RenameUnitIn.nickname)
    pub nickname: ::std::option::Option<::std::string::String>,
    // @@protoc_insertion_point(field:dfproto.RenameUnitIn.profession)
    pub profession: ::std::option::Option<::std::string::String>,
    // special fields
    // @@protoc_insertion_point(special_field:dfproto.RenameUnitIn.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a RenameUnitIn {
    fn default() -> &'a RenameUnitIn {
        <RenameUnitIn as ::protobuf::Message>::default_instance()
    }
}

impl RenameUnitIn {
    pub fn new() -> RenameUnitIn {
        ::std::default::Default::default()
    }

    // required int32 unit_id = 1;

    pub fn unit_id(&self) -> i32 {
        self.unit_id.unwrap_or(0)
    }

    pub fn clear_unit_id(&mut self) {
        self.unit_id = ::std::option::Option::None;
    }

    pub fn has_unit_id(&self) -> bool {
        self.unit_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_unit_id(&mut self, v: i32) {
        self.unit_id = ::std::option::Option::Some(v);
    }

    // optional string nickname = 2;

    pub fn nickname(&self) -> &str {
        match self.nickname.as_ref() {
            Some(v) => v,
            None => "",
        }
    }

    pub fn clear_nickname(&mut self) {
        self.nickname = ::std::option::Option::None;
    }

    pub fn has_nickname(&self) -> bool {
        self.nickname.is_some()
    }

    // Param is passed by value, moved
    pub fn set_nickname(&mut self, v: ::std::string::String) {
        self.nickname = ::std::option::Option::Some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_nickname(&mut self) -> &mut ::std::string::String {
        if self.nickname.is_none() {
            self.nickname = ::std::option::Option::Some(::std::string::String::new());
        }
        self.nickname.as_mut().unwrap()
    }

    // Take field
    pub fn take_nickname(&mut self) -> ::std::string::String {
        self.nickname.take().unwrap_or_else(|| ::std::string::String::new())
    }

    // optional string profession = 3;

    pub fn profession(&self) -> &str {
        match self.profession.as_ref() {
            Some(v) => v,
            None => "",
        }
    }

    pub fn clear_profession(&mut self) {
        self.profession = ::std::option::Option::None;
    }

    pub fn has_profession(&self) -> bool {
        self.profession.is_some()
    }

    // Param is passed by value, moved
    pub fn set_profession(&mut self, v: ::std::string::String) {
        self.profession = ::std::option::Option::Some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_profession(&mut self) -> &mut ::std::string::String {
        if self.profession.is_none() {
            self.profession = ::std::option::Option::Some(::std::string::String::new());
        }
        self.profession.as_mut().unwrap()
    }

    // Take field
    pub fn take_profession(&mut self) -> ::std::string::String {
        self.profession.take().unwrap_or_else(|| ::std::string::String::new())
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(3);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_option_accessor::<_, _>(
            "unit_id",
            |m: &RenameUnitIn| { &m.unit_id },
            |m: &mut RenameUnitIn| { &mut m.unit_id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_option_accessor::<_, _>(
            "nickname",
            |m: &RenameUnitIn| { &m.nickname },
            |m: &mut RenameUnitIn| { &mut m.nickname },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_option_accessor::<_, _>(
            "profession",
            |m: &RenameUnitIn| { &m.profession },
            |m: &mut RenameUnitIn| { &mut m.profession },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<RenameUnitIn>(
            "RenameUnitIn",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for RenameUnitIn {
    const NAME: &'static str = "RenameUnitIn";

    fn is_initialized(&self) -> bool {
        if self.unit_id.is_none() {
            return false;
        }
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                8 => {
                    self.unit_id = ::std::option::Option::Some(is.read_int32()?);
                },
                18 => {
                    self.nickname = ::std::option::Option::Some(is.read_string()?);
                },
                26 => {
                    self.profession = ::std::option::Option::Some(is.read_string()?);
                },
                tag => {
                    ::protobuf::rt::read_unknown_or_skip_group(tag, is, self.special_fields.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u64 {
        let mut my_size = 0;
        if let Some(v) = self.unit_id {
            my_size += ::protobuf::rt::int32_size(1, v);
        }
        if let Some(v) = self.nickname.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        }
        if let Some(v) = self.profession.as_ref() {
            my_size += ::protobuf::rt::string_size(3, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if let Some(v) = self.unit_id {
            os.write_int32(1, v)?;
        }
        if let Some(v) = self.nickname.as_ref() {
            os.write_string(2, v)?;
        }
        if let Some(v) = self.profession.as_ref() {
            os.write_string(3, v)?;
        }
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &::protobuf::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::protobuf::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> RenameUnitIn {
        RenameUnitIn::new()
    }

    fn clear(&mut self) {
        self.unit_id = ::std::option::Option::None;
        self.nickname = ::std::option::Option::None;
        self.profession = ::std::option::Option::None;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static RenameUnitIn {
        static instance: RenameUnitIn = RenameUnitIn {
            unit_id: ::std::option::Option::None,
            nickname: ::std::option::Option::None,
            profession: ::std::option::Option::None,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for RenameUnitIn {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("RenameUnitIn").unwrap()).clone()
    }
}

impl ::std::fmt::Display for RenameUnitIn {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RenameUnitIn {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

// @@protoc_insertion_point(message:dfproto.RenameBuildingIn)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct RenameBuildingIn {
    // message fields
    // @@protoc_insertion_point(field:dfproto.RenameBuildingIn.building_id)
    pub building_id: ::std::option::Option<i32>,
    // @@protoc_insertion_point(field:dfproto.RenameBuildingIn.name)
    pub name: ::std::option::Option<::std::string::String>,
    // special fields
    // @@protoc_insertion_point(special_field:dfproto.RenameBuildingIn.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a RenameBuildingIn {
    fn default() -> &'a RenameBuildingIn {
        <RenameBuildingIn as ::protobuf::Message>::default_instance()
    }
}

impl RenameBuildingIn {
    pub fn new() -> RenameBuildingIn {
        ::std::default::Default::default()
    }

    // required int32 building_id = 1;

    pub fn building_id(&self) -> i32 {
        self.building_id.unwrap_or(0)
    }

    pub fn clear_building_id(&mut self) {
        self.building_id = ::std::option::Option::None;
    }

    pub fn has_building_id(&self) -> bool {
        self.building_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_building_id(&mut self, v: i32) {
        self.building_id = ::std::option::Option::Some(v);
    }

    // optional string name = 2;

    pub fn name(&self) -> &str {
        match self.name.as_ref() {
            Some(v) => v,
            None => "",
        }
    }

    pub fn clear_name(&mut self) {
        self.name = ::std::option::Option::None;
    }

    pub fn has_name(&self) -> bool {
        self.name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_name(&mut self, v: ::std::string::String) {
        self.name = ::std::option::Option::Some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_name(&mut self) -> &mut ::std::string::String {
        if self.name.is_none() {
            self.name = ::std::option::Option::Some(::std::string::String::new());
        }
        self.name.as_mut().unwrap()
    }

    // Take field
    pub fn take_name(&mut self) -> ::std::string::String {
        self.name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(2);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_option_accessor::<_, _>(
            "building_id",
            |m: &RenameBuildingIn| { &m.building_id },
            |m: &mut RenameBuildingIn| { &mut m.building_id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_option_accessor::<_, _>(
            "name",
            |m: &RenameBuildingIn| { &m.name },
            |m: &mut RenameBuildingIn| { &mut m.name },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<RenameBuildingIn>(
            "RenameBuildingIn",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for RenameBuildingIn {
    const NAME: &'static str = "RenameBuildingIn";

    fn is_initialized(&self) -> bool {
        if self.building_id.is_none() {
            return false;
        }
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                8 => {
                    self.building_id = ::std::option::Option::Some(is.read_int32()?);
                },
                18 => {
                    self.name = ::std::option::Option::Some(is.read_string()?);
                },
                tag => {
                    ::protobuf::rt::read_unknown_or_skip_group(tag, is, self.special_fields.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u64 {
        let mut my_size = 0;
        if let Some(v) = self.building_id {
            my_size += ::protobuf::rt::int32_size(1, v);
        }
        if let Some(v) = self.name.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if let Some(v) = self.building_id {
            os.write_int32(1, v)?;
        }
        if let Some(v) = self.name.as_ref() {
            os.write_string(2, v)?;
        }
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &::protobuf::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::protobuf::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> RenameBuildingIn {
        RenameBuildingIn::new()
    }

    fn clear(&mut self) {
        self.building_id = ::std::option::Option::None;
        self.name = ::std::option::Option::None;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static RenameBuildingIn {
        static instance: RenameBuildingIn = RenameBuildingIn {
            building_id: ::std::option::Option::None,
            name: ::std::option::Option::None,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for RenameBuildingIn {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("RenameBuildingIn").unwrap()).clone()
    }
}

impl ::std::fmt::Display for RenameBuildingIn {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RenameBuildingIn {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x0crename.proto\x12\x07dfproto\"\\\n\rRenameSquadIn\x12\x19\n\x08squa\
    d_id\x18\x01\x20\x02(\x05R\x07squadId\x12\x1a\n\x08nickname\x18\x02\x20\
    \x01(\tR\x08nickname\x12\x14\n\x05alias\x18\x03\x20\x01(\tR\x05alias\"c\
    \n\x0cRenameUnitIn\x12\x17\n\x07unit_id\x18\x01\x20\x02(\x05R\x06unitId\
    \x12\x1a\n\x08nickname\x18\x02\x20\x01(\tR\x08nickname\x12\x1e\n\nprofes\
    sion\x18\x03\x20\x01(\tR\nprofession\"G\n\x10RenameBuildingIn\x12\x1f\n\
    \x0bbuilding_id\x18\x01\x20\x02(\x05R\nbuildingId\x12\x12\n\x04name\x18\
    \x02\x20\x01(\tR\x04nameB\x02H\x03b\x06proto2\
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
            let mut messages = ::std::vec::Vec::with_capacity(3);
            messages.push(RenameSquadIn::generated_message_descriptor_data());
            messages.push(RenameUnitIn::generated_message_descriptor_data());
            messages.push(RenameBuildingIn::generated_message_descriptor_data());
            let mut enums = ::std::vec::Vec::with_capacity(0);
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
