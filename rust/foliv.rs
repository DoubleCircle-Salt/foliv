// This file is generated by rust-protobuf 2.24.1. Do not edit
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
//! Generated file from `foliv.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
// const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_24_1;

#[derive(PartialEq,Clone,Default)]
pub struct Foliv {
    // message fields
    pub userHash: ::std::string::String,
    pub command: Command,
    pub addressType: AddressType,
    pub address: ::std::vec::Vec<u8>,
    pub port: u32,
    pub sourceName: ::std::string::String,
    pub routerName: ::std::string::String,
    pub processName: ::std::string::String,
    pub xForwardedFor: ::protobuf::RepeatedField<::std::vec::Vec<u8>>,
    pub isTouch: bool,
    pub muxID: u32,
    pub platform: ::std::string::String,
    pub requestID: ::std::vec::Vec<u8>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a Foliv {
    fn default() -> &'a Foliv {
        <Foliv as ::protobuf::Message>::default_instance()
    }
}

impl Foliv {
    pub fn new() -> Foliv {
        ::std::default::Default::default()
    }

    // string userHash = 1;


    pub fn get_userHash(&self) -> &str {
        &self.userHash
    }
    pub fn clear_userHash(&mut self) {
        self.userHash.clear();
    }

    // Param is passed by value, moved
    pub fn set_userHash(&mut self, v: ::std::string::String) {
        self.userHash = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_userHash(&mut self) -> &mut ::std::string::String {
        &mut self.userHash
    }

    // Take field
    pub fn take_userHash(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.userHash, ::std::string::String::new())
    }

    // .foliv.Command command = 2;


    pub fn get_command(&self) -> Command {
        self.command
    }
    pub fn clear_command(&mut self) {
        self.command = Command::Empty;
    }

    // Param is passed by value, moved
    pub fn set_command(&mut self, v: Command) {
        self.command = v;
    }

    // .foliv.AddressType addressType = 3;


    pub fn get_addressType(&self) -> AddressType {
        self.addressType
    }
    pub fn clear_addressType(&mut self) {
        self.addressType = AddressType::InvalidType;
    }

    // Param is passed by value, moved
    pub fn set_addressType(&mut self, v: AddressType) {
        self.addressType = v;
    }

    // bytes address = 4;


    pub fn get_address(&self) -> &[u8] {
        &self.address
    }
    pub fn clear_address(&mut self) {
        self.address.clear();
    }

    // Param is passed by value, moved
    pub fn set_address(&mut self, v: ::std::vec::Vec<u8>) {
        self.address = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_address(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.address
    }

    // Take field
    pub fn take_address(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.address, ::std::vec::Vec::new())
    }

    // uint32 port = 5;


    pub fn get_port(&self) -> u32 {
        self.port
    }
    pub fn clear_port(&mut self) {
        self.port = 0;
    }

    // Param is passed by value, moved
    pub fn set_port(&mut self, v: u32) {
        self.port = v;
    }

    // string sourceName = 6;


    pub fn get_sourceName(&self) -> &str {
        &self.sourceName
    }
    pub fn clear_sourceName(&mut self) {
        self.sourceName.clear();
    }

    // Param is passed by value, moved
    pub fn set_sourceName(&mut self, v: ::std::string::String) {
        self.sourceName = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_sourceName(&mut self) -> &mut ::std::string::String {
        &mut self.sourceName
    }

    // Take field
    pub fn take_sourceName(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.sourceName, ::std::string::String::new())
    }

    // string routerName = 7;


    pub fn get_routerName(&self) -> &str {
        &self.routerName
    }
    pub fn clear_routerName(&mut self) {
        self.routerName.clear();
    }

    // Param is passed by value, moved
    pub fn set_routerName(&mut self, v: ::std::string::String) {
        self.routerName = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_routerName(&mut self) -> &mut ::std::string::String {
        &mut self.routerName
    }

    // Take field
    pub fn take_routerName(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.routerName, ::std::string::String::new())
    }

    // string processName = 8;


    pub fn get_processName(&self) -> &str {
        &self.processName
    }
    pub fn clear_processName(&mut self) {
        self.processName.clear();
    }

