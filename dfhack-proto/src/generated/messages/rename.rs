// This file is generated by rust-protobuf 2.27.1. Do not edit
// @generated

// https://github.com/rust-lang/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![allow(unused_attributes)]
#![cfg_attr(rustfmt, rustfmt::skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unused_imports)]
#![allow(unused_results)]
//! Generated file from `rename.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
// const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_27_1;

#[derive(PartialEq,Clone,Default)]
pub struct RenameSquadIn {
    // message fields
    squad_id: ::std::option::Option<i32>,
    nickname: ::protobuf::SingularField<::std::string::String>,
    alias: ::protobuf::SingularField<::std::string::String>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
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


    pub fn get_squad_id(&self) -> i32 {
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


    pub fn get_nickname(&self) -> &str {
        match self.nickname.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }
    pub fn clear_nickname(&mut self) {
        self.nickname.clear();
    }

    pub fn has_nickname(&self) -> bool {
        self.nickname.is_some()
    }

    // Param is passed by value, moved
    pub fn set_nickname(&mut self, v: ::std::string::String) {
        self.nickname = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_nickname(&mut self) -> &mut ::std::string::String {
        if self.nickname.is_none() {
            self.nickname.set_default();
        }
        self.nickname.as_mut().unwrap()
    }

    // Take field
    pub fn take_nickname(&mut self) -> ::std::string::String {
        self.nickname.take().unwrap_or_else(|| ::std::string::String::new())
    }

    // optional string alias = 3;


    pub fn get_alias(&self) -> &str {
        match self.alias.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }
    pub fn clear_alias(&mut self) {
        self.alias.clear();
    }

    pub fn has_alias(&self) -> bool {
        self.alias.is_some()
    }

    // Param is passed by value, moved
    pub fn set_alias(&mut self, v: ::std::string::String) {
        self.alias = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_alias(&mut self) -> &mut ::std::string::String {
        if self.alias.is_none() {
            self.alias.set_default();
        }
        self.alias.as_mut().unwrap()
    }

    // Take field
    pub fn take_alias(&mut self) -> ::std::string::String {
        self.alias.take().unwrap_or_else(|| ::std::string::String::new())
    }
}

impl ::protobuf::Message for RenameSquadIn {
    fn is_initialized(&self) -> bool {
        if self.squad_id.is_none() {
            return false;
        }
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.squad_id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.nickname)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.alias)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.squad_id {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.nickname.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        }
        if let Some(ref v) = self.alias.as_ref() {
            my_size += ::protobuf::rt::string_size(3, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.squad_id {
            os.write_int32(1, v)?;
        }
        if let Some(ref v) = self.nickname.as_ref() {
            os.write_string(2, &v)?;
        }
        if let Some(ref v) = self.alias.as_ref() {
            os.write_string(3, &v)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: ::std::boxed::Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> RenameSquadIn {
        RenameSquadIn::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                "squad_id",
                |m: &RenameSquadIn| { &m.squad_id },
                |m: &mut RenameSquadIn| { &mut m.squad_id },
            ));
            fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "nickname",
                |m: &RenameSquadIn| { &m.nickname },
                |m: &mut RenameSquadIn| { &mut m.nickname },
            ));
            fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "alias",
                |m: &RenameSquadIn| { &m.alias },
                |m: &mut RenameSquadIn| { &mut m.alias },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<RenameSquadIn>(
                "RenameSquadIn",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static RenameSquadIn {
        static instance: ::protobuf::rt::LazyV2<RenameSquadIn> = ::protobuf::rt::LazyV2::INIT;
        instance.get(RenameSquadIn::new)
    }
}

impl ::protobuf::Clear for RenameSquadIn {
    fn clear(&mut self) {
        self.squad_id = ::std::option::Option::None;
        self.nickname.clear();
        self.alias.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for RenameSquadIn {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RenameSquadIn {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct RenameUnitIn {
    // message fields
    unit_id: ::std::option::Option<i32>,
    nickname: ::protobuf::SingularField<::std::string::String>,
    profession: ::protobuf::SingularField<::std::string::String>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
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


    pub fn get_unit_id(&self) -> i32 {
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


    pub fn get_nickname(&self) -> &str {
        match self.nickname.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }
    pub fn clear_nickname(&mut self) {
        self.nickname.clear();
    }

    pub fn has_nickname(&self) -> bool {
        self.nickname.is_some()
    }

    // Param is passed by value, moved
    pub fn set_nickname(&mut self, v: ::std::string::String) {
        self.nickname = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_nickname(&mut self) -> &mut ::std::string::String {
        if self.nickname.is_none() {
            self.nickname.set_default();
        }
        self.nickname.as_mut().unwrap()
    }

    // Take field
    pub fn take_nickname(&mut self) -> ::std::string::String {
        self.nickname.take().unwrap_or_else(|| ::std::string::String::new())
    }

    // optional string profession = 3;


    pub fn get_profession(&self) -> &str {
        match self.profession.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }
    pub fn clear_profession(&mut self) {
        self.profession.clear();
    }

    pub fn has_profession(&self) -> bool {
        self.profession.is_some()
    }

    // Param is passed by value, moved
    pub fn set_profession(&mut self, v: ::std::string::String) {
        self.profession = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_profession(&mut self) -> &mut ::std::string::String {
        if self.profession.is_none() {
            self.profession.set_default();
        }
        self.profession.as_mut().unwrap()
    }

    // Take field
    pub fn take_profession(&mut self) -> ::std::string::String {
        self.profession.take().unwrap_or_else(|| ::std::string::String::new())
    }
}

impl ::protobuf::Message for RenameUnitIn {
    fn is_initialized(&self) -> bool {
        if self.unit_id.is_none() {
            return false;
        }
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.unit_id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.nickname)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.profession)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.unit_id {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.nickname.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        }
        if let Some(ref v) = self.profession.as_ref() {
            my_size += ::protobuf::rt::string_size(3, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.unit_id {
            os.write_int32(1, v)?;
        }
        if let Some(ref v) = self.nickname.as_ref() {
            os.write_string(2, &v)?;
        }
        if let Some(ref v) = self.profession.as_ref() {
            os.write_string(3, &v)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: ::std::boxed::Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> RenameUnitIn {
        RenameUnitIn::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                "unit_id",
                |m: &RenameUnitIn| { &m.unit_id },
                |m: &mut RenameUnitIn| { &mut m.unit_id },
            ));
            fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "nickname",
                |m: &RenameUnitIn| { &m.nickname },
                |m: &mut RenameUnitIn| { &mut m.nickname },
            ));
            fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "profession",
                |m: &RenameUnitIn| { &m.profession },
                |m: &mut RenameUnitIn| { &mut m.profession },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<RenameUnitIn>(
                "RenameUnitIn",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static RenameUnitIn {
        static instance: ::protobuf::rt::LazyV2<RenameUnitIn> = ::protobuf::rt::LazyV2::INIT;
        instance.get(RenameUnitIn::new)
    }
}

impl ::protobuf::Clear for RenameUnitIn {
    fn clear(&mut self) {
        self.unit_id = ::std::option::Option::None;
        self.nickname.clear();
        self.profession.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for RenameUnitIn {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RenameUnitIn {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct RenameBuildingIn {
    // message fields
    building_id: ::std::option::Option<i32>,
    name: ::protobuf::SingularField<::std::string::String>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
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


    pub fn get_building_id(&self) -> i32 {
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


    pub fn get_name(&self) -> &str {
        match self.name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }
    pub fn clear_name(&mut self) {
        self.name.clear();
    }

    pub fn has_name(&self) -> bool {
        self.name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_name(&mut self, v: ::std::string::String) {
        self.name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_name(&mut self) -> &mut ::std::string::String {
        if self.name.is_none() {
            self.name.set_default();
        }
        self.name.as_mut().unwrap()
    }

    // Take field
    pub fn take_name(&mut self) -> ::std::string::String {
        self.name.take().unwrap_or_else(|| ::std::string::String::new())
    }
}

impl ::protobuf::Message for RenameBuildingIn {
    fn is_initialized(&self) -> bool {
        if self.building_id.is_none() {
            return false;
        }
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.building_id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.name)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.building_id {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.name.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.building_id {
            os.write_int32(1, v)?;
        }
        if let Some(ref v) = self.name.as_ref() {
            os.write_string(2, &v)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: ::std::boxed::Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> RenameBuildingIn {
        RenameBuildingIn::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                "building_id",
                |m: &RenameBuildingIn| { &m.building_id },
                |m: &mut RenameBuildingIn| { &mut m.building_id },
            ));
            fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "name",
                |m: &RenameBuildingIn| { &m.name },
                |m: &mut RenameBuildingIn| { &mut m.name },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<RenameBuildingIn>(
                "RenameBuildingIn",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static RenameBuildingIn {
        static instance: ::protobuf::rt::LazyV2<RenameBuildingIn> = ::protobuf::rt::LazyV2::INIT;
        instance.get(RenameBuildingIn::new)
    }
}

impl ::protobuf::Clear for RenameBuildingIn {
    fn clear(&mut self) {
        self.building_id = ::std::option::Option::None;
        self.name.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for RenameBuildingIn {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RenameBuildingIn {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x0crename.proto\x12\x07dfproto\"d\n\rRenameSquadIn\x12\x1b\n\x08squad\
    _id\x18\x01\x20\x02(\x05R\x07squadIdB\0\x12\x1c\n\x08nickname\x18\x02\
    \x20\x01(\tR\x08nicknameB\0\x12\x16\n\x05alias\x18\x03\x20\x01(\tR\x05al\
    iasB\0:\0\"k\n\x0cRenameUnitIn\x12\x19\n\x07unit_id\x18\x01\x20\x02(\x05\
    R\x06unitIdB\0\x12\x1c\n\x08nickname\x18\x02\x20\x01(\tR\x08nicknameB\0\
    \x12\x20\n\nprofession\x18\x03\x20\x01(\tR\nprofessionB\0:\0\"M\n\x10Ren\
    ameBuildingIn\x12!\n\x0bbuilding_id\x18\x01\x20\x02(\x05R\nbuildingIdB\0\
    \x12\x14\n\x04name\x18\x02\x20\x01(\tR\x04nameB\0:\0B\0b\x06proto2\
";

static file_descriptor_proto_lazy: ::protobuf::rt::LazyV2<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::rt::LazyV2::INIT;

fn parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto {
    ::protobuf::Message::parse_from_bytes(file_descriptor_proto_data).unwrap()
}

pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    file_descriptor_proto_lazy.get(|| {
        parse_descriptor_proto()
    })
}