// source: foliv.proto
/**
 * @fileoverview
 * @enhanceable
 * @suppress {missingRequire} reports error on implicit type usages.
 * @suppress {messageConventions} JS Compiler reports an error if a variable or
 *     field starts with 'MSG_' and isn't a translatable message.
 * @public
 */
// GENERATED CODE -- DO NOT EDIT!
/* eslint-disable */
// @ts-nocheck

goog.provide('proto.foliv.Foliv');

goog.require('jspb.BinaryReader');
goog.require('jspb.BinaryWriter');
goog.require('jspb.Message');

goog.forwardDeclare('proto.foliv.AddressType');
goog.forwardDeclare('proto.foliv.Command');
/**
 * Generated by JsPbCodeGenerator.
 * @param {Array=} opt_data Optional initial data array, typically from a
 * server response, or constructed directly in Javascript. The array is used
 * in place and becomes part of the constructed object. It is not cloned.
 * If no data is provided, the constructed object will be empty, but still
 * valid.
 * @extends {jspb.Message}
 * @constructor
 */
proto.foliv.Foliv = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, proto.foliv.Foliv.repeatedFields_, null);
};
goog.inherits(proto.foliv.Foliv, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.foliv.Foliv.displayName = 'proto.foliv.Foliv';
}

/**
 * List of repeated fields within this message type.
 * @private {!Array<number>}
 * @const
 */
proto.foliv.Foliv.repeatedFields_ = [9,21];



if (jspb.Message.GENERATE_TO_OBJECT) {
/**
 * Creates an object representation of this proto.
 * Field names that are reserved in JavaScript and will be renamed to pb_name.
 * Optional fields that are not set will be set to undefined.
 * To access a reserved field use, foo.pb_<name>, eg, foo.pb_default.
 * For the list of reserved names please see:
 *     net/proto2/compiler/js/internal/generator.cc#kKeyword.
 * @param {boolean=} opt_includeInstance Deprecated. whether to include the
 *     JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @return {!Object}
 */
proto.foliv.Foliv.prototype.toObject = function(opt_includeInstance) {
  return proto.foliv.Foliv.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.foliv.Foliv} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.foliv.Foliv.toObject = function(includeInstance, msg) {
  var f, obj = {
    userhash: jspb.Message.getFieldWithDefault(msg, 1, ""),
    command: jspb.Message.getFieldWithDefault(msg, 2, 0),
    addresstype: jspb.Message.getFieldWithDefault(msg, 3, 0),
    address: msg.getAddress_asB64(),
    port: jspb.Message.getFieldWithDefault(msg, 5, 0),
    sourcename: jspb.Message.getFieldWithDefault(msg, 6, ""),
    routername: jspb.Message.getFieldWithDefault(msg, 7, ""),
    processname: jspb.Message.getFieldWithDefault(msg, 8, ""),
    xforwardedforList: msg.getXforwardedforList_asB64(),
    istouch: jspb.Message.getBooleanFieldWithDefault(msg, 10, false),
    muxid: jspb.Message.getFieldWithDefault(msg, 11, 0),
    platform: jspb.Message.getFieldWithDefault(msg, 12, ""),
    requestid: msg.getRequestid_asB64(),
    routerlevel: jspb.Message.getFieldWithDefault(msg, 14, 0),
    useragent: jspb.Message.getFieldWithDefault(msg, 15, ""),
    requesthop: jspb.Message.getFieldWithDefault(msg, 16, 0),
    appid: jspb.Message.getFieldWithDefault(msg, 17, 0),
    peerid: jspb.Message.getFieldWithDefault(msg, 18, 0),
    version: jspb.Message.getFieldWithDefault(msg, 19, 0),
    roundtriptime: jspb.Message.getFieldWithDefault(msg, 20, 0),
    bindipsList: msg.getBindipsList_asB64(),
    routerpath: jspb.Message.getFieldWithDefault(msg, 22, "")
  };

  if (includeInstance) {
    obj.$jspbMessageInstance = msg;
  }
  return obj;
};
}


