// Generated by the protocol buffer compiler.  DO NOT EDIT!
// source: foliv.proto

#include "foliv.pb.h"

#include <algorithm>

#include <google/protobuf/io/coded_stream.h>
#include <google/protobuf/extension_set.h>
#include <google/protobuf/wire_format_lite.h>
#include <google/protobuf/descriptor.h>
#include <google/protobuf/generated_message_reflection.h>
#include <google/protobuf/reflection_ops.h>
#include <google/protobuf/wire_format.h>
// @@protoc_insertion_point(includes)
#include <google/protobuf/port_def.inc>

PROTOBUF_PRAGMA_INIT_SEG
namespace foliv {
constexpr Foliv::Foliv(
  ::PROTOBUF_NAMESPACE_ID::internal::ConstantInitialized)
  : xforwardedfor_()
  , userhash_(&::PROTOBUF_NAMESPACE_ID::internal::fixed_address_empty_string)
  , address_(&::PROTOBUF_NAMESPACE_ID::internal::fixed_address_empty_string)
  , sourcename_(&::PROTOBUF_NAMESPACE_ID::internal::fixed_address_empty_string)
  , routername_(&::PROTOBUF_NAMESPACE_ID::internal::fixed_address_empty_string)
  , processname_(&::PROTOBUF_NAMESPACE_ID::internal::fixed_address_empty_string)
  , command_(0)

  , addresstype_(0)

  , port_(0u)
  , istouch_(false)
  , muxid_(0u){}
struct FolivDefaultTypeInternal {
  constexpr FolivDefaultTypeInternal()
    : _instance(::PROTOBUF_NAMESPACE_ID::internal::ConstantInitialized{}) {}
  ~FolivDefaultTypeInternal() {}
  union {
    Foliv _instance;
  };
};
PROTOBUF_ATTRIBUTE_NO_DESTROY PROTOBUF_CONSTINIT FolivDefaultTypeInternal _Foliv_default_instance_;
}  // namespace foliv
static ::PROTOBUF_NAMESPACE_ID::Metadata file_level_metadata_foliv_2eproto[1];
static const ::PROTOBUF_NAMESPACE_ID::EnumDescriptor* file_level_enum_descriptors_foliv_2eproto[2];
static constexpr ::PROTOBUF_NAMESPACE_ID::ServiceDescriptor const** file_level_service_descriptors_foliv_2eproto = nullptr;

const ::PROTOBUF_NAMESPACE_ID::uint32 TableStruct_foliv_2eproto::offsets[] PROTOBUF_SECTION_VARIABLE(protodesc_cold) = {
  ~0u,  // no _has_bits_
  PROTOBUF_FIELD_OFFSET(::foliv::Foliv, _internal_metadata_),
  ~0u,  // no _extensions_
  ~0u,  // no _oneof_case_
  ~0u,  // no _weak_field_map_
  PROTOBUF_FIELD_OFFSET(::foliv::Foliv, userhash_),
  PROTOBUF_FIELD_OFFSET(::foliv::Foliv, command_),
  PROTOBUF_FIELD_OFFSET(::foliv::Foliv, addresstype_),
  PROTOBUF_FIELD_OFFSET(::foliv::Foliv, address_),
  PROTOBUF_FIELD_OFFSET(::foliv::Foliv, port_),
  PROTOBUF_FIELD_OFFSET(::foliv::Foliv, sourcename_),
  PROTOBUF_FIELD_OFFSET(::foliv::Foliv, routername_),
  PROTOBUF_FIELD_OFFSET(::foliv::Foliv, processname_),
  PROTOBUF_FIELD_OFFSET(::foliv::Foliv, xforwardedfor_),
  PROTOBUF_FIELD_OFFSET(::foliv::Foliv, istouch_),
  PROTOBUF_FIELD_OFFSET(::foliv::Foliv, muxid_),
};
static const ::PROTOBUF_NAMESPACE_ID::internal::MigrationSchema schemas[] PROTOBUF_SECTION_VARIABLE(protodesc_cold) = {
  { 0, -1, sizeof(::foliv::Foliv)},
};

static ::PROTOBUF_NAMESPACE_ID::Message const * const file_default_instances[] = {
  reinterpret_cast<const ::PROTOBUF_NAMESPACE_ID::Message*>(&::foliv::_Foliv_default_instance_),
};

