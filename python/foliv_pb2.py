# -*- coding: utf-8 -*-
# Generated by the protocol buffer compiler.  DO NOT EDIT!
# source: foliv.proto
"""Generated protocol buffer code."""
from google.protobuf.internal import enum_type_wrapper
from google.protobuf import descriptor as _descriptor
from google.protobuf import message as _message
from google.protobuf import reflection as _reflection
from google.protobuf import symbol_database as _symbol_database
# @@protoc_insertion_point(imports)

_sym_db = _symbol_database.Default()




DESCRIPTOR = _descriptor.FileDescriptor(
  name='foliv.proto',
  package='foliv',
  syntax='proto3',
  serialized_options=b'Z\007./foliv',
  create_key=_descriptor._internal_create_key,
  serialized_pb=b'\n\x0b\x66oliv.proto\x12\x05\x66oliv\"\xaf\x03\n\x05\x46oliv\x12\x10\n\x08userHash\x18\x01 \x01(\t\x12\x1f\n\x07\x63ommand\x18\x02 \x01(\x0e\x32\x0e.foliv.Command\x12\'\n\x0b\x61\x64\x64ressType\x18\x03 \x01(\x0e\x32\x12.foliv.AddressType\x12\x0f\n\x07\x61\x64\x64ress\x18\x04 \x01(\x0c\x12\x0c\n\x04port\x18\x05 \x01(\r\x12\x12\n\nsourceName\x18\x06 \x01(\t\x12\x12\n\nrouterName\x18\x07 \x01(\t\x12\x13\n\x0bprocessName\x18\x08 \x01(\t\x12\x15\n\rxForwardedFor\x18\t \x03(\x0c\x12\x0f\n\x07isTouch\x18\n \x01(\x08\x12\r\n\x05muxID\x18\x0b \x01(\r\x12\x10\n\x08platform\x18\x0c \x01(\t\x12\x11\n\trequestID\x18\r \x01(\x0c\x12\x13\n\x0brouterLevel\x18\x0e \x01(\r\x12\x11\n\tuserAgent\x18\x0f \x01(\t\x12\x12\n\nrequestHop\x18\x10 \x01(\r\x12\r\n\x05\x61ppID\x18\x11 \x01(\r\x12\x0e\n\x06peerID\x18\x12 \x01(\r\x12\x0f\n\x07version\x18\x13 \x01(\r\x12\x15\n\rroundtripTime\x18\x14 \x01(\x05\x12\x0f\n\x07\x62indIPs\x18\x15 \x03(\x0c*9\n\x07\x43ommand\x12\t\n\x05\x45mpty\x10\x00\x12\x0b\n\x07\x43onnect\x10\x01\x12\r\n\tAssociate\x10\x03\x12\x07\n\x03Mux\x10\x7f*B\n\x0b\x41\x64\x64ressType\x12\x0f\n\x0bInvalidType\x10\x00\x12\x08\n\x04IPv4\x10\x01\x12\x0e\n\nDomainName\x10\x03\x12\x08\n\x04IPv6\x10\x04\x42\tZ\x07./folivb\x06proto3'
)

_COMMAND = _descriptor.EnumDescriptor(
  name='Command',
  full_name='foliv.Command',
  filename=None,
  file=DESCRIPTOR,
  create_key=_descriptor._internal_create_key,
  values=[
    _descriptor.EnumValueDescriptor(
      name='Empty', index=0, number=0,
      serialized_options=None,
      type=None,
      create_key=_descriptor._internal_create_key),
    _descriptor.EnumValueDescriptor(
      name='Connect', index=1, number=1,
      serialized_options=None,
      type=None,
      create_key=_descriptor._internal_create_key),
    _descriptor.EnumValueDescriptor(
      name='Associate', index=2, number=3,
      serialized_options=None,
      type=None,
      create_key=_descriptor._internal_create_key),
    _descriptor.EnumValueDescriptor(
      name='Mux', index=3, number=127,
      serialized_options=None,
      type=None,
      create_key=_descriptor._internal_create_key),
  ],
  containing_type=None,
  serialized_options=None,
  serialized_start=456,
  serialized_end=513,
)
_sym_db.RegisterEnumDescriptor(_COMMAND)