/**
 * Deserializes binary data (in protobuf wire format).
 * @param {jspb.ByteSource} bytes The bytes to deserialize.
 * @return {!proto.foliv.Foliv}
 */
proto.foliv.Foliv.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.foliv.Foliv;
  return proto.foliv.Foliv.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.foliv.Foliv} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.foliv.Foliv}
 */
proto.foliv.Foliv.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = /** @type {string} */ (reader.readString());
      msg.setUserhash(value);
      break;
    case 2:
      var value = /** @type {!proto.foliv.Command} */ (reader.readEnum());
      msg.setCommand(value);
      break;
    case 3:
      var value = /** @type {!proto.foliv.AddressType} */ (reader.readEnum());
      msg.setAddresstype(value);
      break;
    case 4:
      var value = /** @type {!Uint8Array} */ (reader.readBytes());
      msg.setAddress(value);
      break;
    case 5:
      var value = /** @type {number} */ (reader.readUint32());
      msg.setPort(value);
      break;
    case 6:
      var value = /** @type {string} */ (reader.readString());
      msg.setSourcename(value);
      break;
    case 7:
      var value = /** @type {string} */ (reader.readString());
      msg.setRoutername(value);
      break;
    case 8:
      var value = /** @type {string} */ (reader.readString());
      msg.setProcessname(value);
      break;
    case 9:
      var value = /** @type {!Uint8Array} */ (reader.readBytes());
      msg.addXforwardedfor(value);
      break;
    case 10:
      var value = /** @type {boolean} */ (reader.readBool());
      msg.setIstouch(value);
      break;
    case 11:
      var value = /** @type {number} */ (reader.readUint32());
      msg.setMuxid(value);
      break;
    case 12:
      var value = /** @type {string} */ (reader.readString());
      msg.setPlatform(value);
      break;
    case 13:
      var value = /** @type {!Uint8Array} */ (reader.readBytes());
      msg.setRequestid(value);
      break;
    case 14:
      var value = /** @type {number} */ (reader.readUint32());
      msg.setRouterlevel(value);
      break;
    case 15:
      var value = /** @type {string} */ (reader.readString());
      msg.setUseragent(value);
      break;
    case 16:
      var value = /** @type {number} */ (reader.readUint32());
      msg.setRequesthop(value);
      break;
    case 17:
      var value = /** @type {number} */ (reader.readUint32());
      msg.setAppid(value);
      break;
    case 18:
      var value = /** @type {number} */ (reader.readUint32());
      msg.setPeerid(value);
      break;
    case 19:
      var value = /** @type {number} */ (reader.readUint32());
      msg.setVersion(value);
      break;
    case 20:
      var value = /** @type {number} */ (reader.readInt32());
      msg.setRoundtriptime(value);
      break;
    case 21:
      var value = /** @type {!Uint8Array} */ (reader.readBytes());
      msg.addBindips(value);
      break;
    case 22:
      var value = /** @type {string} */ (reader.readString());
      msg.setRouterpath(value);
      break;
    default:
      reader.skipField();
      break;
    }
  }
  return msg;
};


/**
 * Serializes the message to binary data (in protobuf wire format).
 * @return {!Uint8Array}
 */
