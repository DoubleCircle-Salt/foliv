// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

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
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Foliv {}

impl Foliv {
    pub fn new() -> Foliv {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Foliv {
        static mut instance: ::protobuf::lazy::Lazy<Foliv> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Foliv,
        };
        unsafe {
            instance.get(Foliv::new)
        }
    }

    // string userHash = 1;

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

    pub fn get_userHash(&self) -> &str {
        &self.userHash
    }

    fn get_userHash_for_reflect(&self) -> &::std::string::String {
        &self.userHash
    }

    fn mut_userHash_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.userHash
    }

    // .foliv.Command command = 2;

    pub fn clear_command(&mut self) {
        self.command = Command::Empty;
    }

    // Param is passed by value, moved
    pub fn set_command(&mut self, v: Command) {
        self.command = v;
    }

    pub fn get_command(&self) -> Command {
        self.command
    }

    fn get_command_for_reflect(&self) -> &Command {
        &self.command
    }

    fn mut_command_for_reflect(&mut self) -> &mut Command {
        &mut self.command
    }

    // .foliv.AddressType addressType = 3;

    pub fn clear_addressType(&mut self) {
        self.addressType = AddressType::InvalidType;
    }

    // Param is passed by value, moved
    pub fn set_addressType(&mut self, v: AddressType) {
        self.addressType = v;
    }

    pub fn get_addressType(&self) -> AddressType {
        self.addressType
    }

    fn get_addressType_for_reflect(&self) -> &AddressType {
        &self.addressType
    }

    fn mut_addressType_for_reflect(&mut self) -> &mut AddressType {
        &mut self.addressType
    }

    // bytes address = 4;

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

    pub fn get_address(&self) -> &[u8] {
        &self.address
    }

    fn get_address_for_reflect(&self) -> &::std::vec::Vec<u8> {
        &self.address
    }

    fn mut_address_for_reflect(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.address
    }

    // uint32 port = 5;

    pub fn clear_port(&mut self) {
        self.port = 0;
    }

    // Param is passed by value, moved
    pub fn set_port(&mut self, v: u32) {
        self.port = v;
    }

    pub fn get_port(&self) -> u32 {
        self.port
    }

    fn get_port_for_reflect(&self) -> &u32 {
        &self.port
    }

    fn mut_port_for_reflect(&mut self) -> &mut u32 {
        &mut self.port
    }

    // string sourceName = 6;

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

    pub fn get_sourceName(&self) -> &str {
        &self.sourceName
    }

    fn get_sourceName_for_reflect(&self) -> &::std::string::String {
        &self.sourceName
    }

    fn mut_sourceName_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.sourceName
    }

    // string routerName = 7;

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

    pub fn get_routerName(&self) -> &str {
        &self.routerName
    }

    fn get_routerName_for_reflect(&self) -> &::std::string::String {
        &self.routerName
    }

    fn mut_routerName_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.routerName
    }

    // string processName = 8;

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

    pub fn get_processName(&self) -> &str {
        &self.processName
    }

    fn get_processName_for_reflect(&self) -> &::std::string::String {
        &self.processName
    }

    fn mut_processName_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.processName
    }

    // repeated bytes xForwardedFor = 9;

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

    pub fn get_xForwardedFor(&self) -> &[::std::vec::Vec<u8>] {
        &self.xForwardedFor
    }

    fn get_xForwardedFor_for_reflect(&self) -> &::protobuf::RepeatedField<::std::vec::Vec<u8>> {
        &self.xForwardedFor
    }

    fn mut_xForwardedFor_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<::std::vec::Vec<u8>> {
        &mut self.xForwardedFor
    }

    // bool isTouch = 10;

    pub fn clear_isTouch(&mut self) {
        self.isTouch = false;
    }

    // Param is passed by value, moved
    pub fn set_isTouch(&mut self, v: bool) {
        self.isTouch = v;
    }

    pub fn get_isTouch(&self) -> bool {
        self.isTouch
    }

    fn get_isTouch_for_reflect(&self) -> &bool {
        &self.isTouch
    }

    fn mut_isTouch_for_reflect(&mut self) -> &mut bool {
        &mut self.isTouch
    }

    // uint32 muxID = 11;

    pub fn clear_muxID(&mut self) {
        self.muxID = 0;
    }

    // Param is passed by value, moved
    pub fn set_muxID(&mut self, v: u32) {
        self.muxID = v;
    }

    pub fn get_muxID(&self) -> u32 {
        self.muxID
    }

    fn get_muxID_for_reflect(&self) -> &u32 {
        &self.muxID
    }

    fn mut_muxID_for_reflect(&mut self) -> &mut u32 {
        &mut self.muxID
    }
}

