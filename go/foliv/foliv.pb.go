// ?O$$O??77O7>:!!7$O$O>>7?O$?CC7>7C$$QO77!!>7777>>>!!!>!!!!!!>7>>>>>>>>>>>>>>
// ?O$OO??>?O7>:!>7$Q$OQOO?OO?CC?77O$Q$C77777?????77777>>>>>>>>>>>>>>>>>>>>>>>
// ?$$OO?7>?C>OQ$$OHHQC?$$QHHNHHO>?$$QO77>>7?????????7>>>!!!!!>>>>>>>>>>>>>>>>
// C$$OC?7>O$?!O$N$$QQNCCCC7?QN$Q?OOCOC>7!77??CCCC?C??7>>>>>>!>>>>>>>>>>>>>>>>
// OOOO??>7O7QNNNHQNNQHHHHHQ?7C7OH$QQO7>>!7???$$$$$$$$OCC???7>>>>>>>>>>>>>>>>>
// $OOO??>?CHQ7NNHHQ$HHC!>7$OOC!:CQOO$$7!!7???$$$$$$$$$O$OC?7>>>>>>>>>>>>>>>!!
// ??CC?7??C>7$NHQOQOOQCOOO?>!>?O>:7CCCQH>??7?COOOOO$$$$OCC77>>>>>>>>>>>>>!!>!
// ?C??C777H$CHH$O?OOQO?7COQ$Q$O!QO!>CC?OCCH???????????77>>>!!>>>!!>>>!!!>!>!!
// >>77?7?HQO7HQOO?O$$C>>COOO$$$$>Q?>7>?>7C??Q>>>>>>>7>>>>>>>>>>!>!!!!!!!!!!>!
// ?????!:>7>CH$$OCOOO?!!>!!:!7C$OCQO:>:7?C7?7QQ?????????????7>!!>>>>>>>>>!!!?
// >>>>>>$NQ>$$?77?OOO>:!:QNNNH->?$Q$?:7!!??7>>$$O>77777>>777>>>7!!!!!!!!!!!??
// >>>>7C7-:>O?7>>>7O$>::7;-?NHH-!?$QH7:C!>>C7!7OO$77>>>777>>>>>77>>>77>>77C?7
// >>>>OQ;;-CC?77>>!>CC:?!..:QQQO!!!$QO:>>>!>>>!7OC$C77>>>>>>7>>>>>>>>!!!!C777
// >>>7?CO>$CC7777>!!7O$>7::$$$O7!!>77$C>7:>:>>>!?CONO77>>>>>>>>>>>>>>!!?C7777
// ?>>C?CQQ?7>>>7>!!!>?O$QQOC>>!!>>>!!!C7?!>::>>!!?C7QNN?77777777>>>>>7C?77777
// ???CQQ$7>::::!!!!:!7>7COC$$$Q$O7>!:-!C-7:>::!>!!?7-QNNNQ$$$$$$$OOOCC?777777
// 777COO7>:---:;--::!!!!!!!7????CC>:!::?--7!:!!!>!:7!7QHHNNO>>>>>>?CC77777>>!
// >>>>O?7!::::--:::::::-:-::>77?77>:::!>:-:!!::!!!>>>-$QQQHNNO7>>?C?7>>>7>!!:
// !>!!??7>!::---:::---;..::!!>>777>--:!:::-!!>:!::!!:-O$$$$QHNNH??77>>>>!:!::
// >>>>>?>>!!::-;;;.;;;;;--!>>>>>>>:::::!:::!::::!-!!::COO$O$$QHNMC7>!>>:::!::
// 77777?>>!!::-;.-----::-->>>>>!!:-:-::::::::::!:::::7??CCCO$$$QQNNC>!:::::::
// ???????>!!!::-:!!::::::!>!>>>!!----:::---::::::::::?????7COO$$$$QHN?:::::::
// ????????!!!:!:::::::::!>!!!!!:----:::-:::::::::-:::7???777??OO$$$$QQH>!::::
// CCCCCCCCC>!!:!:::::::!>>!!!::---:-:::::::::::::::-77777>>>77??C$$OO$$H7::::
// CCCCCCCCCC7:!::::::!!!!!!::---:-:::::-:-:--::-::-77777>>>>>>77777CCOO$Q7:::
// CCCCCCCCC7>?::::::::::::--------------:----:-:-->>7777>>>>>>>>>77?COOO$Q7::
// CCCCCCCCC>>?>::-:--:-------------;------------->>>>7>>>>>>>>>>7777??OOO$Q!:
// CCCCCCCC?>>?7-:-------------;-;;-:-:---------->>>>>>>>>>>>>>>>>>>77??COO$Q:
// CCCCCCCC>>>??>-:----------------------------;>>>>>>!>>!!>>>>>>>>>>77??CCO$Q
// CCCCCCC?>>>777>::-----------;-------------;:>>>>>>!>>!!!!!!>>>>>>>>77?CCCO$
// CCCCCCC>!!>7>7>>!:--------;---;;--;---;;-;!!!>>>>!>>>!!!!!!>>>>>>>>>7??CCCO
// CCCCCCC>!!>7>>>>>>;:--;-;;--;;-;-----;;-!!!!!!!>!>>>!!!!!!!>>>>>>>>>7???COO
// CCCCCC7!!!!>>>>>>>>!;-----;;;;-;;;;-;:!!!!!!!!!!>>>>!!!!!!!>>>>>>>>>77??CCC
// QQQQQQQQQQ$$C?777>>>!!;;;;;-;;;;;;;:!!!!!!!!!:!>>>>!!>!>!!!>>>>>>>>>77????C
// >>>>>>>>>>>!>>>>>>>>>>>>>>>OQ;.::::!!!!!!!!:!!!>>>>>!!>!!!!!>>>>>>>>77????C
// >>>>>>>>>>>>>>>>>>>>>>>>>>>>>>!!!!!!!!!!!:!!!!!>>>>>>>!!:!!!>>>>>>>>77???CC
// >>>>>>>>>>>>>>>>>>>>>>>>>>>>>>:!!!!!!!::!!!!!!!!!!>>!!!!:!!!>>>>>>>>>77???C
// !!!!>>!!!!!!!!!!!!!!!!>>>>>>>7;;!>:-::!!!!!!!!!!!!!!!!!::!!!>>>>>>>>>777???
// !!!!!!!!!!!!!!!!!!>!!!!!!!>!!>>-::::!!!!!!!!!!!!!!!!!!!::!!!>>>>>>>>>7777??

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

	UserHash      string      `protobuf:"bytes,1,opt,name=userHash,proto3" json:"userHash,omitempty"`
	Command       Command     `protobuf:"varint,2,opt,name=command,proto3,enum=foliv.Command" json:"command,omitempty"`
	AddressType   AddressType `protobuf:"varint,3,opt,name=addressType,proto3,enum=foliv.AddressType" json:"addressType,omitempty"`
	Address       []byte      `protobuf:"bytes,4,opt,name=address,proto3" json:"address,omitempty"`
	Port          uint32      `protobuf:"varint,5,opt,name=port,proto3" json:"port,omitempty"`
	SourceName    string      `protobuf:"bytes,6,opt,name=sourceName,proto3" json:"sourceName,omitempty"`
	RouterName    string      `protobuf:"bytes,7,opt,name=routerName,proto3" json:"routerName,omitempty"`
	ProcessName   string      `protobuf:"bytes,8,opt,name=processName,proto3" json:"processName,omitempty"`
	XForwardedFor [][]byte    `protobuf:"bytes,9,rep,name=xForwardedFor,proto3" json:"xForwardedFor,omitempty"`
	IsTouch       bool        `protobuf:"varint,10,opt,name=isTouch,proto3" json:"isTouch,omitempty"`
	MuxID         uint32      `protobuf:"varint,11,opt,name=muxID,proto3" json:"muxID,omitempty"`
	Platform      string      `protobuf:"bytes,12,opt,name=platform,proto3" json:"platform,omitempty"`
	RequestID     []byte      `protobuf:"bytes,13,opt,name=requestID,proto3" json:"requestID,omitempty"`
	RouterLevel   uint32      `protobuf:"varint,14,opt,name=routerLevel,proto3" json:"routerLevel,omitempty"`
	UserAgent     string      `protobuf:"bytes,15,opt,name=userAgent,proto3" json:"userAgent,omitempty"`
	RequestHop    uint32      `protobuf:"varint,16,opt,name=requestHop,proto3" json:"requestHop,omitempty"`
	AppID         uint32      `protobuf:"varint,17,opt,name=appID,proto3" json:"appID,omitempty"`
	PeerID        uint32      `protobuf:"varint,18,opt,name=peerID,proto3" json:"peerID,omitempty"`
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