proto.foliv.Foliv.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.foliv.Foliv.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.foliv.Foliv} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.foliv.Foliv.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getUserhash();
  if (f.length > 0) {
    writer.writeString(
      1,
      f
    );
  }
  f = message.getCommand();
  if (f !== 0.0) {
    writer.writeEnum(
      2,
      f
    );
  }
  f = message.getAddresstype();
  if (f !== 0.0) {
    writer.writeEnum(
      3,
      f
    );
  }
  f = message.getAddress_asU8();
  if (f.length > 0) {
    writer.writeBytes(
      4,
      f
    );
  }
  f = message.getPort();
  if (f !== 0) {
    writer.writeUint32(
      5,
      f
    );
  }
  f = message.getSourcename();
  if (f.length > 0) {
    writer.writeString(
      6,
      f
    );
  }
  f = message.getRoutername();
  if (f.length > 0) {
    writer.writeString(
      7,
      f
    );
  }
  f = message.getProcessname();
  if (f.length > 0) {
    writer.writeString(
      8,
      f
    );
  }
  f = message.getXforwardedforList_asU8();
  if (f.length > 0) {
    writer.writeRepeatedBytes(
      9,
      f
    );
  }
  f = message.getIstouch();
  if (f) {
    writer.writeBool(
      10,
      f
    );
  }
  f = message.getMuxid();
  if (f !== 0) {
    writer.writeUint32(
      11,
      f
    );
  }
  f = message.getPlatform();
  if (f.length > 0) {
    writer.writeString(
      12,
      f
    );
  }
  f = message.getRequestid_asU8();
  if (f.length > 0) {
    writer.writeBytes(
      13,
      f
    );
  }
  f = message.getRouterlevel();
  if (f !== 0) {
    writer.writeUint32(
      14,
      f
    );
  }
  f = message.getUseragent();
  if (f.length > 0) {
    writer.writeString(
      15,
      f
    );
  }
  f = message.getRequesthop();
  if (f !== 0) {
    writer.writeUint32(
      16,
      f
    );
  }
  f = message.getAppid();
  if (f !== 0) {
    writer.writeUint32(
      17,
      f
    );
  }
  f = message.getPeerid();
  if (f !== 0) {
    writer.writeUint32(
      18,
      f
    );
  }
  f = message.getVersion();
  if (f !== 0) {
    writer.writeUint32(
      19,
      f
    );
  }
  f = message.getRoundtriptime();
  if (f !== 0) {
    writer.writeInt32(
      20,
      f
    );
  }
  f = message.getBindipsList_asU8();
  if (f.length > 0) {
    writer.writeRepeatedBytes(
      21,
      f
    );
  }
  f = message.getRouterpath();
  if (f.length > 0) {
    writer.writeString(
      22,
      f
    );
  }
};


/**
 * optional string userHash = 1;
 * @return {string}
 */
proto.foliv.Foliv.prototype.getUserhash = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 1, ""));
};


/**
 * @param {string} value
 * @return {!proto.foliv.Foliv} returns this
 */
proto.foliv.Foliv.prototype.setUserhash = function(value) {
  return jspb.Message.setProto3StringField(this, 1, value);
};


/**
 * optional Command command = 2;
 * @return {!proto.foliv.Command}
 */
proto.foliv.Foliv.prototype.getCommand = function() {
  return /** @type {!proto.foliv.Command} */ (jspb.Message.getFieldWithDefault(this, 2, 0));
};


/**
 * @param {!proto.foliv.Command} value
 * @return {!proto.foliv.Foliv} returns this
 */
proto.foliv.Foliv.prototype.setCommand = function(value) {
  return jspb.Message.setProto3EnumField(this, 2, value);
};


/**
 * optional AddressType addressType = 3;
 * @return {!proto.foliv.AddressType}
 */
proto.foliv.Foliv.prototype.getAddresstype = function() {
  return /** @type {!proto.foliv.AddressType} */ (jspb.Message.getFieldWithDefault(this, 3, 0));
};


/**
 * @param {!proto.foliv.AddressType} value
 * @return {!proto.foliv.Foliv} returns this
 */
proto.foliv.Foliv.prototype.setAddresstype = function(value) {
  return jspb.Message.setProto3EnumField(this, 3, value);
};


/**
 * optional bytes address = 4;
 * @return {string}
 */
proto.foliv.Foliv.prototype.getAddress = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 4, ""));
};


/**
 * optional bytes address = 4;
 * This is a type-conversion wrapper around `getAddress()`
 * @return {string}
 */
proto.foliv.Foliv.prototype.getAddress_asB64 = function() {
  return /** @type {string} */ (jspb.Message.bytesAsB64(
      this.getAddress()));
};


/**
 * optional bytes address = 4;
 * Note that Uint8Array is not supported on all browsers.
 * @see http://caniuse.com/Uint8Array
 * This is a type-conversion wrapper around `getAddress()`
 * @return {!Uint8Array}
 */