const char descriptor_table_protodef_foliv_2eproto[] PROTOBUF_SECTION_VARIABLE(protodesc_cold) =
  "\n\013foliv.proto\022\005foliv\"\366\001\n\005Foliv\022\020\n\010userHa"
  "sh\030\001 \001(\t\022\037\n\007command\030\002 \001(\0162\016.foliv.Comman"
  "d\022\'\n\013addressType\030\003 \001(\0162\022.foliv.AddressTy"
  "pe\022\017\n\007address\030\004 \001(\014\022\014\n\004port\030\005 \001(\r\022\022\n\nsou"
  "rceName\030\006 \001(\t\022\022\n\nrouterName\030\007 \001(\t\022\023\n\013pro"
  "cessName\030\010 \001(\t\022\025\n\rxForwardedFor\030\t \003(\014\022\017\n"
  "\007isTouch\030\n \001(\010\022\r\n\005muxID\030\013 \001(\r*9\n\007Command"
  "\022\t\n\005Empty\020\000\022\013\n\007Connect\020\001\022\r\n\tAssociate\020\003\022"
  "\007\n\003Mux\020\177*B\n\013AddressType\022\017\n\013InvalidType\020\000"
  "\022\010\n\004IPv4\020\001\022\016\n\nDomainName\020\003\022\010\n\004IPv6\020\004B\tZ\007"
  "./folivb\006proto3"
  ;
static ::PROTOBUF_NAMESPACE_ID::internal::once_flag descriptor_table_foliv_2eproto_once;
const ::PROTOBUF_NAMESPACE_ID::internal::DescriptorTable descriptor_table_foliv_2eproto = {
  false, false, 415, descriptor_table_protodef_foliv_2eproto, "foliv.proto", 
  &descriptor_table_foliv_2eproto_once, nullptr, 0, 1,
  schemas, file_default_instances, TableStruct_foliv_2eproto::offsets,
  file_level_metadata_foliv_2eproto, file_level_enum_descriptors_foliv_2eproto, file_level_service_descriptors_foliv_2eproto,
};
PROTOBUF_ATTRIBUTE_WEAK ::PROTOBUF_NAMESPACE_ID::Metadata
descriptor_table_foliv_2eproto_metadata_getter(int index) {
  ::PROTOBUF_NAMESPACE_ID::internal::AssignDescriptors(&descriptor_table_foliv_2eproto);
  return descriptor_table_foliv_2eproto.file_level_metadata[index];
}

