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
  serialized_pb=b'\n\x0b\x66oliv.proto\x12\x05\x66oliv\"\x9c\x01\n\x05\x46oliv\x12\x10\n\x08userHash\x18\x01 \x01(\t\x12\x1f\n\x07\x63ommand\x18\x02 \x01(\x0e\x32\x0e.foliv.Command\x12\'\n\x0b\x61\x64\x64ressType\x18\x03 \x01(\x0e\x32\x12.foliv.AddressType\x12\x0f\n\x07\x61\x64\x64ress\x18\x04 \x01(\t\x12\x12\n\nsourceName\x18\x05 \x01(\t\x12\x12\n\nrouterName\x18\x06 \x01(\t*9\n\x07\x43ommand\x12\t\n\x05\x45mpty\x10\x00\x12\x0b\n\x07\x43onnect\x10\x01\x12\r\n\tAssociate\x10\x03\x12\x07\n\x03Mux\x10\x7f*B\n\x0b\x41\x64\x64ressType\x12\x0f\n\x0bInvalidType\x10\x00\x12\x08\n\x04IPv4\x10\x01\x12\x0e\n\nDomainName\x10\x03\x12\x08\n\x04IPv6\x10\x04\x42\tZ\x07./folivb\x06proto3'
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
  serialized_start=181,
  serialized_end=238,
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
  serialized_start=240,
  serialized_end=306,
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
      number=4, type=9, cpp_type=9, label=1,
      has_default_value=False, default_value=b"".decode('utf-8'),
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      serialized_options=None, file=DESCRIPTOR,  create_key=_descriptor._internal_create_key),
    _descriptor.FieldDescriptor(
      name='sourceName', full_name='foliv.Foliv.sourceName', index=4,
      number=5, type=9, cpp_type=9, label=1,
      has_default_value=False, default_value=b"".decode('utf-8'),
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      serialized_options=None, file=DESCRIPTOR,  create_key=_descriptor._internal_create_key),
    _descriptor.FieldDescriptor(
      name='routerName', full_name='foliv.Foliv.routerName', index=5,
      number=6, type=9, cpp_type=9, label=1,
      has_default_value=False, default_value=b"".decode('utf-8'),
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
  serialized_end=179,
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