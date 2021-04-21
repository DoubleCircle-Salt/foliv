// Generated by the protocol buffer compiler.  DO NOT EDIT!
// source: foliv.proto

// This CPP symbol can be defined to use imports that match up to the framework
// imports needed when using CocoaPods.
#if !defined(GPB_USE_PROTOBUF_FRAMEWORK_IMPORTS)
 #define GPB_USE_PROTOBUF_FRAMEWORK_IMPORTS 0
#endif

#if GPB_USE_PROTOBUF_FRAMEWORK_IMPORTS
 #import <Protobuf/GPBProtocolBuffers.h>
#else
 #import "GPBProtocolBuffers.h"
#endif

#if GOOGLE_PROTOBUF_OBJC_VERSION < 30004
#error This file was generated by a newer version of protoc which is incompatible with your Protocol Buffer library sources.
#endif
#if 30004 < GOOGLE_PROTOBUF_OBJC_MIN_SUPPORTED_VERSION
#error This file was generated by an older version of protoc which is incompatible with your Protocol Buffer library sources.
#endif

// @@protoc_insertion_point(imports)

#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wdeprecated-declarations"

CF_EXTERN_C_BEGIN

NS_ASSUME_NONNULL_BEGIN

#pragma mark - Enum Command

typedef GPB_ENUM(Command) {
  /**
   * Value used if any message's field encounters a value that is not defined
   * by this enum. The message will also have C functions to get/set the rawValue
   * of the field.
   **/
  Command_GPBUnrecognizedEnumeratorValue = kGPBUnrecognizedEnumeratorValue,
  Command_Empty = 0,
  Command_Connect = 1,
  Command_Associate = 3,
  Command_Mux = 127,
};

GPBEnumDescriptor *Command_EnumDescriptor(void);

/**
 * Checks to see if the given value is defined by the enum or was not known at
 * the time this source was generated.
 **/
BOOL Command_IsValidValue(int32_t value);

#pragma mark - Enum AddressType

typedef GPB_ENUM(AddressType) {
  /**
   * Value used if any message's field encounters a value that is not defined
   * by this enum. The message will also have C functions to get/set the rawValue
   * of the field.
   **/
  AddressType_GPBUnrecognizedEnumeratorValue = kGPBUnrecognizedEnumeratorValue,
  AddressType_InvalidType = 0,
  AddressType_Ipv4 = 1,
  AddressType_DomainName = 3,
  AddressType_Ipv6 = 4,
};

GPBEnumDescriptor *AddressType_EnumDescriptor(void);

/**
 * Checks to see if the given value is defined by the enum or was not known at
 * the time this source was generated.
 **/
BOOL AddressType_IsValidValue(int32_t value);

#pragma mark - FolivRoot

/**
 * Exposes the extension registry for this file.
 *
 * The base class provides:
 * @code
 *   + (GPBExtensionRegistry *)extensionRegistry;
 * @endcode
 * which is a @c GPBExtensionRegistry that includes all the extensions defined by
 * this file and all files that it depends on.
 **/
GPB_FINAL @interface FolivRoot : GPBRootObject
@end

#pragma mark - Foliv

typedef GPB_ENUM(Foliv_FieldNumber) {
  Foliv_FieldNumber_UserHash = 1,
  Foliv_FieldNumber_Command = 2,
  Foliv_FieldNumber_AddressType = 3,
  Foliv_FieldNumber_Address = 4,
  Foliv_FieldNumber_SourceName = 5,
  Foliv_FieldNumber_RouterName = 6,
};

GPB_FINAL @interface Foliv : GPBMessage

@property(nonatomic, readwrite, copy, null_resettable) NSString *userHash;

@property(nonatomic, readwrite) Command command;

@property(nonatomic, readwrite) AddressType addressType;

@property(nonatomic, readwrite, copy, null_resettable) NSString *address;

@property(nonatomic, readwrite, copy, null_resettable) NSString *sourceName;

@property(nonatomic, readwrite, copy, null_resettable) NSString *routerName;

@end

/**
 * Fetches the raw value of a @c Foliv's @c command property, even
 * if the value was not defined by the enum at the time the code was generated.
 **/
int32_t Foliv_Command_RawValue(Foliv *message);
/**
 * Sets the raw value of an @c Foliv's @c command property, allowing
 * it to be set to a value that was not defined by the enum at the time the code
 * was generated.
 **/
void SetFoliv_Command_RawValue(Foliv *message, int32_t value);

/**
 * Fetches the raw value of a @c Foliv's @c addressType property, even
 * if the value was not defined by the enum at the time the code was generated.
 **/
int32_t Foliv_AddressType_RawValue(Foliv *message);
/**
 * Sets the raw value of an @c Foliv's @c addressType property, allowing
 * it to be set to a value that was not defined by the enum at the time the code
 * was generated.
 **/
void SetFoliv_AddressType_RawValue(Foliv *message, int32_t value);

NS_ASSUME_NONNULL_END

CF_EXTERN_C_END

#pragma clang diagnostic pop

// @@protoc_insertion_point(global_scope)