// Force running AddDescriptors() at dynamic initialization time.
PROTOBUF_ATTRIBUTE_INIT_PRIORITY static ::PROTOBUF_NAMESPACE_ID::internal::AddDescriptorsRunner dynamic_init_dummy_foliv_2eproto(&descriptor_table_foliv_2eproto);
namespace foliv {
const ::PROTOBUF_NAMESPACE_ID::EnumDescriptor* Command_descriptor() {
  ::PROTOBUF_NAMESPACE_ID::internal::AssignDescriptors(&descriptor_table_foliv_2eproto);
  return file_level_enum_descriptors_foliv_2eproto[0];
}
bool Command_IsValid(int value) {
  switch (value) {
    case 0:
    case 1:
    case 3:
    case 127:
      return true;
    default:
      return false;
  }
}

const ::PROTOBUF_NAMESPACE_ID::EnumDescriptor* AddressType_descriptor() {
  ::PROTOBUF_NAMESPACE_ID::internal::AssignDescriptors(&descriptor_table_foliv_2eproto);
  return file_level_enum_descriptors_foliv_2eproto[1];
}
bool AddressType_IsValid(int value) {
  switch (value) {
    case 0:
    case 1:
    case 3:
    case 4:
      return true;
    default:
      return false;
  }
}


// ===================================================================

class Foliv::_Internal {
 public:
};

Foliv::Foliv(::PROTOBUF_NAMESPACE_ID::Arena* arena)
  : ::PROTOBUF_NAMESPACE_ID::Message(arena),
  xforwardedfor_(arena) {
  SharedCtor();
  RegisterArenaDtor(arena);
  // @@protoc_insertion_point(arena_constructor:foliv.Foliv)
}
Foliv::Foliv(const Foliv& from)
  : ::PROTOBUF_NAMESPACE_ID::Message(),
      xforwardedfor_(from.xforwardedfor_) {
  _internal_metadata_.MergeFrom<::PROTOBUF_NAMESPACE_ID::UnknownFieldSet>(from._internal_metadata_);
  userhash_.UnsafeSetDefault(&::PROTOBUF_NAMESPACE_ID::internal::GetEmptyStringAlreadyInited());
  if (!from._internal_userhash().empty()) {
    userhash_.Set(::PROTOBUF_NAMESPACE_ID::internal::ArenaStringPtr::EmptyDefault{}, from._internal_userhash(), 
      GetArena());
  }
  address_.UnsafeSetDefault(&::PROTOBUF_NAMESPACE_ID::internal::GetEmptyStringAlreadyInited());
  if (!from._internal_address().empty()) {
    address_.Set(::PROTOBUF_NAMESPACE_ID::internal::ArenaStringPtr::EmptyDefault{}, from._internal_address(), 
      GetArena());
  }
  sourcename_.UnsafeSetDefault(&::PROTOBUF_NAMESPACE_ID::internal::GetEmptyStringAlreadyInited());
  if (!from._internal_sourcename().empty()) {
    sourcename_.Set(::PROTOBUF_NAMESPACE_ID::internal::ArenaStringPtr::EmptyDefault{}, from._internal_sourcename(), 
      GetArena());
  }
  routername_.UnsafeSetDefault(&::PROTOBUF_NAMESPACE_ID::internal::GetEmptyStringAlreadyInited());
  if (!from._internal_routername().empty()) {
    routername_.Set(::PROTOBUF_NAMESPACE_ID::internal::ArenaStringPtr::EmptyDefault{}, from._internal_routername(), 
      GetArena());
  }
  processname_.UnsafeSetDefault(&::PROTOBUF_NAMESPACE_ID::internal::GetEmptyStringAlreadyInited());
  if (!from._internal_processname().empty()) {
    processname_.Set(::PROTOBUF_NAMESPACE_ID::internal::ArenaStringPtr::EmptyDefault{}, from._internal_processname(), 
      GetArena());
  }
  ::memcpy(&command_, &from.command_,
    static_cast<size_t>(reinterpret_cast<char*>(&muxid_) -
    reinterpret_cast<char*>(&command_)) + sizeof(muxid_));
  // @@protoc_insertion_point(copy_constructor:foliv.Foliv)
}

void Foliv::SharedCtor() {
userhash_.UnsafeSetDefault(&::PROTOBUF_NAMESPACE_ID::internal::GetEmptyStringAlreadyInited());
address_.UnsafeSetDefault(&::PROTOBUF_NAMESPACE_ID::internal::GetEmptyStringAlreadyInited());
sourcename_.UnsafeSetDefault(&::PROTOBUF_NAMESPACE_ID::internal::GetEmptyStringAlreadyInited());
routername_.UnsafeSetDefault(&::PROTOBUF_NAMESPACE_ID::internal::GetEmptyStringAlreadyInited());
processname_.UnsafeSetDefault(&::PROTOBUF_NAMESPACE_ID::internal::GetEmptyStringAlreadyInited());
::memset(reinterpret_cast<char*>(this) + static_cast<size_t>(
    reinterpret_cast<char*>(&command_) - reinterpret_cast<char*>(this)),
    0, static_cast<size_t>(reinterpret_cast<char*>(&muxid_) -
    reinterpret_cast<char*>(&command_)) + sizeof(muxid_));
}

Foliv::~Foliv() {
  // @@protoc_insertion_point(destructor:foliv.Foliv)
  SharedDtor();
  _internal_metadata_.Delete<::PROTOBUF_NAMESPACE_ID::UnknownFieldSet>();
}

void Foliv::SharedDtor() {
  GOOGLE_DCHECK(GetArena() == nullptr);
  userhash_.DestroyNoArena(&::PROTOBUF_NAMESPACE_ID::internal::GetEmptyStringAlreadyInited());
  address_.DestroyNoArena(&::PROTOBUF_NAMESPACE_ID::internal::GetEmptyStringAlreadyInited());
  sourcename_.DestroyNoArena(&::PROTOBUF_NAMESPACE_ID::internal::GetEmptyStringAlreadyInited());
  routername_.DestroyNoArena(&::PROTOBUF_NAMESPACE_ID::internal::GetEmptyStringAlreadyInited());
  processname_.DestroyNoArena(&::PROTOBUF_NAMESPACE_ID::internal::GetEmptyStringAlreadyInited());
}

void Foliv::ArenaDtor(void* object) {
  Foliv* _this = reinterpret_cast< Foliv* >(object);
  (void)_this;
}
void Foliv::RegisterArenaDtor(::PROTOBUF_NAMESPACE_ID::Arena*) {
}
void Foliv::SetCachedSize(int size) const {
  _cached_size_.Set(size);
}

void Foliv::Clear() {
// @@protoc_insertion_point(message_clear_start:foliv.Foliv)
  ::PROTOBUF_NAMESPACE_ID::uint32 cached_has_bits = 0;
  // Prevent compiler warnings about cached_has_bits being unused
  (void) cached_has_bits;

  xforwardedfor_.Clear();
  userhash_.ClearToEmpty();
  address_.ClearToEmpty();
  sourcename_.ClearToEmpty();
  routername_.ClearToEmpty();
  processname_.ClearToEmpty();
  ::memset(&command_, 0, static_cast<size_t>(
      reinterpret_cast<char*>(&muxid_) -
      reinterpret_cast<char*>(&command_)) + sizeof(muxid_));
  _internal_metadata_.Clear<::PROTOBUF_NAMESPACE_ID::UnknownFieldSet>();
}

const char* Foliv::_InternalParse(const char* ptr, ::PROTOBUF_NAMESPACE_ID::internal::ParseContext* ctx) {
#define CHK_(x) if (PROTOBUF_PREDICT_FALSE(!(x))) goto failure
  while (!ctx->Done(&ptr)) {
    ::PROTOBUF_NAMESPACE_ID::uint32 tag;
    ptr = ::PROTOBUF_NAMESPACE_ID::internal::ReadTag(ptr, &tag);
    CHK_(ptr);
    switch (tag >> 3) {
      // string userHash = 1;
      case 1:
        if (PROTOBUF_PREDICT_TRUE(static_cast<::PROTOBUF_NAMESPACE_ID::uint8>(tag) == 10)) {
          auto str = _internal_mutable_userhash();
          ptr = ::PROTOBUF_NAMESPACE_ID::internal::InlineGreedyStringParser(str, ptr, ctx);
          CHK_(::PROTOBUF_NAMESPACE_ID::internal::VerifyUTF8(str, "foliv.Foliv.userHash"));
          CHK_(ptr);
        } else goto handle_unusual;
        continue;
      // .foliv.Command command = 2;
      case 2:
        if (PROTOBUF_PREDICT_TRUE(static_cast<::PROTOBUF_NAMESPACE_ID::uint8>(tag) == 16)) {
          ::PROTOBUF_NAMESPACE_ID::uint64 val = ::PROTOBUF_NAMESPACE_ID::internal::ReadVarint64(&ptr);
          CHK_(ptr);
          _internal_set_command(static_cast<::foliv::Command>(val));
        } else goto handle_unusual;
        continue;
      // .foliv.AddressType addressType = 3;
      case 3:
        if (PROTOBUF_PREDICT_TRUE(static_cast<::PROTOBUF_NAMESPACE_ID::uint8>(tag) == 24)) {
          ::PROTOBUF_NAMESPACE_ID::uint64 val = ::PROTOBUF_NAMESPACE_ID::internal::ReadVarint64(&ptr);
          CHK_(ptr);
          _internal_set_addresstype(static_cast<::foliv::AddressType>(val));
        } else goto handle_unusual;
        continue;
      // bytes address = 4;
      case 4:
        if (PROTOBUF_PREDICT_TRUE(static_cast<::PROTOBUF_NAMESPACE_ID::uint8>(tag) == 34)) {
          auto str = _internal_mutable_address();
          ptr = ::PROTOBUF_NAMESPACE_ID::internal::InlineGreedyStringParser(str, ptr, ctx);
          CHK_(ptr);
        } else goto handle_unusual;
        continue;
      // uint32 port = 5;
      case 5:
        if (PROTOBUF_PREDICT_TRUE(static_cast<::PROTOBUF_NAMESPACE_ID::uint8>(tag) == 40)) {
          port_ = ::PROTOBUF_NAMESPACE_ID::internal::ReadVarint32(&ptr);
          CHK_(ptr);
        } else goto handle_unusual;
        continue;
      // string sourceName = 6;
      case 6:
        if (PROTOBUF_PREDICT_TRUE(static_cast<::PROTOBUF_NAMESPACE_ID::uint8>(tag) == 50)) {
          auto str = _internal_mutable_sourcename();
          ptr = ::PROTOBUF_NAMESPACE_ID::internal::InlineGreedyStringParser(str, ptr, ctx);
          CHK_(::PROTOBUF_NAMESPACE_ID::internal::VerifyUTF8(str, "foliv.Foliv.sourceName"));
          CHK_(ptr);
        } else goto handle_unusual;
        continue;
      // string routerName = 7;
      case 7:
        if (PROTOBUF_PREDICT_TRUE(static_cast<::PROTOBUF_NAMESPACE_ID::uint8>(tag) == 58)) {
          auto str = _internal_mutable_routername();
          ptr = ::PROTOBUF_NAMESPACE_ID::internal::InlineGreedyStringParser(str, ptr, ctx);
          CHK_(::PROTOBUF_NAMESPACE_ID::internal::VerifyUTF8(str, "foliv.Foliv.routerName"));
          CHK_(ptr);
        } else goto handle_unusual;
        continue;
      // string processName = 8;
      case 8:
        if (PROTOBUF_PREDICT_TRUE(static_cast<::PROTOBUF_NAMESPACE_ID::uint8>(tag) == 66)) {
          auto str = _internal_mutable_processname();
          ptr = ::PROTOBUF_NAMESPACE_ID::internal::InlineGreedyStringParser(str, ptr, ctx);
          CHK_(::PROTOBUF_NAMESPACE_ID::internal::VerifyUTF8(str, "foliv.Foliv.processName"));
          CHK_(ptr);
        } else goto handle_unusual;
        continue;
      // repeated bytes xForwardedFor = 9;
      case 9:
        if (PROTOBUF_PREDICT_TRUE(static_cast<::PROTOBUF_NAMESPACE_ID::uint8>(tag) == 74)) {
          ptr -= 1;
          do {
            ptr += 1;
            auto str = _internal_add_xforwardedfor();
            ptr = ::PROTOBUF_NAMESPACE_ID::internal::InlineGreedyStringParser(str, ptr, ctx);
            CHK_(ptr);
            if (!ctx->DataAvailable(ptr)) break;
          } while (::PROTOBUF_NAMESPACE_ID::internal::ExpectTag<74>(ptr));
        } else goto handle_unusual;
        continue;
      // bool isTouch = 10;
      case 10:
        if (PROTOBUF_PREDICT_TRUE(static_cast<::PROTOBUF_NAMESPACE_ID::uint8>(tag) == 80)) {
          istouch_ = ::PROTOBUF_NAMESPACE_ID::internal::ReadVarint64(&ptr);
          CHK_(ptr);
        } else goto handle_unusual;
        continue;
      // uint32 muxID = 11;
      case 11:
        if (PROTOBUF_PREDICT_TRUE(static_cast<::PROTOBUF_NAMESPACE_ID::uint8>(tag) == 88)) {
          muxid_ = ::PROTOBUF_NAMESPACE_ID::internal::ReadVarint32(&ptr);
          CHK_(ptr);
        } else goto handle_unusual;
        continue;
      default: {
      handle_unusual:
        if ((tag & 7) == 4 || tag == 0) {
          ctx->SetLastTag(tag);
          goto success;
        }
        ptr = UnknownFieldParse(tag,
            _internal_metadata_.mutable_unknown_fields<::PROTOBUF_NAMESPACE_ID::UnknownFieldSet>(),
            ptr, ctx);
        CHK_(ptr != nullptr);
        continue;
      }
    }  // switch
  }  // while
success:
  return ptr;
failure:
  ptr = nullptr;
  goto success;
#undef CHK_
}

::PROTOBUF_NAMESPACE_ID::uint8* Foliv::_InternalSerialize(
    ::PROTOBUF_NAMESPACE_ID::uint8* target, ::PROTOBUF_NAMESPACE_ID::io::EpsCopyOutputStream* stream) const {
  // @@protoc_insertion_point(serialize_to_array_start:foliv.Foliv)
  ::PROTOBUF_NAMESPACE_ID::uint32 cached_has_bits = 0;
  (void) cached_has_bits;

  // string userHash = 1;
  if (this->userhash().size() > 0) {
    ::PROTOBUF_NAMESPACE_ID::internal::WireFormatLite::VerifyUtf8String(
      this->_internal_userhash().data(), static_cast<int>(this->_internal_userhash().length()),
      ::PROTOBUF_NAMESPACE_ID::internal::WireFormatLite::SERIALIZE,
      "foliv.Foliv.userHash");
    target = stream->WriteStringMaybeAliased(
        1, this->_internal_userhash(), target);
  }

  // .foliv.Command command = 2;
  if (this->command() != 0) {
    target = stream->EnsureSpace(target);
    target = ::PROTOBUF_NAMESPACE_ID::internal::WireFormatLite::WriteEnumToArray(
      2, this->_internal_command(), target);
  }

  // .foliv.AddressType addressType = 3;
  if (this->addresstype() != 0) {
    target = stream->EnsureSpace(target);
    target = ::PROTOBUF_NAMESPACE_ID::internal::WireFormatLite::WriteEnumToArray(
      3, this->_internal_addresstype(), target);
  }

  // bytes address = 4;
  if (this->address().size() > 0) {
    target = stream->WriteBytesMaybeAliased(
        4, this->_internal_address(), target);
  }

  // uint32 port = 5;
  if (this->port() != 0) {
    target = stream->EnsureSpace(target);
    target = ::PROTOBUF_NAMESPACE_ID::internal::WireFormatLite::WriteUInt32ToArray(5, this->_internal_port(), target);
  }

  // string sourceName = 6;
  if (this->sourcename().size() > 0) {
    ::PROTOBUF_NAMESPACE_ID::internal::WireFormatLite::VerifyUtf8String(
      this->_internal_sourcename().data(), static_cast<int>(this->_internal_sourcename().length()),
      ::PROTOBUF_NAMESPACE_ID::internal::WireFormatLite::SERIALIZE,
      "foliv.Foliv.sourceName");
    target = stream->WriteStringMaybeAliased(
        6, this->_internal_sourcename(), target);
  }

  // string routerName = 7;
  if (this->routername().size() > 0) {
    ::PROTOBUF_NAMESPACE_ID::internal::WireFormatLite::VerifyUtf8String(
      this->_internal_routername().data(), static_cast<int>(this->_internal_routername().length()),
      ::PROTOBUF_NAMESPACE_ID::internal::WireFormatLite::SERIALIZE,
      "foliv.Foliv.routerName");
    target = stream->WriteStringMaybeAliased(
        7, this->_internal_routername(), target);
  }

  // string processName = 8;
  if (this->processname().size() > 0) {
    ::PROTOBUF_NAMESPACE_ID::internal::WireFormatLite::VerifyUtf8String(
      this->_internal_processname().data(), static_cast<int>(this->_internal_processname().length()),
      ::PROTOBUF_NAMESPACE_ID::internal::WireFormatLite::SERIALIZE,
      "foliv.Foliv.processName");
    target = stream->WriteStringMaybeAliased(
        8, this->_internal_processname(), target);
  }

  // repeated bytes xForwardedFor = 9;
  for (int i = 0, n = this->_internal_xforwardedfor_size(); i < n; i++) {
    const auto& s = this->_internal_xforwardedfor(i);
    target = stream->WriteBytes(9, s, target);
  }

  // bool isTouch = 10;
  if (this->istouch() != 0) {
    target = stream->EnsureSpace(target);
    target = ::PROTOBUF_NAMESPACE_ID::internal::WireFormatLite::WriteBoolToArray(10, this->_internal_istouch(), target);
  }

  // uint32 muxID = 11;
  if (this->muxid() != 0) {
    target = stream->EnsureSpace(target);
    target = ::PROTOBUF_NAMESPACE_ID::internal::WireFormatLite::WriteUInt32ToArray(11, this->_internal_muxid(), target);
  }

  if (PROTOBUF_PREDICT_FALSE(_internal_metadata_.have_unknown_fields())) {
    target = ::PROTOBUF_NAMESPACE_ID::internal::WireFormat::InternalSerializeUnknownFieldsToArray(
        _internal_metadata_.unknown_fields<::PROTOBUF_NAMESPACE_ID::UnknownFieldSet>(::PROTOBUF_NAMESPACE_ID::UnknownFieldSet::default_instance), target, stream);
  }
  // @@protoc_insertion_point(serialize_to_array_end:foliv.Foliv)
  return target;
}

size_t Foliv::ByteSizeLong() const {
// @@protoc_insertion_point(message_byte_size_start:foliv.Foliv)
  size_t total_size = 0;

  ::PROTOBUF_NAMESPACE_ID::uint32 cached_has_bits = 0;
  // Prevent compiler warnings about cached_has_bits being unused
  (void) cached_has_bits;

  // repeated bytes xForwardedFor = 9;
  total_size += 1 *
      ::PROTOBUF_NAMESPACE_ID::internal::FromIntSize(xforwardedfor_.size());
  for (int i = 0, n = xforwardedfor_.size(); i < n; i++) {
    total_size += ::PROTOBUF_NAMESPACE_ID::internal::WireFormatLite::BytesSize(
      xforwardedfor_.Get(i));
  }

  // string userHash = 1;
  if (this->userhash().size() > 0) {
    total_size += 1 +
      ::PROTOBUF_NAMESPACE_ID::internal::WireFormatLite::StringSize(
        this->_internal_userhash());
  }

  // bytes address = 4;
  if (this->address().size() > 0) {
    total_size += 1 +
      ::PROTOBUF_NAMESPACE_ID::internal::WireFormatLite::BytesSize(
        this->_internal_address());
  }

  // string sourceName = 6;
  if (this->sourcename().size() > 0) {
    total_size += 1 +
      ::PROTOBUF_NAMESPACE_ID::internal::WireFormatLite::StringSize(
        this->_internal_sourcename());
  }

  // string routerName = 7;
  if (this->routername().size() > 0) {
    total_size += 1 +
      ::PROTOBUF_NAMESPACE_ID::internal::WireFormatLite::StringSize(
        this->_internal_routername());
  }

  // string processName = 8;
  if (this->processname().size() > 0) {
    total_size += 1 +
      ::PROTOBUF_NAMESPACE_ID::internal::WireFormatLite::StringSize(
        this->_internal_processname());
  }

  // .foliv.Command command = 2;
  if (this->command() != 0) {
    total_size += 1 +
      ::PROTOBUF_NAMESPACE_ID::internal::WireFormatLite::EnumSize(this->_internal_command());
  }

  // .foliv.AddressType addressType = 3;
  if (this->addresstype() != 0) {
    total_size += 1 +
      ::PROTOBUF_NAMESPACE_ID::internal::WireFormatLite::EnumSize(this->_internal_addresstype());
  }

  // uint32 port = 5;
  if (this->port() != 0) {
    total_size += 1 +
      ::PROTOBUF_NAMESPACE_ID::internal::WireFormatLite::UInt32Size(
        this->_internal_port());
  }

  // bool isTouch = 10;
  if (this->istouch() != 0) {
    total_size += 1 + 1;
  }

  // uint32 muxID = 11;
  if (this->muxid() != 0) {
    total_size += 1 +
      ::PROTOBUF_NAMESPACE_ID::internal::WireFormatLite::UInt32Size(
        this->_internal_muxid());
  }

  if (PROTOBUF_PREDICT_FALSE(_internal_metadata_.have_unknown_fields())) {
    return ::PROTOBUF_NAMESPACE_ID::internal::ComputeUnknownFieldsSize(
        _internal_metadata_, total_size, &_cached_size_);
  }
  int cached_size = ::PROTOBUF_NAMESPACE_ID::internal::ToCachedSize(total_size);
  SetCachedSize(cached_size);
  return total_size;
}

void Foliv::MergeFrom(const ::PROTOBUF_NAMESPACE_ID::Message& from) {
// @@protoc_insertion_point(generalized_merge_from_start:foliv.Foliv)
  GOOGLE_DCHECK_NE(&from, this);
  const Foliv* source =
      ::PROTOBUF_NAMESPACE_ID::DynamicCastToGenerated<Foliv>(
          &from);
  if (source == nullptr) {
  // @@protoc_insertion_point(generalized_merge_from_cast_fail:foliv.Foliv)
    ::PROTOBUF_NAMESPACE_ID::internal::ReflectionOps::Merge(from, this);
  } else {
  // @@protoc_insertion_point(generalized_merge_from_cast_success:foliv.Foliv)
    MergeFrom(*source);
  }
}

void Foliv::MergeFrom(const Foliv& from) {
// @@protoc_insertion_point(class_specific_merge_from_start:foliv.Foliv)
  GOOGLE_DCHECK_NE(&from, this);
  _internal_metadata_.MergeFrom<::PROTOBUF_NAMESPACE_ID::UnknownFieldSet>(from._internal_metadata_);
  ::PROTOBUF_NAMESPACE_ID::uint32 cached_has_bits = 0;
  (void) cached_has_bits;

  xforwardedfor_.MergeFrom(from.xforwardedfor_);
  if (from.userhash().size() > 0) {
    _internal_set_userhash(from._internal_userhash());
  }
  if (from.address().size() > 0) {
    _internal_set_address(from._internal_address());
  }
  if (from.sourcename().size() > 0) {
    _internal_set_sourcename(from._internal_sourcename());
  }
  if (from.routername().size() > 0) {
    _internal_set_routername(from._internal_routername());
  }
  if (from.processname().size() > 0) {
    _internal_set_processname(from._internal_processname());
  }
  if (from.command() != 0) {
    _internal_set_command(from._internal_command());
  }
  if (from.addresstype() != 0) {
    _internal_set_addresstype(from._internal_addresstype());
  }
  if (from.port() != 0) {
    _internal_set_port(from._internal_port());
  }
  if (from.istouch() != 0) {
    _internal_set_istouch(from._internal_istouch());
  }
  if (from.muxid() != 0) {
    _internal_set_muxid(from._internal_muxid());
  }
}

void Foliv::CopyFrom(const ::PROTOBUF_NAMESPACE_ID::Message& from) {
// @@protoc_insertion_point(generalized_copy_from_start:foliv.Foliv)
  if (&from == this) return;
  Clear();
  MergeFrom(from);
}

void Foliv::CopyFrom(const Foliv& from) {
// @@protoc_insertion_point(class_specific_copy_from_start:foliv.Foliv)
  if (&from == this) return;
  Clear();
  MergeFrom(from);
}

bool Foliv::IsInitialized() const {
  return true;
}

void Foliv::InternalSwap(Foliv* other) {
  using std::swap;
  _internal_metadata_.Swap<::PROTOBUF_NAMESPACE_ID::UnknownFieldSet>(&other->_internal_metadata_);
  xforwardedfor_.InternalSwap(&other->xforwardedfor_);
  userhash_.Swap(&other->userhash_, &::PROTOBUF_NAMESPACE_ID::internal::GetEmptyStringAlreadyInited(), GetArena());
  address_.Swap(&other->address_, &::PROTOBUF_NAMESPACE_ID::internal::GetEmptyStringAlreadyInited(), GetArena());
  sourcename_.Swap(&other->sourcename_, &::PROTOBUF_NAMESPACE_ID::internal::GetEmptyStringAlreadyInited(), GetArena());
  routername_.Swap(&other->routername_, &::PROTOBUF_NAMESPACE_ID::internal::GetEmptyStringAlreadyInited(), GetArena());
  processname_.Swap(&other->processname_, &::PROTOBUF_NAMESPACE_ID::internal::GetEmptyStringAlreadyInited(), GetArena());
  ::PROTOBUF_NAMESPACE_ID::internal::memswap<
      PROTOBUF_FIELD_OFFSET(Foliv, muxid_)
      + sizeof(Foliv::muxid_)
      - PROTOBUF_FIELD_OFFSET(Foliv, command_)>(
          reinterpret_cast<char*>(&command_),
          reinterpret_cast<char*>(&other->command_));
}

::PROTOBUF_NAMESPACE_ID::Metadata Foliv::GetMetadata() const {
  return GetMetadataStatic();
}


// @@protoc_insertion_point(namespace_scope)
}  // namespace foliv
PROTOBUF_NAMESPACE_OPEN
template<> PROTOBUF_NOINLINE ::foliv::Foliv* Arena::CreateMaybeMessage< ::foliv::Foliv >(Arena* arena) {
  return Arena::CreateMessageInternal< ::foliv::Foliv >(arena);
}
PROTOBUF_NAMESPACE_CLOSE

// @@protoc_insertion_point(global_scope)
#include <google/protobuf/port_undef.inc>
