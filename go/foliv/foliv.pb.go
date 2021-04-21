// Code generated by protoc-gen-go. DO NOT EDIT.
// versions:
// 	protoc-gen-go v1.26.0-devel
// 	protoc        v3.15.8
// source: foliv.proto

package foliv

import (
	protoreflect "google.golang.org/protobuf/reflect/protoreflect"
	protoimpl "google.golang.org/protobuf/runtime/protoimpl"
	reflect "reflect"
	sync "sync"
)

const (
	// Verify that this generated code is sufficiently up-to-date.
	_ = protoimpl.EnforceVersion(20 - protoimpl.MinVersion)
	// Verify that runtime/protoimpl is sufficiently up-to-date.
	_ = protoimpl.EnforceVersion(protoimpl.MaxVersion - 20)
)

type Command int32

const (
	Command_Empty     Command = 0
	Command_Connect   Command = 1
	Command_Associate Command = 3
	Command_Mux       Command = 127
)

// Enum value maps for Command.
var (
	Command_name = map[int32]string{
		0:   "Empty",
		1:   "Connect",
		3:   "Associate",
		127: "Mux",
	}
	Command_value = map[string]int32{
		"Empty":     0,
		"Connect":   1,
		"Associate": 3,
		"Mux":       127,
	}
)

func (x Command) Enum() *Command {
	p := new(Command)
	*p = x
	return p
}

func (x Command) String() string {
	return protoimpl.X.EnumStringOf(x.Descriptor(), protoreflect.EnumNumber(x))
}

func (Command) Descriptor() protoreflect.EnumDescriptor {
	return file_foliv_proto_enumTypes[0].Descriptor()
}

func (Command) Type() protoreflect.EnumType {
	return &file_foliv_proto_enumTypes[0]
}

func (x Command) Number() protoreflect.EnumNumber {
	return protoreflect.EnumNumber(x)
}

// Deprecated: Use Command.Descriptor instead.
func (Command) EnumDescriptor() ([]byte, []int) {
	return file_foliv_proto_rawDescGZIP(), []int{0}
}

type AddressType int32

const (
	AddressType_InvalidType AddressType = 0
	AddressType_IPv4        AddressType = 1
	AddressType_DomainName  AddressType = 3
	AddressType_IPv6        AddressType = 4
)

// Enum value maps for AddressType.
var (
	AddressType_name = map[int32]string{
		0: "InvalidType",
		1: "IPv4",
		3: "DomainName",
		4: "IPv6",
	}
	AddressType_value = map[string]int32{
		"InvalidType": 0,
		"IPv4":        1,
		"DomainName":  3,
		"IPv6":        4,
	}
)

func (x AddressType) Enum() *AddressType {
	p := new(AddressType)
	*p = x
	return p
}

func (x AddressType) String() string {
	return protoimpl.X.EnumStringOf(x.Descriptor(), protoreflect.EnumNumber(x))
}

func (AddressType) Descriptor() protoreflect.EnumDescriptor {
	return file_foliv_proto_enumTypes[1].Descriptor()
}

func (AddressType) Type() protoreflect.EnumType {
	return &file_foliv_proto_enumTypes[1]
}

func (x AddressType) Number() protoreflect.EnumNumber {
	return protoreflect.EnumNumber(x)
}

// Deprecated: Use AddressType.Descriptor instead.
func (AddressType) EnumDescriptor() ([]byte, []int) {
	return file_foliv_proto_rawDescGZIP(), []int{1}
}

type Foliv struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	UserHash    string      `protobuf:"bytes,1,opt,name=userHash,proto3" json:"userHash,omitempty"`
	Command     Command     `protobuf:"varint,2,opt,name=command,proto3,enum=foliv.Command" json:"command,omitempty"`
	AddressType AddressType `protobuf:"varint,3,opt,name=addressType,proto3,enum=foliv.AddressType" json:"addressType,omitempty"`
	Address     string      `protobuf:"bytes,4,opt,name=address,proto3" json:"address,omitempty"`
	SourceName  string      `protobuf:"bytes,5,opt,name=sourceName,proto3" json:"sourceName,omitempty"`
	RouterName  string      `protobuf:"bytes,6,opt,name=routerName,proto3" json:"routerName,omitempty"`
}

func (x *Foliv) Reset() {
	*x = Foliv{}
	if protoimpl.UnsafeEnabled {
		mi := &file_foliv_proto_msgTypes[0]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *Foliv) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*Foliv) ProtoMessage() {}