Command = enum_type_wrapper.EnumTypeWrapper(_COMMAND)
_ADDRESSTYPE = _descriptor.EnumDescriptor(
  name='AddressType',
  full_name='foliv.AddressType',
  filename=None,
  file=DESCRIPTOR,
  create_key=_descriptor._internal_create_key,
  values=[
    _descriptor.EnumValueDescriptor(
      name='InvalidType', index=0, number=0,
      serialized_options=None,
      type=None,
      create_key=_descriptor._internal_create_key),
    _descriptor.EnumValueDescriptor(
      name='IPv4', index=1, number=1,
      serialized_options=None,
      type=None,
      create_key=_descriptor._internal_create_key),
    _descriptor.EnumValueDescriptor(
      name='DomainName', index=2, number=3,
      serialized_options=None,
      type=None,
      create_key=_descriptor._internal_create_key),
    _descriptor.EnumValueDescriptor(
      name='IPv6', index=3, number=4,
      serialized_options=None,
      type=None,
      create_key=_descriptor._internal_create_key),
  ],
  containing_type=None,
  serialized_options=None,
  serialized_start=515,
  serialized_end=581,
)
_sym_db.RegisterEnumDescriptor(_ADDRESSTYPE)

AddressType = enum_type_wrapper.EnumTypeWrapper(_ADDRESSTYPE)
Empty = 0
Connect = 1
Associate = 3
Mux = 127
InvalidType = 0
IPv4 = 1
DomainName = 3
IPv6 = 4