func (x *Foliv) GetAddress() []byte {
	if x != nil {
		return x.Address
	}
	return nil
}

func (x *Foliv) GetPort() uint32 {
	if x != nil {
		return x.Port
	}
	return 0
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

func (x *Foliv) GetProcessName() string {
	if x != nil {
		return x.ProcessName
	}
	return ""
}

func (x *Foliv) GetXForwardedFor() [][]byte {
	if x != nil {
		return x.XForwardedFor
	}
	return nil
}

func (x *Foliv) GetIsTouch() bool {
	if x != nil {
		return x.IsTouch
	}
	return false
}

func (x *Foliv) GetMuxID() uint32 {
	if x != nil {
		return x.MuxID
	}
	return 0
}

func (x *Foliv) GetPlatform() string {
	if x != nil {
		return x.Platform
	}
	return ""
}

func (x *Foliv) GetRequestID() []byte {
	if x != nil {
		return x.RequestID
	}
	return nil
}

func (x *Foliv) GetRouterLevel() uint32 {
	if x != nil {
		return x.RouterLevel
	}
	return 0
}

func (x *Foliv) GetUserAgent() string {
	if x != nil {
		return x.UserAgent
	}
	return ""
}

func (x *Foliv) GetRequestHop() uint32 {
	if x != nil {
		return x.RequestHop
	}
	return 0
}

func (x *Foliv) GetAppID() uint32 {
	if x != nil {
		return x.AppID
	}
	return 0
}

func (x *Foliv) GetPeerID() uint32 {
	if x != nil {
		return x.PeerID
	}
	return 0
}

var File_foliv_proto protoreflect.FileDescriptor

var file_foliv_proto_rawDesc = []byte{
	0x0a, 0x0b, 0x66, 0x6f, 0x6c, 0x69, 0x76, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x05, 0x66,
	0x6f, 0x6c, 0x69, 0x76, 0x22, 0xb1, 0x04, 0x0a, 0x05, 0x46, 0x6f, 0x6c, 0x69, 0x76, 0x12, 0x1a,
	0x0a, 0x08, 0x75, 0x73, 0x65, 0x72, 0x48, 0x61, 0x73, 0x68, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09,
	0x52, 0x08, 0x75, 0x73, 0x65, 0x72, 0x48, 0x61, 0x73, 0x68, 0x12, 0x28, 0x0a, 0x07, 0x63, 0x6f,
	0x6d, 0x6d, 0x61, 0x6e, 0x64, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x0e, 0x2e, 0x66, 0x6f,
	0x6c, 0x69, 0x76, 0x2e, 0x43, 0x6f, 0x6d, 0x6d, 0x61, 0x6e, 0x64, 0x52, 0x07, 0x63, 0x6f, 0x6d,
	0x6d, 0x61, 0x6e, 0x64, 0x12, 0x34, 0x0a, 0x0b, 0x61, 0x64, 0x64, 0x72, 0x65, 0x73, 0x73, 0x54,
	0x79, 0x70, 0x65, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x12, 0x2e, 0x66, 0x6f, 0x6c, 0x69,
	0x76, 0x2e, 0x41, 0x64, 0x64, 0x72, 0x65, 0x73, 0x73, 0x54, 0x79, 0x70, 0x65, 0x52, 0x0b, 0x61,
	0x64, 0x64, 0x72, 0x65, 0x73, 0x73, 0x54, 0x79, 0x70, 0x65, 0x12, 0x18, 0x0a, 0x07, 0x61, 0x64,
	0x64, 0x72, 0x65, 0x73, 0x73, 0x18, 0x04, 0x20, 0x01, 0x28, 0x0c, 0x52, 0x07, 0x61, 0x64, 0x64,
	0x72, 0x65, 0x73, 0x73, 0x12, 0x12, 0x0a, 0x04, 0x70, 0x6f, 0x72, 0x74, 0x18, 0x05, 0x20, 0x01,
	0x28, 0x0d, 0x52, 0x04, 0x70, 0x6f, 0x72, 0x74, 0x12, 0x1e, 0x0a, 0x0a, 0x73, 0x6f, 0x75, 0x72,
	0x63, 0x65, 0x4e, 0x61, 0x6d, 0x65, 0x18, 0x06, 0x20, 0x01, 0x28, 0x09, 0x52, 0x0a, 0x73, 0x6f,
	0x75, 0x72, 0x63, 0x65, 0x4e, 0x61, 0x6d, 0x65, 0x12, 0x1e, 0x0a, 0x0a, 0x72, 0x6f, 0x75, 0x74,
	0x65, 0x72, 0x4e, 0x61, 0x6d, 0x65, 0x18, 0x07, 0x20, 0x01, 0x28, 0x09, 0x52, 0x0a, 0x72, 0x6f,
	0x75, 0x74, 0x65, 0x72, 0x4e, 0x61, 0x6d, 0x65, 0x12, 0x20, 0x0a, 0x0b, 0x70, 0x72, 0x6f, 0x63,
	0x65, 0x73, 0x73, 0x4e, 0x61, 0x6d, 0x65, 0x18, 0x08, 0x20, 0x01, 0x28, 0x09, 0x52, 0x0b, 0x70,
	0x72, 0x6f, 0x63, 0x65, 0x73, 0x73, 0x4e, 0x61, 0x6d, 0x65, 0x12, 0x24, 0x0a, 0x0d, 0x78, 0x46,
	0x6f, 0x72, 0x77, 0x61, 0x72, 0x64, 0x65, 0x64, 0x46, 0x6f, 0x72, 0x18, 0x09, 0x20, 0x03, 0x28,
	0x0c, 0x52, 0x0d, 0x78, 0x46, 0x6f, 0x72, 0x77, 0x61, 0x72, 0x64, 0x65, 0x64, 0x46, 0x6f, 0x72,
	0x12, 0x18, 0x0a, 0x07, 0x69, 0x73, 0x54, 0x6f, 0x75, 0x63, 0x68, 0x18, 0x0a, 0x20, 0x01, 0x28,
	0x08, 0x52, 0x07, 0x69, 0x73, 0x54, 0x6f, 0x75, 0x63, 0x68, 0x12, 0x14, 0x0a, 0x05, 0x6d, 0x75,
	0x78, 0x49, 0x44, 0x18, 0x0b, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x05, 0x6d, 0x75, 0x78, 0x49, 0x44,
	0x12, 0x1a, 0x0a, 0x08, 0x70, 0x6c, 0x61, 0x74, 0x66, 0x6f, 0x72, 0x6d, 0x18, 0x0c, 0x20, 0x01,
	0x28, 0x09, 0x52, 0x08, 0x70, 0x6c, 0x61, 0x74, 0x66, 0x6f, 0x72, 0x6d, 0x12, 0x1c, 0x0a, 0x09,
	0x72, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x49, 0x44, 0x18, 0x0d, 0x20, 0x01, 0x28, 0x0c, 0x52,
	0x09, 0x72, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x49, 0x44, 0x12, 0x20, 0x0a, 0x0b, 0x72, 0x6f,
	0x75, 0x74, 0x65, 0x72, 0x4c, 0x65, 0x76, 0x65, 0x6c, 0x18, 0x0e, 0x20, 0x01, 0x28, 0x0d, 0x52,
	0x0b, 0x72, 0x6f, 0x75, 0x74, 0x65, 0x72, 0x4c, 0x65, 0x76, 0x65, 0x6c, 0x12, 0x1c, 0x0a, 0x09,
	0x75, 0x73, 0x65, 0x72, 0x41, 0x67, 0x65, 0x6e, 0x74, 0x18, 0x0f, 0x20, 0x01, 0x28, 0x09, 0x52,
	0x09, 0x75, 0x73, 0x65, 0x72, 0x41, 0x67, 0x65, 0x6e, 0x74, 0x12, 0x1e, 0x0a, 0x0a, 0x72, 0x65,
	0x71, 0x75, 0x65, 0x73, 0x74, 0x48, 0x6f, 0x70, 0x18, 0x10, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x0a,
	0x72, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x48, 0x6f, 0x70, 0x12, 0x14, 0x0a, 0x05, 0x61, 0x70,
	0x70, 0x49, 0x44, 0x18, 0x11, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x05, 0x61, 0x70, 0x70, 0x49, 0x44,
	0x12, 0x16, 0x0a, 0x06, 0x70, 0x65, 0x65, 0x72, 0x49, 0x44, 0x18, 0x12, 0x20, 0x01, 0x28, 0x0d,
	0x52, 0x06, 0x70, 0x65, 0x65, 0x72, 0x49, 0x44, 0x2a, 0x39, 0x0a, 0x07, 0x43, 0x6f, 0x6d, 0x6d,
	0x61, 0x6e, 0x64, 0x12, 0x09, 0x0a, 0x05, 0x45, 0x6d, 0x70, 0x74, 0x79, 0x10, 0x00, 0x12, 0x0b,
	0x0a, 0x07, 0x43, 0x6f, 0x6e, 0x6e, 0x65, 0x63, 0x74, 0x10, 0x01, 0x12, 0x0d, 0x0a, 0x09, 0x41,
	0x73, 0x73, 0x6f, 0x63, 0x69, 0x61, 0x74, 0x65, 0x10, 0x03, 0x12, 0x07, 0x0a, 0x03, 0x4d, 0x75,
	0x78, 0x10, 0x7f, 0x2a, 0x42, 0x0a, 0x0b, 0x41, 0x64, 0x64, 0x72, 0x65, 0x73, 0x73, 0x54, 0x79,
	0x70, 0x65, 0x12, 0x0f, 0x0a, 0x0b, 0x49, 0x6e, 0x76, 0x61, 0x6c, 0x69, 0x64, 0x54, 0x79, 0x70,
	0x65, 0x10, 0x00, 0x12, 0x08, 0x0a, 0x04, 0x49, 0x50, 0x76, 0x34, 0x10, 0x01, 0x12, 0x0e, 0x0a,
	0x0a, 0x44, 0x6f, 0x6d, 0x61, 0x69, 0x6e, 0x4e, 0x61, 0x6d, 0x65, 0x10, 0x03, 0x12, 0x08, 0x0a,
	0x04, 0x49, 0x50, 0x76, 0x36, 0x10, 0x04, 0x42, 0x09, 0x5a, 0x07, 0x2e, 0x2f, 0x66, 0x6f, 0x6c,
	0x69, 0x76, 0x62, 0x06, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x33,
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