    // Param is passed by value, moved
    pub fn set_processName(&mut self, v: ::std::string::String) {
        self.processName = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_processName(&mut self) -> &mut ::std::string::String {
        &mut self.processName
    }

    // Take field
    pub fn take_processName(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.processName, ::std::string::String::new())
    }

    // repeated bytes xForwardedFor = 9;


    pub fn get_xForwardedFor(&self) -> &[::std::vec::Vec<u8>] {
        &self.xForwardedFor
    }
    pub fn clear_xForwardedFor(&mut self) {
        self.xForwardedFor.clear();
    }

    // Param is passed by value, moved
    pub fn set_xForwardedFor(&mut self, v: ::protobuf::RepeatedField<::std::vec::Vec<u8>>) {
        self.xForwardedFor = v;
    }

    // Mutable pointer to the field.
    pub fn mut_xForwardedFor(&mut self) -> &mut ::protobuf::RepeatedField<::std::vec::Vec<u8>> {
        &mut self.xForwardedFor
    }

    // Take field
    pub fn take_xForwardedFor(&mut self) -> ::protobuf::RepeatedField<::std::vec::Vec<u8>> {
        ::std::mem::replace(&mut self.xForwardedFor, ::protobuf::RepeatedField::new())
    }

    // bool isTouch = 10;


    pub fn get_isTouch(&self) -> bool {
        self.isTouch
    }
    pub fn clear_isTouch(&mut self) {
        self.isTouch = false;
    }

    // Param is passed by value, moved
    pub fn set_isTouch(&mut self, v: bool) {
        self.isTouch = v;
    }

    // uint32 muxID = 11;


    pub fn get_muxID(&self) -> u32 {
        self.muxID
    }
    pub fn clear_muxID(&mut self) {
        self.muxID = 0;
    }

    // Param is passed by value, moved
    pub fn set_muxID(&mut self, v: u32) {
        self.muxID = v;
    }

    // string platform = 12;


    pub fn get_platform(&self) -> &str {
        &self.platform
    }
    pub fn clear_platform(&mut self) {
        self.platform.clear();
    }

    // Param is passed by value, moved
    pub fn set_platform(&mut self, v: ::std::string::String) {
        self.platform = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_platform(&mut self) -> &mut ::std::string::String {
        &mut self.platform
    }

    // Take field
    pub fn take_platform(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.platform, ::std::string::String::new())
    }

    // bytes requestID = 13;


    pub fn get_requestID(&self) -> &[u8] {
        &self.requestID
    }
    pub fn clear_requestID(&mut self) {
        self.requestID.clear();
    }

    // Param is passed by value, moved
    pub fn set_requestID(&mut self, v: ::std::vec::Vec<u8>) {
        self.requestID = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_requestID(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.requestID
    }

    // Take field
    pub fn take_requestID(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.requestID, ::std::vec::Vec::new())
    }
}