proto.foliv.Foliv.prototype.getAddress_asU8 = function() {
  return /** @type {!Uint8Array} */ (jspb.Message.bytesAsU8(
      this.getAddress()));
};


/**
 * @param {!(string|Uint8Array)} value
 * @return {!proto.foliv.Foliv} returns this
 */
proto.foliv.Foliv.prototype.setAddress = function(value) {
  return jspb.Message.setProto3BytesField(this, 4, value);
};


/**
 * optional uint32 port = 5;
 * @return {number}
 */
proto.foliv.Foliv.prototype.getPort = function() {
  return /** @type {number} */ (jspb.Message.getFieldWithDefault(this, 5, 0));
};


/**
 * @param {number} value
 * @return {!proto.foliv.Foliv} returns this
 */
proto.foliv.Foliv.prototype.setPort = function(value) {
  return jspb.Message.setProto3IntField(this, 5, value);
};


/**
 * optional string sourceName = 6;
 * @return {string}
 */
proto.foliv.Foliv.prototype.getSourcename = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 6, ""));
};


/**
 * @param {string} value
 * @return {!proto.foliv.Foliv} returns this
 */
proto.foliv.Foliv.prototype.setSourcename = function(value) {
  return jspb.Message.setProto3StringField(this, 6, value);
};


/**
 * optional string routerName = 7;
 * @return {string}
 */
proto.foliv.Foliv.prototype.getRoutername = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 7, ""));
};


/**
 * @param {string} value
 * @return {!proto.foliv.Foliv} returns this
 */
proto.foliv.Foliv.prototype.setRoutername = function(value) {
  return jspb.Message.setProto3StringField(this, 7, value);
};


/**
 * optional string processName = 8;
 * @return {string}
 */
proto.foliv.Foliv.prototype.getProcessname = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 8, ""));
};


/**
 * @param {string} value
 * @return {!proto.foliv.Foliv} returns this
 */
proto.foliv.Foliv.prototype.setProcessname = function(value) {
  return jspb.Message.setProto3StringField(this, 8, value);
};


/**
 * repeated bytes xForwardedFor = 9;
 * @return {!Array<string>}
 */
proto.foliv.Foliv.prototype.getXforwardedforList = function() {
  return /** @type {!Array<string>} */ (jspb.Message.getRepeatedField(this, 9));
};


/**
 * repeated bytes xForwardedFor = 9;
 * This is a type-conversion wrapper around `getXforwardedforList()`
 * @return {!Array<string>}
 */
proto.foliv.Foliv.prototype.getXforwardedforList_asB64 = function() {
  return /** @type {!Array<string>} */ (jspb.Message.bytesListAsB64(
      this.getXforwardedforList()));
};


/**
 * repeated bytes xForwardedFor = 9;
 * Note that Uint8Array is not supported on all browsers.
 * @see http://caniuse.com/Uint8Array
 * This is a type-conversion wrapper around `getXforwardedforList()`
 * @return {!Array<!Uint8Array>}
 */
proto.foliv.Foliv.prototype.getXforwardedforList_asU8 = function() {
  return /** @type {!Array<!Uint8Array>} */ (jspb.Message.bytesListAsU8(
      this.getXforwardedforList()));
};


/**
 * @param {!(Array<!Uint8Array>|Array<string>)} value
 * @return {!proto.foliv.Foliv} returns this
 */
proto.foliv.Foliv.prototype.setXforwardedforList = function(value) {
  return jspb.Message.setField(this, 9, value || []);
};


/**
 * @param {!(string|Uint8Array)} value
 * @param {number=} opt_index
 * @return {!proto.foliv.Foliv} returns this
 */
proto.foliv.Foliv.prototype.addXforwardedfor = function(value, opt_index) {
  return jspb.Message.addToRepeatedField(this, 9, value, opt_index);
};


/**
 * Clears the list making it empty but non-null.
 * @return {!proto.foliv.Foliv} returns this
 */
proto.foliv.Foliv.prototype.clearXforwardedforList = function() {
  return this.setXforwardedforList([]);
};


