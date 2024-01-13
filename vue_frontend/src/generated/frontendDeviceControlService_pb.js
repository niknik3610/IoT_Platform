// source: frontendDeviceControlService.proto
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

var jspb = require('google-protobuf');
var goog = jspb;
var global = (function() {
  if (this) { return this; }
  if (typeof window !== 'undefined') { return window; }
  if (typeof global !== 'undefined') { return global; }
  if (typeof self !== 'undefined') { return self; }
  return Function('return this')();
}.call(null));

var frontend_types_pb = require('./frontend_types_pb.js');
goog.object.extend(proto, frontend_types_pb);
goog.exportSymbol('proto.frontend.devicecontrol.DeviceControlRequest', null, global);
goog.exportSymbol('proto.frontend.devicecontrol.DeviceControlResponse', null, global);
goog.exportSymbol('proto.frontend.devicecontrol.DeviceControlResult', null, global);
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
proto.frontend.devicecontrol.DeviceControlRequest = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.frontend.devicecontrol.DeviceControlRequest, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.frontend.devicecontrol.DeviceControlRequest.displayName = 'proto.frontend.devicecontrol.DeviceControlRequest';
}
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
proto.frontend.devicecontrol.DeviceControlResponse = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.frontend.devicecontrol.DeviceControlResponse, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.frontend.devicecontrol.DeviceControlResponse.displayName = 'proto.frontend.devicecontrol.DeviceControlResponse';
}



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
proto.frontend.devicecontrol.DeviceControlRequest.prototype.toObject = function(opt_includeInstance) {
  return proto.frontend.devicecontrol.DeviceControlRequest.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.frontend.devicecontrol.DeviceControlRequest} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.frontend.devicecontrol.DeviceControlRequest.toObject = function(includeInstance, msg) {
  var f, obj = {
    deviceUuid: jspb.Message.getFieldWithDefault(msg, 1, ""),
    capability: jspb.Message.getFieldWithDefault(msg, 2, "")
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
 * @return {!proto.frontend.devicecontrol.DeviceControlRequest}
 */
proto.frontend.devicecontrol.DeviceControlRequest.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.frontend.devicecontrol.DeviceControlRequest;
  return proto.frontend.devicecontrol.DeviceControlRequest.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.frontend.devicecontrol.DeviceControlRequest} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.frontend.devicecontrol.DeviceControlRequest}
 */
proto.frontend.devicecontrol.DeviceControlRequest.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = /** @type {string} */ (reader.readString());
      msg.setDeviceUuid(value);
      break;
    case 2:
      var value = /** @type {string} */ (reader.readString());
      msg.setCapability(value);
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
proto.frontend.devicecontrol.DeviceControlRequest.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.frontend.devicecontrol.DeviceControlRequest.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.frontend.devicecontrol.DeviceControlRequest} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.frontend.devicecontrol.DeviceControlRequest.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getDeviceUuid();
  if (f.length > 0) {
    writer.writeString(
      1,
      f
    );
  }
  f = message.getCapability();
  if (f.length > 0) {
    writer.writeString(
      2,
      f
    );
  }
};


/**
 * optional string device_uuid = 1;
 * @return {string}
 */
proto.frontend.devicecontrol.DeviceControlRequest.prototype.getDeviceUuid = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 1, ""));
};


/**
 * @param {string} value
 * @return {!proto.frontend.devicecontrol.DeviceControlRequest} returns this
 */
proto.frontend.devicecontrol.DeviceControlRequest.prototype.setDeviceUuid = function(value) {
  return jspb.Message.setProto3StringField(this, 1, value);
};


/**
 * optional string capability = 2;
 * @return {string}
 */
proto.frontend.devicecontrol.DeviceControlRequest.prototype.getCapability = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 2, ""));
};


/**
 * @param {string} value
 * @return {!proto.frontend.devicecontrol.DeviceControlRequest} returns this
 */
proto.frontend.devicecontrol.DeviceControlRequest.prototype.setCapability = function(value) {
  return jspb.Message.setProto3StringField(this, 2, value);
};





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
proto.frontend.devicecontrol.DeviceControlResponse.prototype.toObject = function(opt_includeInstance) {
  return proto.frontend.devicecontrol.DeviceControlResponse.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.frontend.devicecontrol.DeviceControlResponse} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.frontend.devicecontrol.DeviceControlResponse.toObject = function(includeInstance, msg) {
  var f, obj = {
    result: jspb.Message.getFieldWithDefault(msg, 1, 0)
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
 * @return {!proto.frontend.devicecontrol.DeviceControlResponse}
 */
proto.frontend.devicecontrol.DeviceControlResponse.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.frontend.devicecontrol.DeviceControlResponse;
  return proto.frontend.devicecontrol.DeviceControlResponse.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.frontend.devicecontrol.DeviceControlResponse} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.frontend.devicecontrol.DeviceControlResponse}
 */
proto.frontend.devicecontrol.DeviceControlResponse.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = /** @type {!proto.frontend.devicecontrol.DeviceControlResult} */ (reader.readEnum());
      msg.setResult(value);
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
proto.frontend.devicecontrol.DeviceControlResponse.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.frontend.devicecontrol.DeviceControlResponse.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.frontend.devicecontrol.DeviceControlResponse} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.frontend.devicecontrol.DeviceControlResponse.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getResult();
  if (f !== 0.0) {
    writer.writeEnum(
      1,
      f
    );
  }
};


/**
 * optional DeviceControlResult result = 1;
 * @return {!proto.frontend.devicecontrol.DeviceControlResult}
 */
proto.frontend.devicecontrol.DeviceControlResponse.prototype.getResult = function() {
  return /** @type {!proto.frontend.devicecontrol.DeviceControlResult} */ (jspb.Message.getFieldWithDefault(this, 1, 0));
};


/**
 * @param {!proto.frontend.devicecontrol.DeviceControlResult} value
 * @return {!proto.frontend.devicecontrol.DeviceControlResponse} returns this
 */
proto.frontend.devicecontrol.DeviceControlResponse.prototype.setResult = function(value) {
  return jspb.Message.setProto3EnumField(this, 1, value);
};


/**
 * @enum {number}
 */
proto.frontend.devicecontrol.DeviceControlResult = {
  UNKNOWN: 0,
  SUCCESS: 1
};

goog.object.extend(exports, proto.frontend.devicecontrol);