_FOLIV = _descriptor.Descriptor(
  name='Foliv',
  full_name='foliv.Foliv',
  filename=None,
  file=DESCRIPTOR,
  containing_type=None,
  create_key=_descriptor._internal_create_key,
  fields=[
    _descriptor.FieldDescriptor(
      name='userHash', full_name='foliv.Foliv.userHash', index=0,
      number=1, type=9, cpp_type=9, label=1,
      has_default_value=False, default_value=b"".decode('utf-8'),
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      serialized_options=None, file=DESCRIPTOR,  create_key=_descriptor._internal_create_key),
    _descriptor.FieldDescriptor(
      name='command', full_name='foliv.Foliv.command', index=1,
      number=2, type=14, cpp_type=8, label=1,
      has_default_value=False, default_value=0,
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      serialized_options=None, file=DESCRIPTOR,  create_key=_descriptor._internal_create_key),
    _descriptor.FieldDescriptor(
      name='addressType', full_name='foliv.Foliv.addressType', index=2,
      number=3, type=14, cpp_type=8, label=1,
      has_default_value=False, default_value=0,
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      serialized_options=None, file=DESCRIPTOR,  create_key=_descriptor._internal_create_key),
    _descriptor.FieldDescriptor(
      name='address', full_name='foliv.Foliv.address', index=3,
      number=4, type=12, cpp_type=9, label=1,
      has_default_value=False, default_value=b"",
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      serialized_options=None, file=DESCRIPTOR,  create_key=_descriptor._internal_create_key),
    _descriptor.FieldDescriptor(
      name='port', full_name='foliv.Foliv.port', index=4,
      number=5, type=13, cpp_type=3, label=1,
      has_default_value=False, default_value=0,
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      serialized_options=None, file=DESCRIPTOR,  create_key=_descriptor._internal_create_key),
    _descriptor.FieldDescriptor(
      name='sourceName', full_name='foliv.Foliv.sourceName', index=5,
      number=6, type=9, cpp_type=9, label=1,
      has_default_value=False, default_value=b"".decode('utf-8'),
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      serialized_options=None, file=DESCRIPTOR,  create_key=_descriptor._internal_create_key),
    _descriptor.FieldDescriptor(
      name='routerName', full_name='foliv.Foliv.routerName', index=6,
      number=7, type=9, cpp_type=9, label=1,
      has_default_value=False, default_value=b"".decode('utf-8'),
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      serialized_options=None, file=DESCRIPTOR,  create_key=_descriptor._internal_create_key),
    _descriptor.FieldDescriptor(
      name='processName', full_name='foliv.Foliv.processName', index=7,
      number=8, type=9, cpp_type=9, label=1,
      has_default_value=False, default_value=b"".decode('utf-8'),
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      serialized_options=None, file=DESCRIPTOR,  create_key=_descriptor._internal_create_key),
    _descriptor.FieldDescriptor(
      name='xForwardedFor', full_name='foliv.Foliv.xForwardedFor', index=8,
      number=9, type=12, cpp_type=9, label=3,
      has_default_value=False, default_value=[],
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      serialized_options=None, file=DESCRIPTOR,  create_key=_descriptor._internal_create_key),
    _descriptor.FieldDescriptor(
      name='isTouch', full_name='foliv.Foliv.isTouch', index=9,
      number=10, type=8, cpp_type=7, label=1,
      has_default_value=False, default_value=False,
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      serialized_options=None, file=DESCRIPTOR,  create_key=_descriptor._internal_create_key),
    _descriptor.FieldDescriptor(
      name='muxID', full_name='foliv.Foliv.muxID', index=10,
      number=11, type=13, cpp_type=3, label=1,
      has_default_value=False, default_value=0,
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      serialized_options=None, file=DESCRIPTOR,  create_key=_descriptor._internal_create_key),
    _descriptor.FieldDescriptor(
      name='platform', full_name='foliv.Foliv.platform', index=11,
      number=12, type=9, cpp_type=9, label=1,
      has_default_value=False, default_value=b"".decode('utf-8'),
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      serialized_options=None, file=DESCRIPTOR,  create_key=_descriptor._internal_create_key),
    _descriptor.FieldDescriptor(
      name='requestID', full_name='foliv.Foliv.requestID', index=12,
      number=13, type=12, cpp_type=9, label=1,
      has_default_value=False, default_value=b"",
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      serialized_options=None, file=DESCRIPTOR,  create_key=_descriptor._internal_create_key),
    _descriptor.FieldDescriptor(
      name='routerLevel', full_name='foliv.Foliv.routerLevel', index=13,
      number=14, type=13, cpp_type=3, label=1,
      has_default_value=False, default_value=0,
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      serialized_options=None, file=DESCRIPTOR,  create_key=_descriptor._internal_create_key),
    _descriptor.FieldDescriptor(
      name='userAgent', full_name='foliv.Foliv.userAgent', index=14,
      number=15, type=9, cpp_type=9, label=1,
      has_default_value=False, default_value=b"".decode('utf-8'),
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      serialized_options=None, file=DESCRIPTOR,  create_key=_descriptor._internal_create_key),
    _descriptor.FieldDescriptor(
      name='requestHop', full_name='foliv.Foliv.requestHop', index=15,
      number=16, type=13, cpp_type=3, label=1,
      has_default_value=False, default_value=0,
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      serialized_options=None, file=DESCRIPTOR,  create_key=_descriptor._internal_create_key),
    _descriptor.FieldDescriptor(
      name='appID', full_name='foliv.Foliv.appID', index=16,
      number=17, type=13, cpp_type=3, label=1,
      has_default_value=False, default_value=0,
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      serialized_options=None, file=DESCRIPTOR,  create_key=_descriptor._internal_create_key),
    _descriptor.FieldDescriptor(
      name='peerID', full_name='foliv.Foliv.peerID', index=17,
      number=18, type=13, cpp_type=3, label=1,
      has_default_value=False, default_value=0,
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      serialized_options=None, file=DESCRIPTOR,  create_key=_descriptor._internal_create_key),
    _descriptor.FieldDescriptor(
      name='version', full_name='foliv.Foliv.version', index=18,
      number=19, type=13, cpp_type=3, label=1,
      has_default_value=False, default_value=0,
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      serialized_options=None, file=DESCRIPTOR,  create_key=_descriptor._internal_create_key),
    _descriptor.FieldDescriptor(
      name='roundtripTime', full_name='foliv.Foliv.roundtripTime', index=19,
      number=20, type=5, cpp_type=1, label=1,
      has_default_value=False, default_value=0,
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      serialized_options=None, file=DESCRIPTOR,  create_key=_descriptor._internal_create_key),
    _descriptor.FieldDescriptor(
      name='bindIPs', full_name='foliv.Foliv.bindIPs', index=20,
      number=21, type=12, cpp_type=9, label=3,
      has_default_value=False, default_value=[],
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      serialized_options=None, file=DESCRIPTOR,  create_key=_descriptor._internal_create_key),
  ],
  extensions=[
  ],
  nested_types=[],
  enum_types=[
  ],
  serialized_options=None,
  is_extendable=False,
  syntax='proto3',
  extension_ranges=[],
  oneofs=[
  ],
  serialized_start=23,
  serialized_end=454,
)

_FOLIV.fields_by_name['command'].enum_type = _COMMAND
_FOLIV.fields_by_name['addressType'].enum_type = _ADDRESSTYPE
DESCRIPTOR.message_types_by_name['Foliv'] = _FOLIV
DESCRIPTOR.enum_types_by_name['Command'] = _COMMAND
DESCRIPTOR.enum_types_by_name['AddressType'] = _ADDRESSTYPE
_sym_db.RegisterFileDescriptor(DESCRIPTOR)

Foliv = _reflection.GeneratedProtocolMessageType('Foliv', (_message.Message,), {
  'DESCRIPTOR' : _FOLIV,
  '__module__' : 'foliv_pb2'
  # @@protoc_insertion_point(class_scope:foliv.Foliv)
  })
_sym_db.RegisterMessage(Foliv)


DESCRIPTOR._options = None
# @@protoc_insertion_point(module_scope)