/**
 * optional bool isTouch = 10;
 * @return {boolean}
 */
proto.foliv.Foliv.prototype.getIstouch = function() {
  return /** @type {boolean} */ (jspb.Message.getBooleanFieldWithDefault(this, 10, false));
};


/**
 * @param {boolean} value
 * @return {!proto.foliv.Foliv} returns this
 */
proto.foliv.Foliv.prototype.setIstouch = function(value) {
  return jspb.Message.setProto3BooleanField(this, 10, value);
};


/**
 * optional uint32 muxID = 11;
 * @return {number}
 */
proto.foliv.Foliv.prototype.getMuxid = function() {
  return /** @type {number} */ (jspb.Message.getFieldWithDefault(this, 11, 0));
};


/**
 * @param {number} value
 * @return {!proto.foliv.Foliv} returns this
 */
proto.foliv.Foliv.prototype.setMuxid = function(value) {
  return jspb.Message.setProto3IntField(this, 11, value);
};


/**
 * optional string platform = 12;
 * @return {string}
 */
proto.foliv.Foliv.prototype.getPlatform = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 12, ""));
};


/**
 * @param {string} value
 * @return {!proto.foliv.Foliv} returns this
 */
proto.foliv.Foliv.prototype.setPlatform = function(value) {
  return jspb.Message.setProto3StringField(this, 12, value);
};


/**
 * optional bytes requestID = 13;
 * @return {string}
 */
proto.foliv.Foliv.prototype.getRequestid = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 13, ""));
};


/**
 * optional bytes requestID = 13;
 * This is a type-conversion wrapper around `getRequestid()`
 * @return {string}
 */
proto.foliv.Foliv.prototype.getRequestid_asB64 = function() {
  return /** @type {string} */ (jspb.Message.bytesAsB64(
      this.getRequestid()));
};


/**
 * optional bytes requestID = 13;
 * Note that Uint8Array is not supported on all browsers.
 * @see http://caniuse.com/Uint8Array
 * This is a type-conversion wrapper around `getRequestid()`
 * @return {!Uint8Array}
 */
proto.foliv.Foliv.prototype.getRequestid_asU8 = function() {
  return /** @type {!Uint8Array} */ (jspb.Message.bytesAsU8(
      this.getRequestid()));
};


/**
 * @param {!(string|Uint8Array)} value
 * @return {!proto.foliv.Foliv} returns this
 */
proto.foliv.Foliv.prototype.setRequestid = function(value) {
  return jspb.Message.setProto3BytesField(this, 13, value);
};


/**
 * optional uint32 routerLevel = 14;
 * @return {number}
 */
proto.foliv.Foliv.prototype.getRouterlevel = function() {
  return /** @type {number} */ (jspb.Message.getFieldWithDefault(this, 14, 0));
};


/**
 * @param {number} value
 * @return {!proto.foliv.Foliv} returns this
 */
proto.foliv.Foliv.prototype.setRouterlevel = function(value) {
  return jspb.Message.setProto3IntField(this, 14, value);
};


/**
 * optional string userAgent = 15;
 * @return {string}
 */
proto.foliv.Foliv.prototype.getUseragent = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 15, ""));
};


/**
 * @param {string} value
 * @return {!proto.foliv.Foliv} returns this
 */
proto.foliv.Foliv.prototype.setUseragent = function(value) {
  return jspb.Message.setProto3StringField(this, 15, value);
};


/**
 * optional uint32 requestHop = 16;
 * @return {number}
 */
proto.foliv.Foliv.prototype.getRequesthop = function() {
  return /** @type {number} */ (jspb.Message.getFieldWithDefault(this, 16, 0));
};


/**
 * @param {number} value
 * @return {!proto.foliv.Foliv} returns this
 */
proto.foliv.Foliv.prototype.setRequesthop = function(value) {
  return jspb.Message.setProto3IntField(this, 16, value);
};


/**
 * optional uint32 appID = 17;
 * @return {number}
 */
proto.foliv.Foliv.prototype.getAppid = function() {
  return /** @type {number} */ (jspb.Message.getFieldWithDefault(this, 17, 0));
};