impl ::protobuf::Message for Foliv {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
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
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.userHash.is_empty() {
            os.write_string(1, &self.userHash)?;
        }
        if self.command != Command::Empty {
            os.write_enum(2, self.command.value())?;
        }
        if self.addressType != AddressType::InvalidType {
            os.write_enum(3, self.addressType.value())?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Foliv {
    fn new() -> Foliv {
        Foliv::new()
    }

    fn descriptor_static(_: ::std::option::Option<Foliv>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "userHash",
                    Foliv::get_userHash_for_reflect,
                    Foliv::mut_userHash_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeEnum<Command>>(
                    "command",
                    Foliv::get_command_for_reflect,
                    Foliv::mut_command_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeEnum<AddressType>>(
                    "addressType",
                    Foliv::get_addressType_for_reflect,
                    Foliv::mut_addressType_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "address",
                    Foliv::get_address_for_reflect,
                    Foliv::mut_address_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "port",
                    Foliv::get_port_for_reflect,
                    Foliv::mut_port_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "sourceName",
                    Foliv::get_sourceName_for_reflect,
                    Foliv::mut_sourceName_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "routerName",
                    Foliv::get_routerName_for_reflect,
                    Foliv::mut_routerName_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "processName",
                    Foliv::get_processName_for_reflect,
                    Foliv::mut_processName_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "xForwardedFor",
                    Foliv::get_xForwardedFor_for_reflect,
                    Foliv::mut_xForwardedFor_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "isTouch",
                    Foliv::get_isTouch_for_reflect,
                    Foliv::mut_isTouch_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "muxID",
                    Foliv::get_muxID_for_reflect,
                    Foliv::mut_muxID_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Foliv>(
                    "Foliv",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Foliv {
    fn clear(&mut self) {
        self.clear_userHash();
        self.clear_command();
        self.clear_addressType();
        self.clear_address();
        self.clear_port();
        self.clear_sourceName();
        self.clear_routerName();
        self.clear_processName();
        self.clear_xForwardedFor();
        self.clear_isTouch();
        self.clear_muxID();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Foliv {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Foliv {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
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

    fn enum_descriptor_static(_: ::std::option::Option<Command>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("Command", file_descriptor_proto())
            })
        }
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
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
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

    fn enum_descriptor_static(_: ::std::option::Option<AddressType>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("AddressType", file_descriptor_proto())
            })
        }
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
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x0bfoliv.proto\x12\x05foliv\"\xe9\x02\n\x05Foliv\x12\x1a\n\x08userHas\
    h\x18\x01\x20\x01(\tR\x08userHash\x12(\n\x07command\x18\x02\x20\x01(\x0e\
    2\x0e.foliv.CommandR\x07command\x124\n\x0baddressType\x18\x03\x20\x01(\
    \x0e2\x12.foliv.AddressTypeR\x0baddressType\x12\x18\n\x07address\x18\x04\
    \x20\x01(\x0cR\x07address\x12\x12\n\x04port\x18\x05\x20\x01(\rR\x04port\
    \x12\x1e\n\nsourceName\x18\x06\x20\x01(\tR\nsourceName\x12\x1e\n\nrouter\
    Name\x18\x07\x20\x01(\tR\nrouterName\x12\x20\n\x0bprocessName\x18\x08\
    \x20\x01(\tR\x0bprocessName\x12$\n\rxForwardedFor\x18\t\x20\x03(\x0cR\rx\
    ForwardedFor\x12\x18\n\x07isTouch\x18\n\x20\x01(\x08R\x07isTouch\x12\x14\
    \n\x05muxID\x18\x0b\x20\x01(\rR\x05muxID*9\n\x07Command\x12\t\n\x05Empty\
    \x10\0\x12\x0b\n\x07Connect\x10\x01\x12\r\n\tAssociate\x10\x03\x12\x07\n\
    \x03Mux\x10\x7f*B\n\x0bAddressType\x12\x0f\n\x0bInvalidType\x10\0\x12\
    \x08\n\x04IPv4\x10\x01\x12\x0e\n\nDomainName\x10\x03\x12\x08\n\x04IPv6\
    \x10\x04B\tZ\x07./folivJ\xeb\x1f\n\x06\x12\x04(\0G\x01\n\xc6\x17\n\x01\
    \x0c\x12\x03(\0\x122\xbb\x17\x20?O$$O??77O7>:!!7$O$O>>7?O$?CC7>7C$$QO77!\
    !>7777>>>!!!>!!!!!!>7>>>>>>>>>>>>>>\n\x20?O$OO??>?O7>:!>7$Q$OQOO?OO?CC?7\
    7O$Q$C77777?????77777>>>>>>>>>>>>>>>>>>>>>>>\n\x20?$$OO?7>?C>OQ$$OHHQC?$\
    $QHHNHHO>?$$QO77>>7?????????7>>>!!!!!>>>>>>>>>>>>>>>>\n\x20C$$OC?7>O$?!O\
    $N$$QQNCCCC7?QN$Q?OOCOC>7!77??CCCC?C??7>>>>>>!>>>>>>>>>>>>>>>>\n\x20OOOO\
    ??>7O7QNNNHQNNQHHHHHQ?7C7OH$QQO7>>!7???$$$$$$$$OCC???7>>>>>>>>>>>>>>>>>\
    \n\x20$OOO??>?CHQ7NNHHQ$HHC!>7$OOC!:CQOO$$7!!7???$$$$$$$$$O$OC?7>>>>>>>>\
    >>>>>>>!!\n\x20??CC?7??C>7$NHQOQOOQCOOO?>!>?O>:7CCCQH>??7?COOOOO$$$$OCC7\
    7>>>>>>>>>>>>>!!>!\n\x20?C??C777H$CHH$O?OOQO?7COQ$Q$O!QO!>CC?OCCH???????\
    ????77>>>!!>>>!!>>>!!!>!>!!\n\x20>>77?7?HQO7HQOO?O$$C>>COOO$$$$>Q?>7>?>7\
    C??Q>>>>>>>7>>>>>>>>>>!>!!!!!!!!!!>!\n\x20?????!:>7>CH$$OCOOO?!!>!!:!7C$\
    OCQO:>:7?C7?7QQ?????????????7>!!>>>>>>>>>!!!?\n\x20>>>>>>$NQ>$$?77?OOO>:\
    !:QNNNH->?$Q$?:7!!??7>>$$O>77777>>777>>>7!!!!!!!!!!!??\n\x20>>>>7C7-:>O?\
    7>>>7O$>::7;-?NHH-!?$QH7:C!>>C7!7OO$77>>>777>>>>>77>>>77>>77C?7\n\x20>>>\
    >OQ;;-CC?77>>!>CC:?!..:QQQO!!!$QO:>>>!>>>!7OC$C77>>>>>>7>>>>>>>>!!!!C777\
    \n\x20>>>7?CO>$CC7777>!!7O$>7::$$$O7!!>77$C>7:>:>>>!?CONO77>>>>>>>>>>>>>\
    >!!?C7777\n\x20?>>C?CQQ?7>>>7>!!!>?O$QQOC>>!!>>>!!!C7?!>::>>!!?C7QNN?777\
    77777>>>>>7C?77777\n\x20???CQQ$7>::::!!!!:!7>7COC$$$Q$O7>!:-!C-7:>::!>!!\
    ?7-QNNNQ$$$$$$$OOOCC?777777\n\x20777COO7>:---:;--::!!!!!!!7????CC>:!::?-\
    -7!:!!!>!:7!7QHHNNO>>>>>>?CC77777>>!\n\x20>>>>O?7!::::--:::::::-:-::>77?\
    77>:::!>:-:!!::!!!>>>-$QQQHNNO7>>?C?7>>>7>!!:\n\x20!>!!??7>!::---:::---;\
    ..::!!>>777>--:!:::-!!>:!::!!:-O$$$$QHNNH??77>>>>!:!::\n\x20>>>>>?>>!!::\
    -;;;.;;;;;--!>>>>>>>:::::!:::!::::!-!!::COO$O$$QHNMC7>!>>:::!::\n\x20777\
    77?>>!!::-;.-----::-->>>>>!!:-:-::::::::::!:::::7??CCCO$$$QQNNC>!:::::::\
    \n\x20???????>!!!::-:!!::::::!>!>>>!!----:::---::::::::::?????7COO$$$$QH\
    N?:::::::\n\x20????????!!!:!:::::::::!>!!!!!:----:::-:::::::::-:::7???77\
    7??OO$$$$QQH>!::::\n\x20CCCCCCCCC>!!:!:::::::!>>!!!::---:-::::::::::::::\
    :-77777>>>77??C$$OO$$H7::::\n\x20CCCCCCCCCC7:!::::::!!!!!!::---:-:::::-:\
    -:--::-::-77777>>>>>>77777CCOO$Q7:::\n\x20CCCCCCCCC7>?::::::::::::------\
    --------:----:-:-->>7777>>>>>>>>>77?COOO$Q7::\n\x20CCCCCCCCC>>?>::-:--:-\
    ------------;------------->>>>7>>>>>>>>>>7777??OOO$Q!:\n\x20CCCCCCCC?>>?\
    7-:-------------;-;;-:-:---------->>>>>>>>>>>>>>>>>>>77??COO$Q:\n\x20CCC\
    CCCCC>>>??>-:----------------------------;>>>>>>!>>!!>>>>>>>>>>77??CCO$Q\
    \n\x20CCCCCCC?>>>777>::-----------;-------------;:>>>>>>!>>!!!!!!>>>>>>>\
    >77?CCCO$\n\x20CCCCCCC>!!>7>7>>!:--------;---;;--;---;;-;!!!>>>>!>>>!!!!\
    !!>>>>>>>>>7??CCCO\n\x20CCCCCCC>!!>7>>>>>>;:--;-;;--;;-;-----;;-!!!!!!!>\
    !>>>!!!!!!!>>>>>>>>>7???COO\n\x20CCCCCC7!!!!>>>>>>>>!;-----;;;;-;;;;-;:!\
    !!!!!!!!!>>>>!!!!!!!>>>>>>>>>77??CCC\n\x20QQQQQQQQQQ$$C?777>>>!!;;;;;-;;\
    ;;;;;:!!!!!!!!!:!>>>>!!>!>!!!>>>>>>>>>77????C\n\x20>>>>>>>>>>>!>>>>>>>>>\
    >>>>>>OQ;.::::!!!!!!!!:!!!>>>>>!!>!!!!!>>>>>>>>77????C\n\x20>>>>>>>>>>>>\
    >>>>>>>>>>>>>>>>>>!!!!!!!!!!!:!!!!!>>>>>>>!!:!!!>>>>>>>>77???CC\n\x20>>>\
    >>>>>>>>>>>>>>>>>>>>>>>>>>>:!!!!!!!::!!!!!!!!!!>>!!!!:!!!>>>>>>>>>77???C\
    \n\x20!!!!>>!!!!!!!!!!!!!!!!>>>>>>>7;;!>:-::!!!!!!!!!!!!!!!!!::!!!>>>>>>\
    >>>777???\n\x20!!!!!!!!!!!!!!!!!!>!!!!!!!>!!>>-::::!!!!!!!!!!!!!!!!!!!::\
    !!!>>>>>>>>>7777??\n\n\x08\n\x01\x02\x12\x03*\0\x0e\n\x08\n\x01\x08\x12\
    \x03+\0\x1e\n\t\n\x02\x08\x0b\x12\x03+\0\x1e\n\n\n\x02\x05\0\x12\x04-\02\
    \x01\n\n\n\x03\x05\0\x01\x12\x03-\x05\x0c\n\x0b\n\x04\x05\0\x02\0\x12\
    \x03.\x08\x16\n\x0c\n\x05\x05\0\x02\0\x01\x12\x03.\x08\r\n\x0c\n\x05\x05\
    \0\x02\0\x02\x12\x03.\x14\x15\n\x0b\n\x04\x05\0\x02\x01\x12\x03/\x08\x16\
    \n\x0c\n\x05\x05\0\x02\x01\x01\x12\x03/\x08\x0f\n\x0c\n\x05\x05\0\x02\
    \x01\x02\x12\x03/\x14\x15\n\x0b\n\x04\x05\0\x02\x02\x12\x030\x08\x16\n\
    \x0c\n\x05\x05\0\x02\x02\x01\x12\x030\x08\x11\n\x0c\n\x05\x05\0\x02\x02\
    \x02\x12\x030\x14\x15\n\x0b\n\x04\x05\0\x02\x03\x12\x031\x08\x19\n\x0c\n\
    \x05\x05\0\x02\x03\x01\x12\x031\x08\x0b\n\x0c\n\x05\x05\0\x02\x03\x02\
    \x12\x031\x14\x18\n\n\n\x02\x05\x01\x12\x044\09\x01\n\n\n\x03\x05\x01\
    \x01\x12\x034\x05\x10\n\x0b\n\x04\x05\x01\x02\0\x12\x035\x08\x18\n\x0c\n\
    \x05\x05\x01\x02\0\x01\x12\x035\x08\x13\n\x0c\n\x05\x05\x01\x02\0\x02\
    \x12\x035\x16\x17\n\x0b\n\x04\x05\x01\x02\x01\x12\x036\x08\x18\n\x0c\n\
    \x05\x05\x01\x02\x01\x01\x12\x036\x08\x0c\n\x0c\n\x05\x05\x01\x02\x01\
    \x02\x12\x036\x16\x17\n\x0b\n\x04\x05\x01\x02\x02\x12\x037\x08\x18\n\x0c\
    \n\x05\x05\x01\x02\x02\x01\x12\x037\x08\x12\n\x0c\n\x05\x05\x01\x02\x02\
    \x02\x12\x037\x16\x17\n\x0b\n\x04\x05\x01\x02\x03\x12\x038\x08\x18\n\x0c\
    \n\x05\x05\x01\x02\x03\x01\x12\x038\x08\x0c\n\x0c\n\x05\x05\x01\x02\x03\
    \x02\x12\x038\x16\x17\n\n\n\x02\x04\0\x12\x04;\0G\x01\n\n\n\x03\x04\0\
    \x01\x12\x03;\x08\r\n\x0b\n\x04\x04\0\x02\0\x12\x03<\x08$\n\x0c\n\x05\
    \x04\0\x02\0\x05\x12\x03<\x08\x0e\n\x0c\n\x05\x04\0\x02\0\x01\x12\x03<\
    \x17\x1f\n\x0c\n\x05\x04\0\x02\0\x03\x12\x03<\"#\n\x0b\n\x04\x04\0\x02\
    \x01\x12\x03=\x08#\n\x0c\n\x05\x04\0\x02\x01\x06\x12\x03=\x08\x0f\n\x0c\
    \n\x05\x04\0\x02\x01\x01\x12\x03=\x17\x1e\n\x0c\n\x05\x04\0\x02\x01\x03\
    \x12\x03=!\"\n\x0b\n\x04\x04\0\x02\x02\x12\x03>\x08'\n\x0c\n\x05\x04\0\
    \x02\x02\x06\x12\x03>\x08\x13\n\x0c\n\x05\x04\0\x02\x02\x01\x12\x03>\x17\
    \"\n\x0c\n\x05\x04\0\x02\x02\x03\x12\x03>%&\n\x0b\n\x04\x04\0\x02\x03\
    \x12\x03?\x08#\n\x0c\n\x05\x04\0\x02\x03\x05\x12\x03?\x08\r\n\x0c\n\x05\
    \x04\0\x02\x03\x01\x12\x03?\x17\x1e\n\x0c\n\x05\x04\0\x02\x03\x03\x12\
    \x03?!\"\n\x0b\n\x04\x04\0\x02\x04\x12\x03@\x08\x20\n\x0c\n\x05\x04\0\
    \x02\x04\x05\x12\x03@\x08\x0e\n\x0c\n\x05\x04\0\x02\x04\x01\x12\x03@\x17\
    \x1b\n\x0c\n\x05\x04\0\x02\x04\x03\x12\x03@\x1e\x1f\n\x0b\n\x04\x04\0\
    \x02\x05\x12\x03A\x08&\n\x0c\n\x05\x04\0\x02\x05\x05\x12\x03A\x08\x0e\n\
    \x0c\n\x05\x04\0\x02\x05\x01\x12\x03A\x17!\n\x0c\n\x05\x04\0\x02\x05\x03\
    \x12\x03A$%\n\x0b\n\x04\x04\0\x02\x06\x12\x03B\x08&\n\x0c\n\x05\x04\0\
    \x02\x06\x05\x12\x03B\x08\x0e\n\x0c\n\x05\x04\0\x02\x06\x01\x12\x03B\x17\
    !\n\x0c\n\x05\x04\0\x02\x06\x03\x12\x03B$%\n\x0b\n\x04\x04\0\x02\x07\x12\
    \x03C\x08'\n\x0c\n\x05\x04\0\x02\x07\x05\x12\x03C\x08\x0e\n\x0c\n\x05\
    \x04\0\x02\x07\x01\x12\x03C\x17\"\n\x0c\n\x05\x04\0\x02\x07\x03\x12\x03C\
    %&\n\x0b\n\x04\x04\0\x02\x08\x12\x03D\x08)\n\x0c\n\x05\x04\0\x02\x08\x04\
    \x12\x03D\x08\x10\n\x0c\n\x05\x04\0\x02\x08\x05\x12\x03D\x11\x16\n\x0c\n\
    \x05\x04\0\x02\x08\x01\x12\x03D\x17$\n\x0c\n\x05\x04\0\x02\x08\x03\x12\
    \x03D'(\n\x0b\n\x04\x04\0\x02\t\x12\x03E\x08$\n\x0c\n\x05\x04\0\x02\t\
    \x05\x12\x03E\x08\x0c\n\x0c\n\x05\x04\0\x02\t\x01\x12\x03E\x17\x1e\n\x0c\
    \n\x05\x04\0\x02\t\x03\x12\x03E!#\n\x0b\n\x04\x04\0\x02\n\x12\x03F\x08\"\
    \n\x0c\n\x05\x04\0\x02\n\x05\x12\x03F\x08\x0e\n\x0c\n\x05\x04\0\x02\n\
    \x01\x12\x03F\x17\x1c\n\x0c\n\x05\x04\0\x02\n\x03\x12\x03F\x1f!b\x06prot\
    o3\
";

static mut file_descriptor_proto_lazy: ::protobuf::lazy::Lazy<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::lazy::Lazy {
    lock: ::protobuf::lazy::ONCE_INIT,
    ptr: 0 as *const ::protobuf::descriptor::FileDescriptorProto,
};

fn parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto {
    ::protobuf::parse_from_bytes(file_descriptor_proto_data).unwrap()
}

pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    unsafe {
        file_descriptor_proto_lazy.get(|| {
            parse_descriptor_proto()
        })
    }
}