func (x *Foliv) ProtoReflect() protoreflect.Message {
	mi := &file_foliv_proto_msgTypes[0]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use Foliv.ProtoReflect.Descriptor instead.
func (*Foliv) Descriptor() ([]byte, []int) {
	return file_foliv_proto_rawDescGZIP(), []int{0}
}

func (x *Foliv) GetUserHash() string {
	if x != nil {
		return x.UserHash
	}
	return ""
}

func (x *Foliv) GetCommand() Command {
	if x != nil {
		return x.Command
	}
	return Command_Empty
}

func (x *Foliv) GetAddressType() AddressType {
	if x != nil {
		return x.AddressType
	}
	return AddressType_InvalidType
}

func (x *Foliv) GetAddress() string {
	if x != nil {
		return x.Address
	}
	return ""
}

func (x *Foliv) GetSourceName() string {
	if x != nil {
		return x.SourceName
	}
	return ""
}

func (x *Foliv) GetRouterName() string {
	if x != nil {
		return x.RouterName
	}
	return ""
}

var File_foliv_proto protoreflect.FileDescriptor

var file_foliv_proto_rawDesc = []byte{
	0x0a, 0x0b, 0x66, 0x6f, 0x6c, 0x69, 0x76, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x05, 0x66,
	0x6f, 0x6c, 0x69, 0x76, 0x22, 0xdd, 0x01, 0x0a, 0x05, 0x46, 0x6f, 0x6c, 0x69, 0x76, 0x12, 0x1a,
	0x0a, 0x08, 0x75, 0x73, 0x65, 0x72, 0x48, 0x61, 0x73, 0x68, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09,
	0x52, 0x08, 0x75, 0x73, 0x65, 0x72, 0x48, 0x61, 0x73, 0x68, 0x12, 0x28, 0x0a, 0x07, 0x63, 0x6f,
	0x6d, 0x6d, 0x61, 0x6e, 0x64, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x0e, 0x2e, 0x66, 0x6f,
	0x6c, 0x69, 0x76, 0x2e, 0x43, 0x6f, 0x6d, 0x6d, 0x61, 0x6e, 0x64, 0x52, 0x07, 0x63, 0x6f, 0x6d,
	0x6d, 0x61, 0x6e, 0x64, 0x12, 0x34, 0x0a, 0x0b, 0x61, 0x64, 0x64, 0x72, 0x65, 0x73, 0x73, 0x54,
	0x79, 0x70, 0x65, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x12, 0x2e, 0x66, 0x6f, 0x6c, 0x69,
	0x76, 0x2e, 0x41, 0x64, 0x64, 0x72, 0x65, 0x73, 0x73, 0x54, 0x79, 0x70, 0x65, 0x52, 0x0b, 0x61,
	0x64, 0x64, 0x72, 0x65, 0x73, 0x73, 0x54, 0x79, 0x70, 0x65, 0x12, 0x18, 0x0a, 0x07, 0x61, 0x64,
	0x64, 0x72, 0x65, 0x73, 0x73, 0x18, 0x04, 0x20, 0x01, 0x28, 0x09, 0x52, 0x07, 0x61, 0x64, 0x64,
	0x72, 0x65, 0x73, 0x73, 0x12, 0x1e, 0x0a, 0x0a, 0x73, 0x6f, 0x75, 0x72, 0x63, 0x65, 0x4e, 0x61,
	0x6d, 0x65, 0x18, 0x05, 0x20, 0x01, 0x28, 0x09, 0x52, 0x0a, 0x73, 0x6f, 0x75, 0x72, 0x63, 0x65,
	0x4e, 0x61, 0x6d, 0x65, 0x12, 0x1e, 0x0a, 0x0a, 0x72, 0x6f, 0x75, 0x74, 0x65, 0x72, 0x4e, 0x61,
	0x6d, 0x65, 0x18, 0x06, 0x20, 0x01, 0x28, 0x09, 0x52, 0x0a, 0x72, 0x6f, 0x75, 0x74, 0x65, 0x72,
	0x4e, 0x61, 0x6d, 0x65, 0x2a, 0x39, 0x0a, 0x07, 0x43, 0x6f, 0x6d, 0x6d, 0x61, 0x6e, 0x64, 0x12,
	0x09, 0x0a, 0x05, 0x45, 0x6d, 0x70, 0x74, 0x79, 0x10, 0x00, 0x12, 0x0b, 0x0a, 0x07, 0x43, 0x6f,
	0x6e, 0x6e, 0x65, 0x63, 0x74, 0x10, 0x01, 0x12, 0x0d, 0x0a, 0x09, 0x41, 0x73, 0x73, 0x6f, 0x63,
	0x69, 0x61, 0x74, 0x65, 0x10, 0x03, 0x12, 0x07, 0x0a, 0x03, 0x4d, 0x75, 0x78, 0x10, 0x7f, 0x2a,
	0x42, 0x0a, 0x0b, 0x41, 0x64, 0x64, 0x72, 0x65, 0x73, 0x73, 0x54, 0x79, 0x70, 0x65, 0x12, 0x0f,
	0x0a, 0x0b, 0x49, 0x6e, 0x76, 0x61, 0x6c, 0x69, 0x64, 0x54, 0x79, 0x70, 0x65, 0x10, 0x00, 0x12,
	0x08, 0x0a, 0x04, 0x49, 0x50, 0x76, 0x34, 0x10, 0x01, 0x12, 0x0e, 0x0a, 0x0a, 0x44, 0x6f, 0x6d,
	0x61, 0x69, 0x6e, 0x4e, 0x61, 0x6d, 0x65, 0x10, 0x03, 0x12, 0x08, 0x0a, 0x04, 0x49, 0x50, 0x76,
	0x36, 0x10, 0x04, 0x42, 0x09, 0x5a, 0x07, 0x2e, 0x2f, 0x66, 0x6f, 0x6c, 0x69, 0x76, 0x62, 0x06,
	0x70, 0x72, 0x6f, 0x74, 0x6f, 0x33,
}

var (
	file_foliv_proto_rawDescOnce sync.Once
	file_foliv_proto_rawDescData = file_foliv_proto_rawDesc
)

func file_foliv_proto_rawDescGZIP() []byte {
	file_foliv_proto_rawDescOnce.Do(func() {
		file_foliv_proto_rawDescData = protoimpl.X.CompressGZIP(file_foliv_proto_rawDescData)
	})
	return file_foliv_proto_rawDescData
}

var file_foliv_proto_enumTypes = make([]protoimpl.EnumInfo, 2)
var file_foliv_proto_msgTypes = make([]protoimpl.MessageInfo, 1)
var file_foliv_proto_goTypes = []interface{}{
	(Command)(0),     // 0: foliv.Command
	(AddressType)(0), // 1: foliv.AddressType
	(*Foliv)(nil),    // 2: foliv.Foliv
}
var file_foliv_proto_depIdxs = []int32{
	0, // 0: foliv.Foliv.command:type_name -> foliv.Command
	1, // 1: foliv.Foliv.addressType:type_name -> foliv.AddressType
	2, // [2:2] is the sub-list for method output_type
	2, // [2:2] is the sub-list for method input_type
	2, // [2:2] is the sub-list for extension type_name
	2, // [2:2] is the sub-list for extension extendee
	0, // [0:2] is the sub-list for field type_name
}

func init() { file_foliv_proto_init() }
func file_foliv_proto_init() {
	if File_foliv_proto != nil {
		return
	}
	if !protoimpl.UnsafeEnabled {
		file_foliv_proto_msgTypes[0].Exporter = func(v interface{}, i int) interface{} {
			switch v := v.(*Foliv); i {
			case 0:
				return &v.state
			case 1:
				return &v.sizeCache
			case 2:
				return &v.unknownFields
			default:
				return nil
			}
		}
	}
	type x struct{}
	out := protoimpl.TypeBuilder{
		File: protoimpl.DescBuilder{
			GoPackagePath: reflect.TypeOf(x{}).PkgPath(),
			RawDescriptor: file_foliv_proto_rawDesc,
			NumEnums:      2,
			NumMessages:   1,
			NumExtensions: 0,
			NumServices:   0,
		},
		GoTypes:           file_foliv_proto_goTypes,
		DependencyIndexes: file_foliv_proto_depIdxs,
		EnumInfos:         file_foliv_proto_enumTypes,
		MessageInfos:      file_foliv_proto_msgTypes,
	}.Build()
	File_foliv_proto = out.File
	file_foliv_proto_rawDesc = nil
	file_foliv_proto_goTypes = nil
	file_foliv_proto_depIdxs = nil
}