/**
 * @param {number} value
 * @return {!proto.foliv.Foliv} returns this
 */
proto.foliv.Foliv.prototype.setAppid = function(value) {
  return jspb.Message.setProto3IntField(this, 17, value);
};


/**
 * optional uint32 peerID = 18;
 * @return {number}
 */
proto.foliv.Foliv.prototype.getPeerid = function() {
  return /** @type {number} */ (jspb.Message.getFieldWithDefault(this, 18, 0));
};


/**
 * @param {number} value
 * @return {!proto.foliv.Foliv} returns this
 */
proto.foliv.Foliv.prototype.setPeerid = function(value) {
  return jspb.Message.setProto3IntField(this, 18, value);
};


/**
 * optional uint32 version = 19;
 * @return {number}
 */
proto.foliv.Foliv.prototype.getVersion = function() {
  return /** @type {number} */ (jspb.Message.getFieldWithDefault(this, 19, 0));
};


/**
 * @param {number} value
 * @return {!proto.foliv.Foliv} returns this
 */
proto.foliv.Foliv.prototype.setVersion = function(value) {
  return jspb.Message.setProto3IntField(this, 19, value);
};


/**
 * optional int32 roundtripTime = 20;
 * @return {number}
 */
proto.foliv.Foliv.prototype.getRoundtriptime = function() {
  return /** @type {number} */ (jspb.Message.getFieldWithDefault(this, 20, 0));
};


/**
 * @param {number} value
 * @return {!proto.foliv.Foliv} returns this
 */
proto.foliv.Foliv.prototype.setRoundtriptime = function(value) {
  return jspb.Message.setProto3IntField(this, 20, value);
};


/**
 * repeated bytes bindIPs = 21;
 * @return {!Array<string>}
 */
proto.foliv.Foliv.prototype.getBindipsList = function() {
  return /** @type {!Array<string>} */ (jspb.Message.getRepeatedField(this, 21));
};


/**
 * repeated bytes bindIPs = 21;
 * This is a type-conversion wrapper around `getBindipsList()`
 * @return {!Array<string>}
 */
proto.foliv.Foliv.prototype.getBindipsList_asB64 = function() {
  return /** @type {!Array<string>} */ (jspb.Message.bytesListAsB64(
      this.getBindipsList()));
};


/**
 * repeated bytes bindIPs = 21;
 * Note that Uint8Array is not supported on all browsers.
 * @see http://caniuse.com/Uint8Array
 * This is a type-conversion wrapper around `getBindipsList()`
 * @return {!Array<!Uint8Array>}
 */
proto.foliv.Foliv.prototype.getBindipsList_asU8 = function() {
  return /** @type {!Array<!Uint8Array>} */ (jspb.Message.bytesListAsU8(
      this.getBindipsList()));
};


/**
 * @param {!(Array<!Uint8Array>|Array<string>)} value
 * @return {!proto.foliv.Foliv} returns this
 */
proto.foliv.Foliv.prototype.setBindipsList = function(value) {
  return jspb.Message.setField(this, 21, value || []);
};


/**
 * @param {!(string|Uint8Array)} value
 * @param {number=} opt_index
 * @return {!proto.foliv.Foliv} returns this
 */
proto.foliv.Foliv.prototype.addBindips = function(value, opt_index) {
  return jspb.Message.addToRepeatedField(this, 21, value, opt_index);
};


/**
 * Clears the list making it empty but non-null.
 * @return {!proto.foliv.Foliv} returns this
 */
proto.foliv.Foliv.prototype.clearBindipsList = function() {
  return this.setBindipsList([]);
};


/**
 * optional string routerPath = 22;
 * @return {string}
 */
proto.foliv.Foliv.prototype.getRouterpath = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 22, ""));
};


/**
 * @param {string} value
 * @return {!proto.foliv.Foliv} returns this
 */
proto.foliv.Foliv.prototype.setRouterpath = function(value) {
  return jspb.Message.setProto3StringField(this, 22, value);
};