impl ::protobuf::Message for Foliv {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.userHash)?;
                },
                2 => {
                    ::protobuf::rt::read_proto3_enum_with_unknown_fields_into(wire_type, is, &mut self.command, 2, &mut self.unknown_fields)?
                },
                3 => {
                    ::protobuf::rt::read_proto3_enum_with_unknown_fields_into(wire_type, is, &mut self.addressType, 3, &mut self.unknown_fields)?
                },
                4 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.address)?;
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.port = tmp;
                },
                6 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.sourceName)?;
                },
                7 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.routerName)?;
                },
                8 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.processName)?;
                },
                9 => {
                    ::protobuf::rt::read_repeated_bytes_into(wire_type, is, &mut self.xForwardedFor)?;
                },
                10 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.isTouch = tmp;
                },
                11 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.muxID = tmp;
                },
                12 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.platform)?;
                },
                13 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.requestID)?;
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
        if !self.userHash.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.userHash);
        }
        if self.command != Command::Empty {
            my_size += ::protobuf::rt::enum_size(2, self.command);
        }
        if self.addressType != AddressType::InvalidType {
            my_size += ::protobuf::rt::enum_size(3, self.addressType);
        }
        if !self.address.is_empty() {
            my_size += ::protobuf::rt::bytes_size(4, &self.address);
        }
        if self.port != 0 {
            my_size += ::protobuf::rt::value_size(5, self.port, ::protobuf::wire_format::WireTypeVarint);
        }
        if !self.sourceName.is_empty() {
            my_size += ::protobuf::rt::string_size(6, &self.sourceName);
        }
        if !self.routerName.is_empty() {
            my_size += ::protobuf::rt::string_size(7, &self.routerName);
        }
        if !self.processName.is_empty() {
            my_size += ::protobuf::rt::string_size(8, &self.processName);
        }
        for value in &self.xForwardedFor {
            my_size += ::protobuf::rt::bytes_size(9, &value);
        };
        if self.isTouch != false {
            my_size += 2;
        }
        if self.muxID != 0 {
            my_size += ::protobuf::rt::value_size(11, self.muxID, ::protobuf::wire_format::WireTypeVarint);
        }
        if !self.platform.is_empty() {
            my_size += ::protobuf::rt::string_size(12, &self.platform);
        }
        if !self.requestID.is_empty() {
            my_size += ::protobuf::rt::bytes_size(13, &self.requestID);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if !self.userHash.is_empty() {
            os.write_string(1, &self.userHash)?;
        }
        if self.command != Command::Empty {
            os.write_enum(2, ::protobuf::ProtobufEnum::value(&self.command))?;
        }
        if self.addressType != AddressType::InvalidType {
            os.write_enum(3, ::protobuf::ProtobufEnum::value(&self.addressType))?;
        }
        if !self.address.is_empty() {
            os.write_bytes(4, &self.address)?;
        }
        if self.port != 0 {
            os.write_uint32(5, self.port)?;
        }
        if !self.sourceName.is_empty() {
            os.write_string(6, &self.sourceName)?;
        }
        if !self.routerName.is_empty() {
            os.write_string(7, &self.routerName)?;
        }
        if !self.processName.is_empty() {
            os.write_string(8, &self.processName)?;
        }
        for v in &self.xForwardedFor {
            os.write_bytes(9, &v)?;
        };
        if self.isTouch != false {
            os.write_bool(10, self.isTouch)?;
        }
        if self.muxID != 0 {
            os.write_uint32(11, self.muxID)?;
        }
        if !self.platform.is_empty() {
            os.write_string(12, &self.platform)?;
        }
        if !self.requestID.is_empty() {
            os.write_bytes(13, &self.requestID)?;
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

    fn new() -> Foliv {
        Foliv::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "userHash",
                |m: &Foliv| { &m.userHash },
                |m: &mut Foliv| { &mut m.userHash },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeEnum<Command>>(
                "command",
                |m: &Foliv| { &m.command },
                |m: &mut Foliv| { &mut m.command },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeEnum<AddressType>>(
                "addressType",
                |m: &Foliv| { &m.addressType },
                |m: &mut Foliv| { &mut m.addressType },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                "address",
                |m: &Foliv| { &m.address },
                |m: &mut Foliv| { &mut m.address },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                "port",
                |m: &Foliv| { &m.port },
                |m: &mut Foliv| { &mut m.port },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "sourceName",
                |m: &Foliv| { &m.sourceName },
                |m: &mut Foliv| { &mut m.sourceName },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "routerName",
                |m: &Foliv| { &m.routerName },
                |m: &mut Foliv| { &mut m.routerName },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "processName",
                |m: &Foliv| { &m.processName },
                |m: &mut Foliv| { &mut m.processName },
            ));
            fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                "xForwardedFor",
                |m: &Foliv| { &m.xForwardedFor },
                |m: &mut Foliv| { &mut m.xForwardedFor },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                "isTouch",
                |m: &Foliv| { &m.isTouch },
                |m: &mut Foliv| { &mut m.isTouch },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                "muxID",
                |m: &Foliv| { &m.muxID },
                |m: &mut Foliv| { &mut m.muxID },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "platform",
                |m: &Foliv| { &m.platform },
                |m: &mut Foliv| { &mut m.platform },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                "requestID",
                |m: &Foliv| { &m.requestID },
                |m: &mut Foliv| { &mut m.requestID },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<Foliv>(
                "Foliv",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static Foliv {
        static instance: ::protobuf::rt::LazyV2<Foliv> = ::protobuf::rt::LazyV2::INIT;
        instance.get(Foliv::new)
    }
}

impl ::protobuf::Clear for Foliv {
    fn clear(&mut self) {
        self.userHash.clear();
        self.command = Command::Empty;
        self.addressType = AddressType::InvalidType;
        self.address.clear();
        self.port = 0;
        self.sourceName.clear();
        self.routerName.clear();
        self.processName.clear();
        self.xForwardedFor.clear();
        self.isTouch = false;
        self.muxID = 0;
        self.platform.clear();
        self.requestID.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Foliv {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Foliv {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum Command {
    Empty = 0,
    Connect = 1,
    Associate = 3,
    Mux = 127,
}

impl ::protobuf::ProtobufEnum for Command {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<Command> {
        match value {
            0 => ::std::option::Option::Some(Command::Empty),
            1 => ::std::option::Option::Some(Command::Connect),
            3 => ::std::option::Option::Some(Command::Associate),
            127 => ::std::option::Option::Some(Command::Mux),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [Command] = &[
            Command::Empty,
            Command::Connect,
            Command::Associate,
            Command::Mux,
        ];
        values
    }

    fn enum_descriptor_static() -> &'static ::protobuf::reflect::EnumDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            ::protobuf::reflect::EnumDescriptor::new_pb_name::<Command>("Command", file_descriptor_proto())
        })
    }
}

impl ::std::marker::Copy for Command {
}

impl ::std::default::Default for Command {
    fn default() -> Self {
        Command::Empty
    }
}

impl ::protobuf::reflect::ProtobufValue for Command {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Enum(::protobuf::ProtobufEnum::descriptor(self))
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum AddressType {
    InvalidType = 0,
    IPv4 = 1,
    DomainName = 3,
    IPv6 = 4,
}

impl ::protobuf::ProtobufEnum for AddressType {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<AddressType> {
        match value {
            0 => ::std::option::Option::Some(AddressType::InvalidType),
            1 => ::std::option::Option::Some(AddressType::IPv4),
            3 => ::std::option::Option::Some(AddressType::DomainName),
            4 => ::std::option::Option::Some(AddressType::IPv6),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [AddressType] = &[
            AddressType::InvalidType,
            AddressType::IPv4,
            AddressType::DomainName,
            AddressType::IPv6,
        ];
        values
    }

    fn enum_descriptor_static() -> &'static ::protobuf::reflect::EnumDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            ::protobuf::reflect::EnumDescriptor::new_pb_name::<AddressType>("AddressType", file_descriptor_proto())
        })
    }
}

impl ::std::marker::Copy for AddressType {
}

impl ::std::default::Default for AddressType {
    fn default() -> Self {
        AddressType::InvalidType
    }
}

impl ::protobuf::reflect::ProtobufValue for AddressType {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Enum(::protobuf::ProtobufEnum::descriptor(self))
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x0bfoliv.proto\x12\x05foliv\"\xa3\x03\n\x05Foliv\x12\x1a\n\x08userHas\
    h\x18\x01\x20\x01(\tR\x08userHash\x12(\n\x07command\x18\x02\x20\x01(\x0e\
    2\x0e.foliv.CommandR\x07command\x124\n\x0baddressType\x18\x03\x20\x01(\
    \x0e2\x12.foliv.AddressTypeR\x0baddressType\x12\x18\n\x07address\x18\x04\
    \x20\x01(\x0cR\x07address\x12\x12\n\x04port\x18\x05\x20\x01(\rR\x04port\
    \x12\x1e\n\nsourceName\x18\x06\x20\x01(\tR\nsourceName\x12\x1e\n\nrouter\
    Name\x18\x07\x20\x01(\tR\nrouterName\x12\x20\n\x0bprocessName\x18\x08\
    \x20\x01(\tR\x0bprocessName\x12$\n\rxForwardedFor\x18\t\x20\x03(\x0cR\rx\
    ForwardedFor\x12\x18\n\x07isTouch\x18\n\x20\x01(\x08R\x07isTouch\x12\x14\
    \n\x05muxID\x18\x0b\x20\x01(\rR\x05muxID\x12\x1a\n\x08platform\x18\x0c\
    \x20\x01(\tR\x08platform\x12\x1c\n\trequestID\x18\r\x20\x01(\x0cR\treque\
    stID*9\n\x07Command\x12\t\n\x05Empty\x10\0\x12\x0b\n\x07Connect\x10\x01\
    \x12\r\n\tAssociate\x10\x03\x12\x07\n\x03Mux\x10\x7f*B\n\x0bAddressType\
    \x12\x0f\n\x0bInvalidType\x10\0\x12\x08\n\x04IPv4\x10\x01\x12\x0e\n\nDom\
    ainName\x10\x03\x12\x08\n\x04IPv6\x10\x04B\tZ\x07./folivJ\xd9\x20\n\x06\
    \x12\x04(\0I\x01\n\xc6\x17\n\x01\x0c\x12\x03(\0\x122\xbb\x17\x20?O$$O??7\
    7O7>:!!7$O$O>>7?O$?CC7>7C$$QO77!!>7777>>>!!!>!!!!!!>7>>>>>>>>>>>>>>\n\
    \x20?O$OO??>?O7>:!>7$Q$OQOO?OO?CC?77O$Q$C77777?????77777>>>>>>>>>>>>>>>>\
    >>>>>>>\n\x20?$$OO?7>?C>OQ$$OHHQC?$$QHHNHHO>?$$QO77>>7?????????7>>>!!!!!\
    >>>>>>>>>>>>>>>>\n\x20C$$OC?7>O$?!O$N$$QQNCCCC7?QN$Q?OOCOC>7!77??CCCC?C?\
    ?7>>>>>>!>>>>>>>>>>>>>>>>\n\x20OOOO??>7O7QNNNHQNNQHHHHHQ?7C7OH$QQO7>>!7?\
    ??$$$$$$$$OCC???7>>>>>>>>>>>>>>>>>\n\x20$OOO??>?CHQ7NNHHQ$HHC!>7$OOC!:CQ\
    OO$$7!!7???$$$$$$$$$O$OC?7>>>>>>>>>>>>>>>!!\n\x20??CC?7??C>7$NHQOQOOQCOO\
    O?>!>?O>:7CCCQH>??7?COOOOO$$$$OCC77>>>>>>>>>>>>>!!>!\n\x20?C??C777H$CHH$\
    O?OOQO?7COQ$Q$O!QO!>CC?OCCH???????????77>>>!!>>>!!>>>!!!>!>!!\n\x20>>77?\
    7?HQO7HQOO?O$$C>>COOO$$$$>Q?>7>?>7C??Q>>>>>>>7>>>>>>>>>>!>!!!!!!!!!!>!\n\
    \x20?????!:>7>CH$$OCOOO?!!>!!:!7C$OCQO:>:7?C7?7QQ?????????????7>!!>>>>>>\
    >>>!!!?\n\x20>>>>>>$NQ>$$?77?OOO>:!:QNNNH->?$Q$?:7!!??7>>$$O>77777>>777>\
    >>7!!!!!!!!!!!??\n\x20>>>>7C7-:>O?7>>>7O$>::7;-?NHH-!?$QH7:C!>>C7!7OO$77\
    >>>777>>>>>77>>>77>>77C?7\n\x20>>>>OQ;;-CC?77>>!>CC:?!..:QQQO!!!$QO:>>>!\
    >>>!7OC$C77>>>>>>7>>>>>>>>!!!!C777\n\x20>>>7?CO>$CC7777>!!7O$>7::$$$O7!!\
    >77$C>7:>:>>>!?CONO77>>>>>>>>>>>>>>!!?C7777\n\x20?>>C?CQQ?7>>>7>!!!>?O$Q\
    QOC>>!!>>>!!!C7?!>::>>!!?C7QNN?77777777>>>>>7C?77777\n\x20???CQQ$7>::::!\
    !!!:!7>7COC$$$Q$O7>!:-!C-7:>::!>!!?7-QNNNQ$$$$$$$OOOCC?777777\n\x20777CO\
    O7>:---:;--::!!!!!!!7????CC>:!::?--7!:!!!>!:7!7QHHNNO>>>>>>?CC77777>>!\n\
    \x20>>>>O?7!::::--:::::::-:-::>77?77>:::!>:-:!!::!!!>>>-$QQQHNNO7>>?C?7>\
    >>7>!!:\n\x20!>!!??7>!::---:::---;..::!!>>777>--:!:::-!!>:!::!!:-O$$$$QH\
    NNH??77>>>>!:!::\n\x20>>>>>?>>!!::-;;;.;;;;;--!>>>>>>>:::::!:::!::::!-!!\
    ::COO$O$$QHNMC7>!>>:::!::\n\x2077777?>>!!::-;.-----::-->>>>>!!:-:-::::::\
    ::::!:::::7??CCCO$$$QQNNC>!:::::::\n\x20???????>!!!::-:!!::::::!>!>>>!!-\
    ---:::---::::::::::?????7COO$$$$QHN?:::::::\n\x20????????!!!:!:::::::::!\
    >!!!!!:----:::-:::::::::-:::7???777??OO$$$$QQH>!::::\n\x20CCCCCCCCC>!!:!\
    :::::::!>>!!!::---:-:::::::::::::::-77777>>>77??C$$OO$$H7::::\n\x20CCCCC\
    CCCCC7:!::::::!!!!!!::---:-:::::-:-:--::-::-77777>>>>>>77777CCOO$Q7:::\n\
    \x20CCCCCCCCC7>?::::::::::::--------------:----:-:-->>7777>>>>>>>>>77?CO\
    OO$Q7::\n\x20CCCCCCCCC>>?>::-:--:-------------;------------->>>>7>>>>>>>\
    >>>7777??OOO$Q!:\n\x20CCCCCCCC?>>?7-:-------------;-;;-:-:---------->>>>\
    >>>>>>>>>>>>>>>77??COO$Q:\n\x20CCCCCCCC>>>??>-:-------------------------\
    ---;>>>>>>!>>!!>>>>>>>>>>77??CCO$Q\n\x20CCCCCCC?>>>777>::-----------;---\
    ----------;:>>>>>>!>>!!!!!!>>>>>>>>77?CCCO$\n\x20CCCCCCC>!!>7>7>>!:-----\
    ---;---;;--;---;;-;!!!>>>>!>>>!!!!!!>>>>>>>>>7??CCCO\n\x20CCCCCCC>!!>7>>\
    >>>>;:--;-;;--;;-;-----;;-!!!!!!!>!>>>!!!!!!!>>>>>>>>>7???COO\n\x20CCCCC\
    C7!!!!>>>>>>>>!;-----;;;;-;;;;-;:!!!!!!!!!!>>>>!!!!!!!>>>>>>>>>77??CCC\n\
    \x20QQQQQQQQQQ$$C?777>>>!!;;;;;-;;;;;;;:!!!!!!!!!:!>>>>!!>!>!!!>>>>>>>>>\
    77????C\n\x20>>>>>>>>>>>!>>>>>>>>>>>>>>>OQ;.::::!!!!!!!!:!!!>>>>>!!>!!!!\
    !>>>>>>>>77????C\n\x20>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>!!!!!!!!!!!:!!!!!>>>\
    >>>>!!:!!!>>>>>>>>77???CC\n\x20>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>:!!!!!!!::!\
    !!!!!!!!!>>!!!!:!!!>>>>>>>>>77???C\n\x20!!!!>>!!!!!!!!!!!!!!!!>>>>>>>7;;\
    !>:-::!!!!!!!!!!!!!!!!!::!!!>>>>>>>>>777???\n\x20!!!!!!!!!!!!!!!!!!>!!!!\
    !!!>!!>>-::::!!!!!!!!!!!!!!!!!!!::!!!>>>>>>>>>7777??\n\n\x08\n\x01\x02\
    \x12\x03*\0\x0e\n\x08\n\x01\x08\x12\x03+\0\x1e\n\t\n\x02\x08\x0b\x12\x03\
    +\0\x1e\n\n\n\x02\x05\0\x12\x04-\02\x01\n\n\n\x03\x05\0\x01\x12\x03-\x05\
    \x0c\n\x0b\n\x04\x05\0\x02\0\x12\x03.\x08\x16\n\x0c\n\x05\x05\0\x02\0\
    \x01\x12\x03.\x08\r\n\x0c\n\x05\x05\0\x02\0\x02\x12\x03.\x14\x15\n\x0b\n\
    \x04\x05\0\x02\x01\x12\x03/\x08\x16\n\x0c\n\x05\x05\0\x02\x01\x01\x12\
    \x03/\x08\x0f\n\x0c\n\x05\x05\0\x02\x01\x02\x12\x03/\x14\x15\n\x0b\n\x04\
    \x05\0\x02\x02\x12\x030\x08\x16\n\x0c\n\x05\x05\0\x02\x02\x01\x12\x030\
    \x08\x11\n\x0c\n\x05\x05\0\x02\x02\x02\x12\x030\x14\x15\n\x0b\n\x04\x05\
    \0\x02\x03\x12\x031\x08\x19\n\x0c\n\x05\x05\0\x02\x03\x01\x12\x031\x08\
    \x0b\n\x0c\n\x05\x05\0\x02\x03\x02\x12\x031\x14\x18\n\n\n\x02\x05\x01\
    \x12\x044\09\x01\n\n\n\x03\x05\x01\x01\x12\x034\x05\x10\n\x0b\n\x04\x05\
    \x01\x02\0\x12\x035\x08\x18\n\x0c\n\x05\x05\x01\x02\0\x01\x12\x035\x08\
    \x13\n\x0c\n\x05\x05\x01\x02\0\x02\x12\x035\x16\x17\n\x0b\n\x04\x05\x01\
    \x02\x01\x12\x036\x08\x18\n\x0c\n\x05\x05\x01\x02\x01\x01\x12\x036\x08\
    \x0c\n\x0c\n\x05\x05\x01\x02\x01\x02\x12\x036\x16\x17\n\x0b\n\x04\x05\
    \x01\x02\x02\x12\x037\x08\x18\n\x0c\n\x05\x05\x01\x02\x02\x01\x12\x037\
    \x08\x12\n\x0c\n\x05\x05\x01\x02\x02\x02\x12\x037\x16\x17\n\x0b\n\x04\
    \x05\x01\x02\x03\x12\x038\x08\x18\n\x0c\n\x05\x05\x01\x02\x03\x01\x12\
    \x038\x08\x0c\n\x0c\n\x05\x05\x01\x02\x03\x02\x12\x038\x16\x17\n\n\n\x02\
    \x04\0\x12\x04;\0I\x01\n\n\n\x03\x04\0\x01\x12\x03;\x08\r\n\x0b\n\x04\
    \x04\0\x02\0\x12\x03<\x08$\n\x0c\n\x05\x04\0\x02\0\x05\x12\x03<\x08\x0e\
    \n\x0c\n\x05\x04\0\x02\0\x01\x12\x03<\x17\x1f\n\x0c\n\x05\x04\0\x02\0\
    \x03\x12\x03<\"#\n\x0b\n\x04\x04\0\x02\x01\x12\x03=\x08#\n\x0c\n\x05\x04\
    \0\x02\x01\x06\x12\x03=\x08\x0f\n\x0c\n\x05\x04\0\x02\x01\x01\x12\x03=\
    \x17\x1e\n\x0c\n\x05\x04\0\x02\x01\x03\x12\x03=!\"\n\x0b\n\x04\x04\0\x02\
    \x02\x12\x03>\x08'\n\x0c\n\x05\x04\0\x02\x02\x06\x12\x03>\x08\x13\n\x0c\
    \n\x05\x04\0\x02\x02\x01\x12\x03>\x17\"\n\x0c\n\x05\x04\0\x02\x02\x03\
    \x12\x03>%&\n\x0b\n\x04\x04\0\x02\x03\x12\x03?\x08#\n\x0c\n\x05\x04\0\
    \x02\x03\x05\x12\x03?\x08\r\n\x0c\n\x05\x04\0\x02\x03\x01\x12\x03?\x17\
    \x1e\n\x0c\n\x05\x04\0\x02\x03\x03\x12\x03?!\"\n\x0b\n\x04\x04\0\x02\x04\
    \x12\x03@\x08\x20\n\x0c\n\x05\x04\0\x02\x04\x05\x12\x03@\x08\x0e\n\x0c\n\
    \x05\x04\0\x02\x04\x01\x12\x03@\x17\x1b\n\x0c\n\x05\x04\0\x02\x04\x03\
    \x12\x03@\x1e\x1f\n\x0b\n\x04\x04\0\x02\x05\x12\x03A\x08&\n\x0c\n\x05\
    \x04\0\x02\x05\x05\x12\x03A\x08\x0e\n\x0c\n\x05\x04\0\x02\x05\x01\x12\
    \x03A\x17!\n\x0c\n\x05\x04\0\x02\x05\x03\x12\x03A$%\n\x0b\n\x04\x04\0\
    \x02\x06\x12\x03B\x08&\n\x0c\n\x05\x04\0\x02\x06\x05\x12\x03B\x08\x0e\n\
    \x0c\n\x05\x04\0\x02\x06\x01\x12\x03B\x17!\n\x0c\n\x05\x04\0\x02\x06\x03\
    \x12\x03B$%\n\x0b\n\x04\x04\0\x02\x07\x12\x03C\x08'\n\x0c\n\x05\x04\0\
    \x02\x07\x05\x12\x03C\x08\x0e\n\x0c\n\x05\x04\0\x02\x07\x01\x12\x03C\x17\
    \"\n\x0c\n\x05\x04\0\x02\x07\x03\x12\x03C%&\n\x0b\n\x04\x04\0\x02\x08\
    \x12\x03D\x08)\n\x0c\n\x05\x04\0\x02\x08\x04\x12\x03D\x08\x10\n\x0c\n\
    \x05\x04\0\x02\x08\x05\x12\x03D\x11\x16\n\x0c\n\x05\x04\0\x02\x08\x01\
    \x12\x03D\x17$\n\x0c\n\x05\x04\0\x02\x08\x03\x12\x03D'(\n\x0b\n\x04\x04\
    \0\x02\t\x12\x03E\x08$\n\x0c\n\x05\x04\0\x02\t\x05\x12\x03E\x08\x0c\n\
    \x0c\n\x05\x04\0\x02\t\x01\x12\x03E\x17\x1e\n\x0c\n\x05\x04\0\x02\t\x03\
    \x12\x03E!#\n\x0b\n\x04\x04\0\x02\n\x12\x03F\x08\"\n\x0c\n\x05\x04\0\x02\
    \n\x05\x12\x03F\x08\x0e\n\x0c\n\x05\x04\0\x02\n\x01\x12\x03F\x17\x1c\n\
    \x0c\n\x05\x04\0\x02\n\x03\x12\x03F\x1f!\n\x0b\n\x04\x04\0\x02\x0b\x12\
    \x03G\x08%\n\x0c\n\x05\x04\0\x02\x0b\x05\x12\x03G\x08\x0e\n\x0c\n\x05\
    \x04\0\x02\x0b\x01\x12\x03G\x17\x1f\n\x0c\n\x05\x04\0\x02\x0b\x03\x12\
    \x03G\"$\n\x0b\n\x04\x04\0\x02\x0c\x12\x03H\x08&\n\x0c\n\x05\x04\0\x02\
    \x0c\x05\x12\x03H\x08\r\n\x0c\n\x05\x04\0\x02\x0c\x01\x12\x03H\x17\x20\n\
    \x0c\n\x05\x04\0\x02\x0c\x03\x12\x03H#%b\x06proto3\
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
