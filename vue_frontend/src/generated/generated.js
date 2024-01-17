/*eslint-disable block-scoped-var, id-length, no-control-regex, no-magic-numbers, no-prototype-builtins, no-redeclare, no-shadow, no-var, sort-vars*/
import * as $protobuf from "protobufjs/minimal";

// Common aliases
const $Reader = $protobuf.Reader,
    $Writer = $protobuf.Writer,
    $util = $protobuf.util;

// Exported root namespace
const $root = $protobuf.roots["default"] || ($protobuf.roots["default"] = {});

export const frontend = ($root.frontend = (() => {
    /**
     * Namespace frontend.
     * @exports frontend
     * @namespace
     */
    const frontend = {};

    frontend.devicecontrol = (function () {
        /**
         * Namespace devicecontrol.
         * @memberof frontend
         * @namespace
         */
        const devicecontrol = {};

        devicecontrol.FrontendDeviceControlService = (function () {
            /**
             * Constructs a new FrontendDeviceControlService service.
             * @memberof frontend.devicecontrol
             * @classdesc Represents a FrontendDeviceControlService
             * @extends $protobuf.rpc.Service
             * @constructor
             * @param {$protobuf.RPCImpl} rpcImpl RPC implementation
             * @param {boolean} [requestDelimited=false] Whether requests are length-delimited
             * @param {boolean} [responseDelimited=false] Whether responses are length-delimited
             */
            function FrontendDeviceControlService(
                rpcImpl,
                requestDelimited,
                responseDelimited,
            ) {
                $protobuf.rpc.Service.call(
                    this,
                    rpcImpl,
                    requestDelimited,
                    responseDelimited,
                );
            }

            (FrontendDeviceControlService.prototype = Object.create(
                $protobuf.rpc.Service.prototype,
            )).constructor = FrontendDeviceControlService;

            /**
             * Creates new FrontendDeviceControlService service using the specified rpc implementation.
             * @function create
             * @memberof frontend.devicecontrol.FrontendDeviceControlService
             * @static
             * @param {$protobuf.RPCImpl} rpcImpl RPC implementation
             * @param {boolean} [requestDelimited=false] Whether requests are length-delimited
             * @param {boolean} [responseDelimited=false] Whether responses are length-delimited
             * @returns {FrontendDeviceControlService} RPC service. Useful where requests and/or responses are streamed.
             */
            FrontendDeviceControlService.create = function create(
                rpcImpl,
                requestDelimited,
                responseDelimited,
            ) {
                return new this(rpcImpl, requestDelimited, responseDelimited);
            };

            /**
             * Callback as used by {@link frontend.devicecontrol.FrontendDeviceControlService#controlDevice}.
             * @memberof frontend.devicecontrol.FrontendDeviceControlService
             * @typedef ControlDeviceCallback
             * @type {function}
             * @param {Error|null} error Error, if any
             * @param {frontend.devicecontrol.DeviceControlResponse} [response] DeviceControlResponse
             */

            /**
             * Calls ControlDevice.
             * @function controlDevice
             * @memberof frontend.devicecontrol.FrontendDeviceControlService
             * @instance
             * @param {frontend.devicecontrol.IDeviceControlRequest} request DeviceControlRequest message or plain object
             * @param {frontend.devicecontrol.FrontendDeviceControlService.ControlDeviceCallback} callback Node-style callback called with the error, if any, and DeviceControlResponse
             * @returns {undefined}
             * @variation 1
             */
            Object.defineProperty(
                (FrontendDeviceControlService.prototype.controlDevice =
                    function controlDevice(request, callback) {
                        return this.rpcCall(
                            controlDevice,
                            $root.frontend.devicecontrol.DeviceControlRequest,
                            $root.frontend.devicecontrol.DeviceControlResponse,
                            request,
                            callback,
                        );
                    }),
                "name",
                { value: "ControlDevice" },
            );

            /**
             * Calls ControlDevice.
             * @function controlDevice
             * @memberof frontend.devicecontrol.FrontendDeviceControlService
             * @instance
             * @param {frontend.devicecontrol.IDeviceControlRequest} request DeviceControlRequest message or plain object
             * @returns {Promise<frontend.devicecontrol.DeviceControlResponse>} Promise
             * @variation 2
             */

            return FrontendDeviceControlService;
        })();

        devicecontrol.DeviceControlRequest = (function () {
            /**
             * Properties of a DeviceControlRequest.
             * @memberof frontend.devicecontrol
             * @interface IDeviceControlRequest
             * @property {string|null} [deviceUuid] DeviceControlRequest deviceUuid
             * @property {string|null} [capability] DeviceControlRequest capability
             */

            /**
             * Constructs a new DeviceControlRequest.
             * @memberof frontend.devicecontrol
             * @classdesc Represents a DeviceControlRequest.
             * @implements IDeviceControlRequest
             * @constructor
             * @param {frontend.devicecontrol.IDeviceControlRequest=} [properties] Properties to set
             */
            function DeviceControlRequest(properties) {
                if (properties)
                    for (
                        let keys = Object.keys(properties), i = 0;
                        i < keys.length;
                        ++i
                    )
                        if (properties[keys[i]] != null)
                            this[keys[i]] = properties[keys[i]];
            }

            /**
             * DeviceControlRequest deviceUuid.
             * @member {string} deviceUuid
             * @memberof frontend.devicecontrol.DeviceControlRequest
             * @instance
             */
            DeviceControlRequest.prototype.deviceUuid = "";

            /**
             * DeviceControlRequest capability.
             * @member {string} capability
             * @memberof frontend.devicecontrol.DeviceControlRequest
             * @instance
             */
            DeviceControlRequest.prototype.capability = "";

            /**
             * Creates a new DeviceControlRequest instance using the specified properties.
             * @function create
             * @memberof frontend.devicecontrol.DeviceControlRequest
             * @static
             * @param {frontend.devicecontrol.IDeviceControlRequest=} [properties] Properties to set
             * @returns {frontend.devicecontrol.DeviceControlRequest} DeviceControlRequest instance
             */
            DeviceControlRequest.create = function create(properties) {
                return new DeviceControlRequest(properties);
            };

            /**
             * Encodes the specified DeviceControlRequest message. Does not implicitly {@link frontend.devicecontrol.DeviceControlRequest.verify|verify} messages.
             * @function encode
             * @memberof frontend.devicecontrol.DeviceControlRequest
             * @static
             * @param {frontend.devicecontrol.IDeviceControlRequest} message DeviceControlRequest message or plain object to encode
             * @param {$protobuf.Writer} [writer] Writer to encode to
             * @returns {$protobuf.Writer} Writer
             */
            DeviceControlRequest.encode = function encode(message, writer) {
                if (!writer) writer = $Writer.create();
                if (
                    message.deviceUuid != null &&
                    Object.hasOwnProperty.call(message, "deviceUuid")
                )
                    writer
                        .uint32(/* id 1, wireType 2 =*/ 10)
                        .string(message.deviceUuid);
                if (
                    message.capability != null &&
                    Object.hasOwnProperty.call(message, "capability")
                )
                    writer
                        .uint32(/* id 2, wireType 2 =*/ 18)
                        .string(message.capability);
                return writer;
            };

            /**
             * Encodes the specified DeviceControlRequest message, length delimited. Does not implicitly {@link frontend.devicecontrol.DeviceControlRequest.verify|verify} messages.
             * @function encodeDelimited
             * @memberof frontend.devicecontrol.DeviceControlRequest
             * @static
             * @param {frontend.devicecontrol.IDeviceControlRequest} message DeviceControlRequest message or plain object to encode
             * @param {$protobuf.Writer} [writer] Writer to encode to
             * @returns {$protobuf.Writer} Writer
             */
            DeviceControlRequest.encodeDelimited = function encodeDelimited(
                message,
                writer,
            ) {
                return this.encode(message, writer).ldelim();
            };

            /**
             * Decodes a DeviceControlRequest message from the specified reader or buffer.
             * @function decode
             * @memberof frontend.devicecontrol.DeviceControlRequest
             * @static
             * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
             * @param {number} [length] Message length if known beforehand
             * @returns {frontend.devicecontrol.DeviceControlRequest} DeviceControlRequest
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            DeviceControlRequest.decode = function decode(reader, length) {
                if (!(reader instanceof $Reader))
                    reader = $Reader.create(reader);
                let end =
                        length === undefined ? reader.len : reader.pos + length,
                    message =
                        new $root.frontend.devicecontrol.DeviceControlRequest();
                while (reader.pos < end) {
                    let tag = reader.uint32();
                    switch (tag >>> 3) {
                        case 1: {
                            message.deviceUuid = reader.string();
                            break;
                        }
                        case 2: {
                            message.capability = reader.string();
                            break;
                        }
                        default:
                            reader.skipType(tag & 7);
                            break;
                    }
                }
                return message;
            };

            /**
             * Decodes a DeviceControlRequest message from the specified reader or buffer, length delimited.
             * @function decodeDelimited
             * @memberof frontend.devicecontrol.DeviceControlRequest
             * @static
             * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
             * @returns {frontend.devicecontrol.DeviceControlRequest} DeviceControlRequest
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            DeviceControlRequest.decodeDelimited = function decodeDelimited(
                reader,
            ) {
                if (!(reader instanceof $Reader)) reader = new $Reader(reader);
                return this.decode(reader, reader.uint32());
            };

            /**
             * Verifies a DeviceControlRequest message.
             * @function verify
             * @memberof frontend.devicecontrol.DeviceControlRequest
             * @static
             * @param {Object.<string,*>} message Plain object to verify
             * @returns {string|null} `null` if valid, otherwise the reason why it is not
             */
            DeviceControlRequest.verify = function verify(message) {
                if (typeof message !== "object" || message === null)
                    return "object expected";
                if (
                    message.deviceUuid != null &&
                    message.hasOwnProperty("deviceUuid")
                )
                    if (!$util.isString(message.deviceUuid))
                        return "deviceUuid: string expected";
                if (
                    message.capability != null &&
                    message.hasOwnProperty("capability")
                )
                    if (!$util.isString(message.capability))
                        return "capability: string expected";
                return null;
            };

            /**
             * Creates a DeviceControlRequest message from a plain object. Also converts values to their respective internal types.
             * @function fromObject
             * @memberof frontend.devicecontrol.DeviceControlRequest
             * @static
             * @param {Object.<string,*>} object Plain object
             * @returns {frontend.devicecontrol.DeviceControlRequest} DeviceControlRequest
             */
            DeviceControlRequest.fromObject = function fromObject(object) {
                if (
                    object instanceof
                    $root.frontend.devicecontrol.DeviceControlRequest
                )
                    return object;
                let message =
                    new $root.frontend.devicecontrol.DeviceControlRequest();
                if (object.deviceUuid != null)
                    message.deviceUuid = String(object.deviceUuid);
                if (object.capability != null)
                    message.capability = String(object.capability);
                return message;
            };

            /**
             * Creates a plain object from a DeviceControlRequest message. Also converts values to other types if specified.
             * @function toObject
             * @memberof frontend.devicecontrol.DeviceControlRequest
             * @static
             * @param {frontend.devicecontrol.DeviceControlRequest} message DeviceControlRequest
             * @param {$protobuf.IConversionOptions} [options] Conversion options
             * @returns {Object.<string,*>} Plain object
             */
            DeviceControlRequest.toObject = function toObject(
                message,
                options,
            ) {
                if (!options) options = {};
                let object = {};
                if (options.defaults) {
                    object.deviceUuid = "";
                    object.capability = "";
                }
                if (
                    message.deviceUuid != null &&
                    message.hasOwnProperty("deviceUuid")
                )
                    object.deviceUuid = message.deviceUuid;
                if (
                    message.capability != null &&
                    message.hasOwnProperty("capability")
                )
                    object.capability = message.capability;
                return object;
            };

            /**
             * Converts this DeviceControlRequest to JSON.
             * @function toJSON
             * @memberof frontend.devicecontrol.DeviceControlRequest
             * @instance
             * @returns {Object.<string,*>} JSON object
             */
            DeviceControlRequest.prototype.toJSON = function toJSON() {
                return this.constructor.toObject(
                    this,
                    $protobuf.util.toJSONOptions,
                );
            };

            /**
             * Gets the default type url for DeviceControlRequest
             * @function getTypeUrl
             * @memberof frontend.devicecontrol.DeviceControlRequest
             * @static
             * @param {string} [typeUrlPrefix] your custom typeUrlPrefix(default "type.googleapis.com")
             * @returns {string} The default type url
             */
            DeviceControlRequest.getTypeUrl = function getTypeUrl(
                typeUrlPrefix,
            ) {
                if (typeUrlPrefix === undefined) {
                    typeUrlPrefix = "type.googleapis.com";
                }
                return (
                    typeUrlPrefix +
                    "/frontend.devicecontrol.DeviceControlRequest"
                );
            };

            return DeviceControlRequest;
        })();

        devicecontrol.DeviceControlResponse = (function () {
            /**
             * Properties of a DeviceControlResponse.
             * @memberof frontend.devicecontrol
             * @interface IDeviceControlResponse
             * @property {frontend.devicecontrol.DeviceControlResult|null} [result] DeviceControlResponse result
             */

            /**
             * Constructs a new DeviceControlResponse.
             * @memberof frontend.devicecontrol
             * @classdesc Represents a DeviceControlResponse.
             * @implements IDeviceControlResponse
             * @constructor
             * @param {frontend.devicecontrol.IDeviceControlResponse=} [properties] Properties to set
             */
            function DeviceControlResponse(properties) {
                if (properties)
                    for (
                        let keys = Object.keys(properties), i = 0;
                        i < keys.length;
                        ++i
                    )
                        if (properties[keys[i]] != null)
                            this[keys[i]] = properties[keys[i]];
            }

            /**
             * DeviceControlResponse result.
             * @member {frontend.devicecontrol.DeviceControlResult} result
             * @memberof frontend.devicecontrol.DeviceControlResponse
             * @instance
             */
            DeviceControlResponse.prototype.result = 0;

            /**
             * Creates a new DeviceControlResponse instance using the specified properties.
             * @function create
             * @memberof frontend.devicecontrol.DeviceControlResponse
             * @static
             * @param {frontend.devicecontrol.IDeviceControlResponse=} [properties] Properties to set
             * @returns {frontend.devicecontrol.DeviceControlResponse} DeviceControlResponse instance
             */
            DeviceControlResponse.create = function create(properties) {
                return new DeviceControlResponse(properties);
            };

            /**
             * Encodes the specified DeviceControlResponse message. Does not implicitly {@link frontend.devicecontrol.DeviceControlResponse.verify|verify} messages.
             * @function encode
             * @memberof frontend.devicecontrol.DeviceControlResponse
             * @static
             * @param {frontend.devicecontrol.IDeviceControlResponse} message DeviceControlResponse message or plain object to encode
             * @param {$protobuf.Writer} [writer] Writer to encode to
             * @returns {$protobuf.Writer} Writer
             */
            DeviceControlResponse.encode = function encode(message, writer) {
                if (!writer) writer = $Writer.create();
                if (
                    message.result != null &&
                    Object.hasOwnProperty.call(message, "result")
                )
                    writer
                        .uint32(/* id 1, wireType 0 =*/ 8)
                        .int32(message.result);
                return writer;
            };

            /**
             * Encodes the specified DeviceControlResponse message, length delimited. Does not implicitly {@link frontend.devicecontrol.DeviceControlResponse.verify|verify} messages.
             * @function encodeDelimited
             * @memberof frontend.devicecontrol.DeviceControlResponse
             * @static
             * @param {frontend.devicecontrol.IDeviceControlResponse} message DeviceControlResponse message or plain object to encode
             * @param {$protobuf.Writer} [writer] Writer to encode to
             * @returns {$protobuf.Writer} Writer
             */
            DeviceControlResponse.encodeDelimited = function encodeDelimited(
                message,
                writer,
            ) {
                return this.encode(message, writer).ldelim();
            };

            /**
             * Decodes a DeviceControlResponse message from the specified reader or buffer.
             * @function decode
             * @memberof frontend.devicecontrol.DeviceControlResponse
             * @static
             * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
             * @param {number} [length] Message length if known beforehand
             * @returns {frontend.devicecontrol.DeviceControlResponse} DeviceControlResponse
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            DeviceControlResponse.decode = function decode(reader, length) {
                if (!(reader instanceof $Reader))
                    reader = $Reader.create(reader);
                let end =
                        length === undefined ? reader.len : reader.pos + length,
                    message =
                        new $root.frontend.devicecontrol.DeviceControlResponse();
                while (reader.pos < end) {
                    let tag = reader.uint32();
                    switch (tag >>> 3) {
                        case 1: {
                            message.result = reader.int32();
                            break;
                        }
                        default:
                            reader.skipType(tag & 7);
                            break;
                    }
                }
                return message;
            };

            /**
             * Decodes a DeviceControlResponse message from the specified reader or buffer, length delimited.
             * @function decodeDelimited
             * @memberof frontend.devicecontrol.DeviceControlResponse
             * @static
             * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
             * @returns {frontend.devicecontrol.DeviceControlResponse} DeviceControlResponse
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            DeviceControlResponse.decodeDelimited = function decodeDelimited(
                reader,
            ) {
                if (!(reader instanceof $Reader)) reader = new $Reader(reader);
                return this.decode(reader, reader.uint32());
            };

            /**
             * Verifies a DeviceControlResponse message.
             * @function verify
             * @memberof frontend.devicecontrol.DeviceControlResponse
             * @static
             * @param {Object.<string,*>} message Plain object to verify
             * @returns {string|null} `null` if valid, otherwise the reason why it is not
             */
            DeviceControlResponse.verify = function verify(message) {
                if (typeof message !== "object" || message === null)
                    return "object expected";
                if (message.result != null && message.hasOwnProperty("result"))
                    switch (message.result) {
                        default:
                            return "result: enum value expected";
                        case 0:
                        case 1:
                            break;
                    }
                return null;
            };

            /**
             * Creates a DeviceControlResponse message from a plain object. Also converts values to their respective internal types.
             * @function fromObject
             * @memberof frontend.devicecontrol.DeviceControlResponse
             * @static
             * @param {Object.<string,*>} object Plain object
             * @returns {frontend.devicecontrol.DeviceControlResponse} DeviceControlResponse
             */
            DeviceControlResponse.fromObject = function fromObject(object) {
                if (
                    object instanceof
                    $root.frontend.devicecontrol.DeviceControlResponse
                )
                    return object;
                let message =
                    new $root.frontend.devicecontrol.DeviceControlResponse();
                switch (object.result) {
                    default:
                        if (typeof object.result === "number") {
                            message.result = object.result;
                            break;
                        }
                        break;
                    case "UNKNOWN":
                    case 0:
                        message.result = 0;
                        break;
                    case "SUCCESS":
                    case 1:
                        message.result = 1;
                        break;
                }
                return message;
            };

            /**
             * Creates a plain object from a DeviceControlResponse message. Also converts values to other types if specified.
             * @function toObject
             * @memberof frontend.devicecontrol.DeviceControlResponse
             * @static
             * @param {frontend.devicecontrol.DeviceControlResponse} message DeviceControlResponse
             * @param {$protobuf.IConversionOptions} [options] Conversion options
             * @returns {Object.<string,*>} Plain object
             */
            DeviceControlResponse.toObject = function toObject(
                message,
                options,
            ) {
                if (!options) options = {};
                let object = {};
                if (options.defaults)
                    object.result = options.enums === String ? "UNKNOWN" : 0;
                if (message.result != null && message.hasOwnProperty("result"))
                    object.result =
                        options.enums === String
                            ? $root.frontend.devicecontrol.DeviceControlResult[
                                  message.result
                              ] === undefined
                                ? message.result
                                : $root.frontend.devicecontrol
                                      .DeviceControlResult[message.result]
                            : message.result;
                return object;
            };

            /**
             * Converts this DeviceControlResponse to JSON.
             * @function toJSON
             * @memberof frontend.devicecontrol.DeviceControlResponse
             * @instance
             * @returns {Object.<string,*>} JSON object
             */
            DeviceControlResponse.prototype.toJSON = function toJSON() {
                return this.constructor.toObject(
                    this,
                    $protobuf.util.toJSONOptions,
                );
            };

            /**
             * Gets the default type url for DeviceControlResponse
             * @function getTypeUrl
             * @memberof frontend.devicecontrol.DeviceControlResponse
             * @static
             * @param {string} [typeUrlPrefix] your custom typeUrlPrefix(default "type.googleapis.com")
             * @returns {string} The default type url
             */
            DeviceControlResponse.getTypeUrl = function getTypeUrl(
                typeUrlPrefix,
            ) {
                if (typeUrlPrefix === undefined) {
                    typeUrlPrefix = "type.googleapis.com";
                }
                return (
                    typeUrlPrefix +
                    "/frontend.devicecontrol.DeviceControlResponse"
                );
            };

            return DeviceControlResponse;
        })();

        /**
         * DeviceControlResult enum.
         * @name frontend.devicecontrol.DeviceControlResult
         * @enum {number}
         * @property {number} UNKNOWN=0 UNKNOWN value
         * @property {number} SUCCESS=1 SUCCESS value
         */
        devicecontrol.DeviceControlResult = (function () {
            const valuesById = {},
                values = Object.create(valuesById);
            values[(valuesById[0] = "UNKNOWN")] = 0;
            values[(valuesById[1] = "SUCCESS")] = 1;
            return values;
        })();

        return devicecontrol;
    })();

    frontend.types = (function () {
        /**
         * Namespace types.
         * @memberof frontend
         * @namespace
         */
        const types = {};

        types.DeviceCapabilityStatus = (function () {
            /**
             * Properties of a DeviceCapabilityStatus.
             * @memberof frontend.types
             * @interface IDeviceCapabilityStatus
             * @property {boolean|null} [available] DeviceCapabilityStatus available
             * @property {string|null} [capability] DeviceCapabilityStatus capability
             */

            /**
             * Constructs a new DeviceCapabilityStatus.
             * @memberof frontend.types
             * @classdesc Represents a DeviceCapabilityStatus.
             * @implements IDeviceCapabilityStatus
             * @constructor
             * @param {frontend.types.IDeviceCapabilityStatus=} [properties] Properties to set
             */
            function DeviceCapabilityStatus(properties) {
                if (properties)
                    for (
                        let keys = Object.keys(properties), i = 0;
                        i < keys.length;
                        ++i
                    )
                        if (properties[keys[i]] != null)
                            this[keys[i]] = properties[keys[i]];
            }

            /**
             * DeviceCapabilityStatus available.
             * @member {boolean} available
             * @memberof frontend.types.DeviceCapabilityStatus
             * @instance
             */
            DeviceCapabilityStatus.prototype.available = false;

            /**
             * DeviceCapabilityStatus capability.
             * @member {string} capability
             * @memberof frontend.types.DeviceCapabilityStatus
             * @instance
             */
            DeviceCapabilityStatus.prototype.capability = "";

            /**
             * Creates a new DeviceCapabilityStatus instance using the specified properties.
             * @function create
             * @memberof frontend.types.DeviceCapabilityStatus
             * @static
             * @param {frontend.types.IDeviceCapabilityStatus=} [properties] Properties to set
             * @returns {frontend.types.DeviceCapabilityStatus} DeviceCapabilityStatus instance
             */
            DeviceCapabilityStatus.create = function create(properties) {
                return new DeviceCapabilityStatus(properties);
            };

            /**
             * Encodes the specified DeviceCapabilityStatus message. Does not implicitly {@link frontend.types.DeviceCapabilityStatus.verify|verify} messages.
             * @function encode
             * @memberof frontend.types.DeviceCapabilityStatus
             * @static
             * @param {frontend.types.IDeviceCapabilityStatus} message DeviceCapabilityStatus message or plain object to encode
             * @param {$protobuf.Writer} [writer] Writer to encode to
             * @returns {$protobuf.Writer} Writer
             */
            DeviceCapabilityStatus.encode = function encode(message, writer) {
                if (!writer) writer = $Writer.create();
                if (
                    message.available != null &&
                    Object.hasOwnProperty.call(message, "available")
                )
                    writer
                        .uint32(/* id 1, wireType 0 =*/ 8)
                        .bool(message.available);
                if (
                    message.capability != null &&
                    Object.hasOwnProperty.call(message, "capability")
                )
                    writer
                        .uint32(/* id 2, wireType 2 =*/ 18)
                        .string(message.capability);
                return writer;
            };

            /**
             * Encodes the specified DeviceCapabilityStatus message, length delimited. Does not implicitly {@link frontend.types.DeviceCapabilityStatus.verify|verify} messages.
             * @function encodeDelimited
             * @memberof frontend.types.DeviceCapabilityStatus
             * @static
             * @param {frontend.types.IDeviceCapabilityStatus} message DeviceCapabilityStatus message or plain object to encode
             * @param {$protobuf.Writer} [writer] Writer to encode to
             * @returns {$protobuf.Writer} Writer
             */
            DeviceCapabilityStatus.encodeDelimited = function encodeDelimited(
                message,
                writer,
            ) {
                return this.encode(message, writer).ldelim();
            };

            /**
             * Decodes a DeviceCapabilityStatus message from the specified reader or buffer.
             * @function decode
             * @memberof frontend.types.DeviceCapabilityStatus
             * @static
             * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
             * @param {number} [length] Message length if known beforehand
             * @returns {frontend.types.DeviceCapabilityStatus} DeviceCapabilityStatus
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            DeviceCapabilityStatus.decode = function decode(reader, length) {
                if (!(reader instanceof $Reader))
                    reader = $Reader.create(reader);
                let end =
                        length === undefined ? reader.len : reader.pos + length,
                    message = new $root.frontend.types.DeviceCapabilityStatus();
                while (reader.pos < end) {
                    let tag = reader.uint32();
                    switch (tag >>> 3) {
                        case 1: {
                            message.available = reader.bool();
                            break;
                        }
                        case 2: {
                            message.capability = reader.string();
                            break;
                        }
                        default:
                            reader.skipType(tag & 7);
                            break;
                    }
                }
                return message;
            };

            /**
             * Decodes a DeviceCapabilityStatus message from the specified reader or buffer, length delimited.
             * @function decodeDelimited
             * @memberof frontend.types.DeviceCapabilityStatus
             * @static
             * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
             * @returns {frontend.types.DeviceCapabilityStatus} DeviceCapabilityStatus
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            DeviceCapabilityStatus.decodeDelimited = function decodeDelimited(
                reader,
            ) {
                if (!(reader instanceof $Reader)) reader = new $Reader(reader);
                return this.decode(reader, reader.uint32());
            };

            /**
             * Verifies a DeviceCapabilityStatus message.
             * @function verify
             * @memberof frontend.types.DeviceCapabilityStatus
             * @static
             * @param {Object.<string,*>} message Plain object to verify
             * @returns {string|null} `null` if valid, otherwise the reason why it is not
             */
            DeviceCapabilityStatus.verify = function verify(message) {
                if (typeof message !== "object" || message === null)
                    return "object expected";
                if (
                    message.available != null &&
                    message.hasOwnProperty("available")
                )
                    if (typeof message.available !== "boolean")
                        return "available: boolean expected";
                if (
                    message.capability != null &&
                    message.hasOwnProperty("capability")
                )
                    if (!$util.isString(message.capability))
                        return "capability: string expected";
                return null;
            };

            /**
             * Creates a DeviceCapabilityStatus message from a plain object. Also converts values to their respective internal types.
             * @function fromObject
             * @memberof frontend.types.DeviceCapabilityStatus
             * @static
             * @param {Object.<string,*>} object Plain object
             * @returns {frontend.types.DeviceCapabilityStatus} DeviceCapabilityStatus
             */
            DeviceCapabilityStatus.fromObject = function fromObject(object) {
                if (
                    object instanceof
                    $root.frontend.types.DeviceCapabilityStatus
                )
                    return object;
                let message = new $root.frontend.types.DeviceCapabilityStatus();
                if (object.available != null)
                    message.available = Boolean(object.available);
                if (object.capability != null)
                    message.capability = String(object.capability);
                return message;
            };

            /**
             * Creates a plain object from a DeviceCapabilityStatus message. Also converts values to other types if specified.
             * @function toObject
             * @memberof frontend.types.DeviceCapabilityStatus
             * @static
             * @param {frontend.types.DeviceCapabilityStatus} message DeviceCapabilityStatus
             * @param {$protobuf.IConversionOptions} [options] Conversion options
             * @returns {Object.<string,*>} Plain object
             */
            DeviceCapabilityStatus.toObject = function toObject(
                message,
                options,
            ) {
                if (!options) options = {};
                let object = {};
                if (options.defaults) {
                    object.available = false;
                    object.capability = "";
                }
                if (
                    message.available != null &&
                    message.hasOwnProperty("available")
                )
                    object.available = message.available;
                if (
                    message.capability != null &&
                    message.hasOwnProperty("capability")
                )
                    object.capability = message.capability;
                return object;
            };

            /**
             * Converts this DeviceCapabilityStatus to JSON.
             * @function toJSON
             * @memberof frontend.types.DeviceCapabilityStatus
             * @instance
             * @returns {Object.<string,*>} JSON object
             */
            DeviceCapabilityStatus.prototype.toJSON = function toJSON() {
                return this.constructor.toObject(
                    this,
                    $protobuf.util.toJSONOptions,
                );
            };

            /**
             * Gets the default type url for DeviceCapabilityStatus
             * @function getTypeUrl
             * @memberof frontend.types.DeviceCapabilityStatus
             * @static
             * @param {string} [typeUrlPrefix] your custom typeUrlPrefix(default "type.googleapis.com")
             * @returns {string} The default type url
             */
            DeviceCapabilityStatus.getTypeUrl = function getTypeUrl(
                typeUrlPrefix,
            ) {
                if (typeUrlPrefix === undefined) {
                    typeUrlPrefix = "type.googleapis.com";
                }
                return typeUrlPrefix + "/frontend.types.DeviceCapabilityStatus";
            };

            return DeviceCapabilityStatus;
        })();

        return types;
    })();

    frontend.registration = (function () {
        /**
         * Namespace registration.
         * @memberof frontend
         * @namespace
         */
        const registration = {};

        registration.FrontendRegistrationService = (function () {
            /**
             * Constructs a new FrontendRegistrationService service.
             * @memberof frontend.registration
             * @classdesc Represents a FrontendRegistrationService
             * @extends $protobuf.rpc.Service
             * @constructor
             * @param {$protobuf.RPCImpl} rpcImpl RPC implementation
             * @param {boolean} [requestDelimited=false] Whether requests are length-delimited
             * @param {boolean} [responseDelimited=false] Whether responses are length-delimited
             */
            function FrontendRegistrationService(
                rpcImpl,
                requestDelimited,
                responseDelimited,
            ) {
                $protobuf.rpc.Service.call(
                    this,
                    rpcImpl,
                    requestDelimited,
                    responseDelimited,
                );
            }

            (FrontendRegistrationService.prototype = Object.create(
                $protobuf.rpc.Service.prototype,
            )).constructor = FrontendRegistrationService;

            /**
             * Creates new FrontendRegistrationService service using the specified rpc implementation.
             * @function create
             * @memberof frontend.registration.FrontendRegistrationService
             * @static
             * @param {$protobuf.RPCImpl} rpcImpl RPC implementation
             * @param {boolean} [requestDelimited=false] Whether requests are length-delimited
             * @param {boolean} [responseDelimited=false] Whether responses are length-delimited
             * @returns {FrontendRegistrationService} RPC service. Useful where requests and/or responses are streamed.
             */
            FrontendRegistrationService.create = function create(
                rpcImpl,
                requestDelimited,
                responseDelimited,
            ) {
                return new this(rpcImpl, requestDelimited, responseDelimited);
            };

            /**
             * Callback as used by {@link frontend.registration.FrontendRegistrationService#register}.
             * @memberof frontend.registration.FrontendRegistrationService
             * @typedef RegisterCallback
             * @type {function}
             * @param {Error|null} error Error, if any
             * @param {frontend.registration.RegistrationResponse} [response] RegistrationResponse
             */

            /**
             * Calls Register.
             * @function register
             * @memberof frontend.registration.FrontendRegistrationService
             * @instance
             * @param {frontend.registration.IRegistrationRequest} request RegistrationRequest message or plain object
             * @param {frontend.registration.FrontendRegistrationService.RegisterCallback} callback Node-style callback called with the error, if any, and RegistrationResponse
             * @returns {undefined}
             * @variation 1
             */
            Object.defineProperty(
                (FrontendRegistrationService.prototype.register =
                    function register(request, callback) {
                        return this.rpcCall(
                            register,
                            $root.frontend.registration.RegistrationRequest,
                            $root.frontend.registration.RegistrationResponse,
                            request,
                            callback,
                        );
                    }),
                "name",
                { value: "Register" },
            );

            /**
             * Calls Register.
             * @function register
             * @memberof frontend.registration.FrontendRegistrationService
             * @instance
             * @param {frontend.registration.IRegistrationRequest} request RegistrationRequest message or plain object
             * @returns {Promise<frontend.registration.RegistrationResponse>} Promise
             * @variation 2
             */

            /**
             * Callback as used by {@link frontend.registration.FrontendRegistrationService#getConnectedDevices}.
             * @memberof frontend.registration.FrontendRegistrationService
             * @typedef GetConnectedDevicesCallback
             * @type {function}
             * @param {Error|null} error Error, if any
             * @param {frontend.registration.ConnectedDevicesResponse} [response] ConnectedDevicesResponse
             */

            /**
             * Calls GetConnectedDevices.
             * @function getConnectedDevices
             * @memberof frontend.registration.FrontendRegistrationService
             * @instance
             * @param {frontend.registration.IConnectedDevicesRequest} request ConnectedDevicesRequest message or plain object
             * @param {frontend.registration.FrontendRegistrationService.GetConnectedDevicesCallback} callback Node-style callback called with the error, if any, and ConnectedDevicesResponse
             * @returns {undefined}
             * @variation 1
             */
            Object.defineProperty(
                (FrontendRegistrationService.prototype.getConnectedDevices =
                    function getConnectedDevices(request, callback) {
                        return this.rpcCall(
                            getConnectedDevices,
                            $root.frontend.registration.ConnectedDevicesRequest,
                            $root.frontend.registration
                                .ConnectedDevicesResponse,
                            request,
                            callback,
                        );
                    }),
                "name",
                { value: "GetConnectedDevices" },
            );

            /**
             * Calls GetConnectedDevices.
             * @function getConnectedDevices
             * @memberof frontend.registration.FrontendRegistrationService
             * @instance
             * @param {frontend.registration.IConnectedDevicesRequest} request ConnectedDevicesRequest message or plain object
             * @returns {Promise<frontend.registration.ConnectedDevicesResponse>} Promise
             * @variation 2
             */

            return FrontendRegistrationService;
        })();

        registration.RegistrationRequest = (function () {
            /**
             * Properties of a RegistrationRequest.
             * @memberof frontend.registration
             * @interface IRegistrationRequest
             * @property {string|null} [deviceName] RegistrationRequest deviceName
             */

            /**
             * Constructs a new RegistrationRequest.
             * @memberof frontend.registration
             * @classdesc Represents a RegistrationRequest.
             * @implements IRegistrationRequest
             * @constructor
             * @param {frontend.registration.IRegistrationRequest=} [properties] Properties to set
             */
            function RegistrationRequest(properties) {
                if (properties)
                    for (
                        let keys = Object.keys(properties), i = 0;
                        i < keys.length;
                        ++i
                    )
                        if (properties[keys[i]] != null)
                            this[keys[i]] = properties[keys[i]];
            }

            /**
             * RegistrationRequest deviceName.
             * @member {string} deviceName
             * @memberof frontend.registration.RegistrationRequest
             * @instance
             */
            RegistrationRequest.prototype.deviceName = "";

            /**
             * Creates a new RegistrationRequest instance using the specified properties.
             * @function create
             * @memberof frontend.registration.RegistrationRequest
             * @static
             * @param {frontend.registration.IRegistrationRequest=} [properties] Properties to set
             * @returns {frontend.registration.RegistrationRequest} RegistrationRequest instance
             */
            RegistrationRequest.create = function create(properties) {
                return new RegistrationRequest(properties);
            };

            /**
             * Encodes the specified RegistrationRequest message. Does not implicitly {@link frontend.registration.RegistrationRequest.verify|verify} messages.
             * @function encode
             * @memberof frontend.registration.RegistrationRequest
             * @static
             * @param {frontend.registration.IRegistrationRequest} message RegistrationRequest message or plain object to encode
             * @param {$protobuf.Writer} [writer] Writer to encode to
             * @returns {$protobuf.Writer} Writer
             */
            RegistrationRequest.encode = function encode(message, writer) {
                if (!writer) writer = $Writer.create();
                if (
                    message.deviceName != null &&
                    Object.hasOwnProperty.call(message, "deviceName")
                )
                    writer
                        .uint32(/* id 1, wireType 2 =*/ 10)
                        .string(message.deviceName);
                return writer;
            };

            /**
             * Encodes the specified RegistrationRequest message, length delimited. Does not implicitly {@link frontend.registration.RegistrationRequest.verify|verify} messages.
             * @function encodeDelimited
             * @memberof frontend.registration.RegistrationRequest
             * @static
             * @param {frontend.registration.IRegistrationRequest} message RegistrationRequest message or plain object to encode
             * @param {$protobuf.Writer} [writer] Writer to encode to
             * @returns {$protobuf.Writer} Writer
             */
            RegistrationRequest.encodeDelimited = function encodeDelimited(
                message,
                writer,
            ) {
                return this.encode(message, writer).ldelim();
            };

            /**
             * Decodes a RegistrationRequest message from the specified reader or buffer.
             * @function decode
             * @memberof frontend.registration.RegistrationRequest
             * @static
             * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
             * @param {number} [length] Message length if known beforehand
             * @returns {frontend.registration.RegistrationRequest} RegistrationRequest
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            RegistrationRequest.decode = function decode(reader, length) {
                if (!(reader instanceof $Reader))
                    reader = $Reader.create(reader);
                let end =
                        length === undefined ? reader.len : reader.pos + length,
                    message =
                        new $root.frontend.registration.RegistrationRequest();
                while (reader.pos < end) {
                    let tag = reader.uint32();
                    switch (tag >>> 3) {
                        case 1: {
                            message.deviceName = reader.string();
                            break;
                        }
                        default:
                            reader.skipType(tag & 7);
                            break;
                    }
                }
                return message;
            };

            /**
             * Decodes a RegistrationRequest message from the specified reader or buffer, length delimited.
             * @function decodeDelimited
             * @memberof frontend.registration.RegistrationRequest
             * @static
             * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
             * @returns {frontend.registration.RegistrationRequest} RegistrationRequest
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            RegistrationRequest.decodeDelimited = function decodeDelimited(
                reader,
            ) {
                if (!(reader instanceof $Reader)) reader = new $Reader(reader);
                return this.decode(reader, reader.uint32());
            };

            /**
             * Verifies a RegistrationRequest message.
             * @function verify
             * @memberof frontend.registration.RegistrationRequest
             * @static
             * @param {Object.<string,*>} message Plain object to verify
             * @returns {string|null} `null` if valid, otherwise the reason why it is not
             */
            RegistrationRequest.verify = function verify(message) {
                if (typeof message !== "object" || message === null)
                    return "object expected";
                if (
                    message.deviceName != null &&
                    message.hasOwnProperty("deviceName")
                )
                    if (!$util.isString(message.deviceName))
                        return "deviceName: string expected";
                return null;
            };

            /**
             * Creates a RegistrationRequest message from a plain object. Also converts values to their respective internal types.
             * @function fromObject
             * @memberof frontend.registration.RegistrationRequest
             * @static
             * @param {Object.<string,*>} object Plain object
             * @returns {frontend.registration.RegistrationRequest} RegistrationRequest
             */
            RegistrationRequest.fromObject = function fromObject(object) {
                if (
                    object instanceof
                    $root.frontend.registration.RegistrationRequest
                )
                    return object;
                let message =
                    new $root.frontend.registration.RegistrationRequest();
                if (object.deviceName != null)
                    message.deviceName = String(object.deviceName);
                return message;
            };

            /**
             * Creates a plain object from a RegistrationRequest message. Also converts values to other types if specified.
             * @function toObject
             * @memberof frontend.registration.RegistrationRequest
             * @static
             * @param {frontend.registration.RegistrationRequest} message RegistrationRequest
             * @param {$protobuf.IConversionOptions} [options] Conversion options
             * @returns {Object.<string,*>} Plain object
             */
            RegistrationRequest.toObject = function toObject(message, options) {
                if (!options) options = {};
                let object = {};
                if (options.defaults) object.deviceName = "";
                if (
                    message.deviceName != null &&
                    message.hasOwnProperty("deviceName")
                )
                    object.deviceName = message.deviceName;
                return object;
            };

            /**
             * Converts this RegistrationRequest to JSON.
             * @function toJSON
             * @memberof frontend.registration.RegistrationRequest
             * @instance
             * @returns {Object.<string,*>} JSON object
             */
            RegistrationRequest.prototype.toJSON = function toJSON() {
                return this.constructor.toObject(
                    this,
                    $protobuf.util.toJSONOptions,
                );
            };

            /**
             * Gets the default type url for RegistrationRequest
             * @function getTypeUrl
             * @memberof frontend.registration.RegistrationRequest
             * @static
             * @param {string} [typeUrlPrefix] your custom typeUrlPrefix(default "type.googleapis.com")
             * @returns {string} The default type url
             */
            RegistrationRequest.getTypeUrl = function getTypeUrl(
                typeUrlPrefix,
            ) {
                if (typeUrlPrefix === undefined) {
                    typeUrlPrefix = "type.googleapis.com";
                }
                return (
                    typeUrlPrefix + "/frontend.registration.RegistrationRequest"
                );
            };

            return RegistrationRequest;
        })();

        registration.RegistrationResponse = (function () {
            /**
             * Properties of a RegistrationResponse.
             * @memberof frontend.registration
             * @interface IRegistrationResponse
             * @property {string|null} [clientId] RegistrationResponse clientId
             */

            /**
             * Constructs a new RegistrationResponse.
             * @memberof frontend.registration
             * @classdesc Represents a RegistrationResponse.
             * @implements IRegistrationResponse
             * @constructor
             * @param {frontend.registration.IRegistrationResponse=} [properties] Properties to set
             */
            function RegistrationResponse(properties) {
                if (properties)
                    for (
                        let keys = Object.keys(properties), i = 0;
                        i < keys.length;
                        ++i
                    )
                        if (properties[keys[i]] != null)
                            this[keys[i]] = properties[keys[i]];
            }

            /**
             * RegistrationResponse clientId.
             * @member {string} clientId
             * @memberof frontend.registration.RegistrationResponse
             * @instance
             */
            RegistrationResponse.prototype.clientId = "";

            /**
             * Creates a new RegistrationResponse instance using the specified properties.
             * @function create
             * @memberof frontend.registration.RegistrationResponse
             * @static
             * @param {frontend.registration.IRegistrationResponse=} [properties] Properties to set
             * @returns {frontend.registration.RegistrationResponse} RegistrationResponse instance
             */
            RegistrationResponse.create = function create(properties) {
                return new RegistrationResponse(properties);
            };

            /**
             * Encodes the specified RegistrationResponse message. Does not implicitly {@link frontend.registration.RegistrationResponse.verify|verify} messages.
             * @function encode
             * @memberof frontend.registration.RegistrationResponse
             * @static
             * @param {frontend.registration.IRegistrationResponse} message RegistrationResponse message or plain object to encode
             * @param {$protobuf.Writer} [writer] Writer to encode to
             * @returns {$protobuf.Writer} Writer
             */
            RegistrationResponse.encode = function encode(message, writer) {
                if (!writer) writer = $Writer.create();
                if (
                    message.clientId != null &&
                    Object.hasOwnProperty.call(message, "clientId")
                )
                    writer
                        .uint32(/* id 2, wireType 2 =*/ 18)
                        .string(message.clientId);
                return writer;
            };

            /**
             * Encodes the specified RegistrationResponse message, length delimited. Does not implicitly {@link frontend.registration.RegistrationResponse.verify|verify} messages.
             * @function encodeDelimited
             * @memberof frontend.registration.RegistrationResponse
             * @static
             * @param {frontend.registration.IRegistrationResponse} message RegistrationResponse message or plain object to encode
             * @param {$protobuf.Writer} [writer] Writer to encode to
             * @returns {$protobuf.Writer} Writer
             */
            RegistrationResponse.encodeDelimited = function encodeDelimited(
                message,
                writer,
            ) {
                return this.encode(message, writer).ldelim();
            };

            /**
             * Decodes a RegistrationResponse message from the specified reader or buffer.
             * @function decode
             * @memberof frontend.registration.RegistrationResponse
             * @static
             * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
             * @param {number} [length] Message length if known beforehand
             * @returns {frontend.registration.RegistrationResponse} RegistrationResponse
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            RegistrationResponse.decode = function decode(reader, length) {
                if (!(reader instanceof $Reader))
                    reader = $Reader.create(reader);
                let end =
                        length === undefined ? reader.len : reader.pos + length,
                    message =
                        new $root.frontend.registration.RegistrationResponse();
                while (reader.pos < end) {
                    let tag = reader.uint32();
                    switch (tag >>> 3) {
                        case 2: {
                            message.clientId = reader.string();
                            break;
                        }
                        default:
                            reader.skipType(tag & 7);
                            break;
                    }
                }
                return message;
            };

            /**
             * Decodes a RegistrationResponse message from the specified reader or buffer, length delimited.
             * @function decodeDelimited
             * @memberof frontend.registration.RegistrationResponse
             * @static
             * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
             * @returns {frontend.registration.RegistrationResponse} RegistrationResponse
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            RegistrationResponse.decodeDelimited = function decodeDelimited(
                reader,
            ) {
                if (!(reader instanceof $Reader)) reader = new $Reader(reader);
                return this.decode(reader, reader.uint32());
            };

            /**
             * Verifies a RegistrationResponse message.
             * @function verify
             * @memberof frontend.registration.RegistrationResponse
             * @static
             * @param {Object.<string,*>} message Plain object to verify
             * @returns {string|null} `null` if valid, otherwise the reason why it is not
             */
            RegistrationResponse.verify = function verify(message) {
                if (typeof message !== "object" || message === null)
                    return "object expected";
                if (
                    message.clientId != null &&
                    message.hasOwnProperty("clientId")
                )
                    if (!$util.isString(message.clientId))
                        return "clientId: string expected";
                return null;
            };

            /**
             * Creates a RegistrationResponse message from a plain object. Also converts values to their respective internal types.
             * @function fromObject
             * @memberof frontend.registration.RegistrationResponse
             * @static
             * @param {Object.<string,*>} object Plain object
             * @returns {frontend.registration.RegistrationResponse} RegistrationResponse
             */
            RegistrationResponse.fromObject = function fromObject(object) {
                if (
                    object instanceof
                    $root.frontend.registration.RegistrationResponse
                )
                    return object;
                let message =
                    new $root.frontend.registration.RegistrationResponse();
                if (object.clientId != null)
                    message.clientId = String(object.clientId);
                return message;
            };

            /**
             * Creates a plain object from a RegistrationResponse message. Also converts values to other types if specified.
             * @function toObject
             * @memberof frontend.registration.RegistrationResponse
             * @static
             * @param {frontend.registration.RegistrationResponse} message RegistrationResponse
             * @param {$protobuf.IConversionOptions} [options] Conversion options
             * @returns {Object.<string,*>} Plain object
             */
            RegistrationResponse.toObject = function toObject(
                message,
                options,
            ) {
                if (!options) options = {};
                let object = {};
                if (options.defaults) object.clientId = "";
                if (
                    message.clientId != null &&
                    message.hasOwnProperty("clientId")
                )
                    object.clientId = message.clientId;
                return object;
            };

            /**
             * Converts this RegistrationResponse to JSON.
             * @function toJSON
             * @memberof frontend.registration.RegistrationResponse
             * @instance
             * @returns {Object.<string,*>} JSON object
             */
            RegistrationResponse.prototype.toJSON = function toJSON() {
                return this.constructor.toObject(
                    this,
                    $protobuf.util.toJSONOptions,
                );
            };

            /**
             * Gets the default type url for RegistrationResponse
             * @function getTypeUrl
             * @memberof frontend.registration.RegistrationResponse
             * @static
             * @param {string} [typeUrlPrefix] your custom typeUrlPrefix(default "type.googleapis.com")
             * @returns {string} The default type url
             */
            RegistrationResponse.getTypeUrl = function getTypeUrl(
                typeUrlPrefix,
            ) {
                if (typeUrlPrefix === undefined) {
                    typeUrlPrefix = "type.googleapis.com";
                }
                return (
                    typeUrlPrefix +
                    "/frontend.registration.RegistrationResponse"
                );
            };

            return RegistrationResponse;
        })();

        registration.ConnectedDevicesRequest = (function () {
            /**
             * Properties of a ConnectedDevicesRequest.
             * @memberof frontend.registration
             * @interface IConnectedDevicesRequest
             * @property {string|null} [clientId] ConnectedDevicesRequest clientId
             */

            /**
             * Constructs a new ConnectedDevicesRequest.
             * @memberof frontend.registration
             * @classdesc Represents a ConnectedDevicesRequest.
             * @implements IConnectedDevicesRequest
             * @constructor
             * @param {frontend.registration.IConnectedDevicesRequest=} [properties] Properties to set
             */
            function ConnectedDevicesRequest(properties) {
                if (properties)
                    for (
                        let keys = Object.keys(properties), i = 0;
                        i < keys.length;
                        ++i
                    )
                        if (properties[keys[i]] != null)
                            this[keys[i]] = properties[keys[i]];
            }

            /**
             * ConnectedDevicesRequest clientId.
             * @member {string} clientId
             * @memberof frontend.registration.ConnectedDevicesRequest
             * @instance
             */
            ConnectedDevicesRequest.prototype.clientId = "";

            /**
             * Creates a new ConnectedDevicesRequest instance using the specified properties.
             * @function create
             * @memberof frontend.registration.ConnectedDevicesRequest
             * @static
             * @param {frontend.registration.IConnectedDevicesRequest=} [properties] Properties to set
             * @returns {frontend.registration.ConnectedDevicesRequest} ConnectedDevicesRequest instance
             */
            ConnectedDevicesRequest.create = function create(properties) {
                return new ConnectedDevicesRequest(properties);
            };

            /**
             * Encodes the specified ConnectedDevicesRequest message. Does not implicitly {@link frontend.registration.ConnectedDevicesRequest.verify|verify} messages.
             * @function encode
             * @memberof frontend.registration.ConnectedDevicesRequest
             * @static
             * @param {frontend.registration.IConnectedDevicesRequest} message ConnectedDevicesRequest message or plain object to encode
             * @param {$protobuf.Writer} [writer] Writer to encode to
             * @returns {$protobuf.Writer} Writer
             */
            ConnectedDevicesRequest.encode = function encode(message, writer) {
                if (!writer) writer = $Writer.create();
                if (
                    message.clientId != null &&
                    Object.hasOwnProperty.call(message, "clientId")
                )
                    writer
                        .uint32(/* id 1, wireType 2 =*/ 10)
                        .string(message.clientId);
                return writer;
            };

            /**
             * Encodes the specified ConnectedDevicesRequest message, length delimited. Does not implicitly {@link frontend.registration.ConnectedDevicesRequest.verify|verify} messages.
             * @function encodeDelimited
             * @memberof frontend.registration.ConnectedDevicesRequest
             * @static
             * @param {frontend.registration.IConnectedDevicesRequest} message ConnectedDevicesRequest message or plain object to encode
             * @param {$protobuf.Writer} [writer] Writer to encode to
             * @returns {$protobuf.Writer} Writer
             */
            ConnectedDevicesRequest.encodeDelimited = function encodeDelimited(
                message,
                writer,
            ) {
                return this.encode(message, writer).ldelim();
            };

            /**
             * Decodes a ConnectedDevicesRequest message from the specified reader or buffer.
             * @function decode
             * @memberof frontend.registration.ConnectedDevicesRequest
             * @static
             * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
             * @param {number} [length] Message length if known beforehand
             * @returns {frontend.registration.ConnectedDevicesRequest} ConnectedDevicesRequest
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            ConnectedDevicesRequest.decode = function decode(reader, length) {
                if (!(reader instanceof $Reader))
                    reader = $Reader.create(reader);
                let end =
                        length === undefined ? reader.len : reader.pos + length,
                    message =
                        new $root.frontend.registration.ConnectedDevicesRequest();
                while (reader.pos < end) {
                    let tag = reader.uint32();
                    switch (tag >>> 3) {
                        case 1: {
                            message.clientId = reader.string();
                            break;
                        }
                        default:
                            reader.skipType(tag & 7);
                            break;
                    }
                }
                return message;
            };

            /**
             * Decodes a ConnectedDevicesRequest message from the specified reader or buffer, length delimited.
             * @function decodeDelimited
             * @memberof frontend.registration.ConnectedDevicesRequest
             * @static
             * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
             * @returns {frontend.registration.ConnectedDevicesRequest} ConnectedDevicesRequest
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            ConnectedDevicesRequest.decodeDelimited = function decodeDelimited(
                reader,
            ) {
                if (!(reader instanceof $Reader)) reader = new $Reader(reader);
                return this.decode(reader, reader.uint32());
            };

            /**
             * Verifies a ConnectedDevicesRequest message.
             * @function verify
             * @memberof frontend.registration.ConnectedDevicesRequest
             * @static
             * @param {Object.<string,*>} message Plain object to verify
             * @returns {string|null} `null` if valid, otherwise the reason why it is not
             */
            ConnectedDevicesRequest.verify = function verify(message) {
                if (typeof message !== "object" || message === null)
                    return "object expected";
                if (
                    message.clientId != null &&
                    message.hasOwnProperty("clientId")
                )
                    if (!$util.isString(message.clientId))
                        return "clientId: string expected";
                return null;
            };

            /**
             * Creates a ConnectedDevicesRequest message from a plain object. Also converts values to their respective internal types.
             * @function fromObject
             * @memberof frontend.registration.ConnectedDevicesRequest
             * @static
             * @param {Object.<string,*>} object Plain object
             * @returns {frontend.registration.ConnectedDevicesRequest} ConnectedDevicesRequest
             */
            ConnectedDevicesRequest.fromObject = function fromObject(object) {
                if (
                    object instanceof
                    $root.frontend.registration.ConnectedDevicesRequest
                )
                    return object;
                let message =
                    new $root.frontend.registration.ConnectedDevicesRequest();
                if (object.clientId != null)
                    message.clientId = String(object.clientId);
                return message;
            };

            /**
             * Creates a plain object from a ConnectedDevicesRequest message. Also converts values to other types if specified.
             * @function toObject
             * @memberof frontend.registration.ConnectedDevicesRequest
             * @static
             * @param {frontend.registration.ConnectedDevicesRequest} message ConnectedDevicesRequest
             * @param {$protobuf.IConversionOptions} [options] Conversion options
             * @returns {Object.<string,*>} Plain object
             */
            ConnectedDevicesRequest.toObject = function toObject(
                message,
                options,
            ) {
                if (!options) options = {};
                let object = {};
                if (options.defaults) object.clientId = "";
                if (
                    message.clientId != null &&
                    message.hasOwnProperty("clientId")
                )
                    object.clientId = message.clientId;
                return object;
            };

            /**
             * Converts this ConnectedDevicesRequest to JSON.
             * @function toJSON
             * @memberof frontend.registration.ConnectedDevicesRequest
             * @instance
             * @returns {Object.<string,*>} JSON object
             */
            ConnectedDevicesRequest.prototype.toJSON = function toJSON() {
                return this.constructor.toObject(
                    this,
                    $protobuf.util.toJSONOptions,
                );
            };

            /**
             * Gets the default type url for ConnectedDevicesRequest
             * @function getTypeUrl
             * @memberof frontend.registration.ConnectedDevicesRequest
             * @static
             * @param {string} [typeUrlPrefix] your custom typeUrlPrefix(default "type.googleapis.com")
             * @returns {string} The default type url
             */
            ConnectedDevicesRequest.getTypeUrl = function getTypeUrl(
                typeUrlPrefix,
            ) {
                if (typeUrlPrefix === undefined) {
                    typeUrlPrefix = "type.googleapis.com";
                }
                return (
                    typeUrlPrefix +
                    "/frontend.registration.ConnectedDevicesRequest"
                );
            };

            return ConnectedDevicesRequest;
        })();

        registration.ConnectedDevicesResponse = (function () {
            /**
             * Properties of a ConnectedDevicesResponse.
             * @memberof frontend.registration
             * @interface IConnectedDevicesResponse
             * @property {Array.<frontend.registration.IDevice>|null} [devices] ConnectedDevicesResponse devices
             */

            /**
             * Constructs a new ConnectedDevicesResponse.
             * @memberof frontend.registration
             * @classdesc Represents a ConnectedDevicesResponse.
             * @implements IConnectedDevicesResponse
             * @constructor
             * @param {frontend.registration.IConnectedDevicesResponse=} [properties] Properties to set
             */
            function ConnectedDevicesResponse(properties) {
                this.devices = [];
                if (properties)
                    for (
                        let keys = Object.keys(properties), i = 0;
                        i < keys.length;
                        ++i
                    )
                        if (properties[keys[i]] != null)
                            this[keys[i]] = properties[keys[i]];
            }

            /**
             * ConnectedDevicesResponse devices.
             * @member {Array.<frontend.registration.IDevice>} devices
             * @memberof frontend.registration.ConnectedDevicesResponse
             * @instance
             */
            ConnectedDevicesResponse.prototype.devices = $util.emptyArray;

            /**
             * Creates a new ConnectedDevicesResponse instance using the specified properties.
             * @function create
             * @memberof frontend.registration.ConnectedDevicesResponse
             * @static
             * @param {frontend.registration.IConnectedDevicesResponse=} [properties] Properties to set
             * @returns {frontend.registration.ConnectedDevicesResponse} ConnectedDevicesResponse instance
             */
            ConnectedDevicesResponse.create = function create(properties) {
                return new ConnectedDevicesResponse(properties);
            };

            /**
             * Encodes the specified ConnectedDevicesResponse message. Does not implicitly {@link frontend.registration.ConnectedDevicesResponse.verify|verify} messages.
             * @function encode
             * @memberof frontend.registration.ConnectedDevicesResponse
             * @static
             * @param {frontend.registration.IConnectedDevicesResponse} message ConnectedDevicesResponse message or plain object to encode
             * @param {$protobuf.Writer} [writer] Writer to encode to
             * @returns {$protobuf.Writer} Writer
             */
            ConnectedDevicesResponse.encode = function encode(message, writer) {
                if (!writer) writer = $Writer.create();
                if (message.devices != null && message.devices.length)
                    for (let i = 0; i < message.devices.length; ++i)
                        $root.frontend.registration.Device.encode(
                            message.devices[i],
                            writer.uint32(/* id 1, wireType 2 =*/ 10).fork(),
                        ).ldelim();
                return writer;
            };

            /**
             * Encodes the specified ConnectedDevicesResponse message, length delimited. Does not implicitly {@link frontend.registration.ConnectedDevicesResponse.verify|verify} messages.
             * @function encodeDelimited
             * @memberof frontend.registration.ConnectedDevicesResponse
             * @static
             * @param {frontend.registration.IConnectedDevicesResponse} message ConnectedDevicesResponse message or plain object to encode
             * @param {$protobuf.Writer} [writer] Writer to encode to
             * @returns {$protobuf.Writer} Writer
             */
            ConnectedDevicesResponse.encodeDelimited = function encodeDelimited(
                message,
                writer,
            ) {
                return this.encode(message, writer).ldelim();
            };

            /**
             * Decodes a ConnectedDevicesResponse message from the specified reader or buffer.
             * @function decode
             * @memberof frontend.registration.ConnectedDevicesResponse
             * @static
             * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
             * @param {number} [length] Message length if known beforehand
             * @returns {frontend.registration.ConnectedDevicesResponse} ConnectedDevicesResponse
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            ConnectedDevicesResponse.decode = function decode(reader, length) {
                if (!(reader instanceof $Reader))
                    reader = $Reader.create(reader);
                let end =
                        length === undefined ? reader.len : reader.pos + length,
                    message =
                        new $root.frontend.registration.ConnectedDevicesResponse();
                while (reader.pos < end) {
                    let tag = reader.uint32();
                    switch (tag >>> 3) {
                        case 1: {
                            if (!(message.devices && message.devices.length))
                                message.devices = [];
                            message.devices.push(
                                $root.frontend.registration.Device.decode(
                                    reader,
                                    reader.uint32(),
                                ),
                            );
                            break;
                        }
                        default:
                            reader.skipType(tag & 7);
                            break;
                    }
                }
                return message;
            };

            /**
             * Decodes a ConnectedDevicesResponse message from the specified reader or buffer, length delimited.
             * @function decodeDelimited
             * @memberof frontend.registration.ConnectedDevicesResponse
             * @static
             * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
             * @returns {frontend.registration.ConnectedDevicesResponse} ConnectedDevicesResponse
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            ConnectedDevicesResponse.decodeDelimited = function decodeDelimited(
                reader,
            ) {
                if (!(reader instanceof $Reader)) reader = new $Reader(reader);
                return this.decode(reader, reader.uint32());
            };

            /**
             * Verifies a ConnectedDevicesResponse message.
             * @function verify
             * @memberof frontend.registration.ConnectedDevicesResponse
             * @static
             * @param {Object.<string,*>} message Plain object to verify
             * @returns {string|null} `null` if valid, otherwise the reason why it is not
             */
            ConnectedDevicesResponse.verify = function verify(message) {
                if (typeof message !== "object" || message === null)
                    return "object expected";
                if (
                    message.devices != null &&
                    message.hasOwnProperty("devices")
                ) {
                    if (!Array.isArray(message.devices))
                        return "devices: array expected";
                    for (let i = 0; i < message.devices.length; ++i) {
                        let error = $root.frontend.registration.Device.verify(
                            message.devices[i],
                        );
                        if (error) return "devices." + error;
                    }
                }
                return null;
            };

            /**
             * Creates a ConnectedDevicesResponse message from a plain object. Also converts values to their respective internal types.
             * @function fromObject
             * @memberof frontend.registration.ConnectedDevicesResponse
             * @static
             * @param {Object.<string,*>} object Plain object
             * @returns {frontend.registration.ConnectedDevicesResponse} ConnectedDevicesResponse
             */
            ConnectedDevicesResponse.fromObject = function fromObject(object) {
                if (
                    object instanceof
                    $root.frontend.registration.ConnectedDevicesResponse
                )
                    return object;
                let message =
                    new $root.frontend.registration.ConnectedDevicesResponse();
                if (object.devices) {
                    if (!Array.isArray(object.devices))
                        throw TypeError(
                            ".frontend.registration.ConnectedDevicesResponse.devices: array expected",
                        );
                    message.devices = [];
                    for (let i = 0; i < object.devices.length; ++i) {
                        if (typeof object.devices[i] !== "object")
                            throw TypeError(
                                ".frontend.registration.ConnectedDevicesResponse.devices: object expected",
                            );
                        message.devices[i] =
                            $root.frontend.registration.Device.fromObject(
                                object.devices[i],
                            );
                    }
                }
                return message;
            };

            /**
             * Creates a plain object from a ConnectedDevicesResponse message. Also converts values to other types if specified.
             * @function toObject
             * @memberof frontend.registration.ConnectedDevicesResponse
             * @static
             * @param {frontend.registration.ConnectedDevicesResponse} message ConnectedDevicesResponse
             * @param {$protobuf.IConversionOptions} [options] Conversion options
             * @returns {Object.<string,*>} Plain object
             */
            ConnectedDevicesResponse.toObject = function toObject(
                message,
                options,
            ) {
                if (!options) options = {};
                let object = {};
                if (options.arrays || options.defaults) object.devices = [];
                if (message.devices && message.devices.length) {
                    object.devices = [];
                    for (let j = 0; j < message.devices.length; ++j)
                        object.devices[j] =
                            $root.frontend.registration.Device.toObject(
                                message.devices[j],
                                options,
                            );
                }
                return object;
            };

            /**
             * Converts this ConnectedDevicesResponse to JSON.
             * @function toJSON
             * @memberof frontend.registration.ConnectedDevicesResponse
             * @instance
             * @returns {Object.<string,*>} JSON object
             */
            ConnectedDevicesResponse.prototype.toJSON = function toJSON() {
                return this.constructor.toObject(
                    this,
                    $protobuf.util.toJSONOptions,
                );
            };

            /**
             * Gets the default type url for ConnectedDevicesResponse
             * @function getTypeUrl
             * @memberof frontend.registration.ConnectedDevicesResponse
             * @static
             * @param {string} [typeUrlPrefix] your custom typeUrlPrefix(default "type.googleapis.com")
             * @returns {string} The default type url
             */
            ConnectedDevicesResponse.getTypeUrl = function getTypeUrl(
                typeUrlPrefix,
            ) {
                if (typeUrlPrefix === undefined) {
                    typeUrlPrefix = "type.googleapis.com";
                }
                return (
                    typeUrlPrefix +
                    "/frontend.registration.ConnectedDevicesResponse"
                );
            };

            return ConnectedDevicesResponse;
        })();

        registration.Device = (function () {
            /**
             * Properties of a Device.
             * @memberof frontend.registration
             * @interface IDevice
             * @property {string|null} [deviceName] Device deviceName
             * @property {string|null} [deviceUuid] Device deviceUuid
             * @property {Array.<frontend.types.IDeviceCapabilityStatus>|null} [capabilities] Device capabilities
             */

            /**
             * Constructs a new Device.
             * @memberof frontend.registration
             * @classdesc Represents a Device.
             * @implements IDevice
             * @constructor
             * @param {frontend.registration.IDevice=} [properties] Properties to set
             */
            function Device(properties) {
                this.capabilities = [];
                if (properties)
                    for (
                        let keys = Object.keys(properties), i = 0;
                        i < keys.length;
                        ++i
                    )
                        if (properties[keys[i]] != null)
                            this[keys[i]] = properties[keys[i]];
            }

            /**
             * Device deviceName.
             * @member {string} deviceName
             * @memberof frontend.registration.Device
             * @instance
             */
            Device.prototype.deviceName = "";

            /**
             * Device deviceUuid.
             * @member {string} deviceUuid
             * @memberof frontend.registration.Device
             * @instance
             */
            Device.prototype.deviceUuid = "";

            /**
             * Device capabilities.
             * @member {Array.<frontend.types.IDeviceCapabilityStatus>} capabilities
             * @memberof frontend.registration.Device
             * @instance
             */
            Device.prototype.capabilities = $util.emptyArray;

            /**
             * Creates a new Device instance using the specified properties.
             * @function create
             * @memberof frontend.registration.Device
             * @static
             * @param {frontend.registration.IDevice=} [properties] Properties to set
             * @returns {frontend.registration.Device} Device instance
             */
            Device.create = function create(properties) {
                return new Device(properties);
            };

            /**
             * Encodes the specified Device message. Does not implicitly {@link frontend.registration.Device.verify|verify} messages.
             * @function encode
             * @memberof frontend.registration.Device
             * @static
             * @param {frontend.registration.IDevice} message Device message or plain object to encode
             * @param {$protobuf.Writer} [writer] Writer to encode to
             * @returns {$protobuf.Writer} Writer
             */
            Device.encode = function encode(message, writer) {
                if (!writer) writer = $Writer.create();
                if (
                    message.deviceName != null &&
                    Object.hasOwnProperty.call(message, "deviceName")
                )
                    writer
                        .uint32(/* id 1, wireType 2 =*/ 10)
                        .string(message.deviceName);
                if (
                    message.deviceUuid != null &&
                    Object.hasOwnProperty.call(message, "deviceUuid")
                )
                    writer
                        .uint32(/* id 2, wireType 2 =*/ 18)
                        .string(message.deviceUuid);
                if (message.capabilities != null && message.capabilities.length)
                    for (let i = 0; i < message.capabilities.length; ++i)
                        $root.frontend.types.DeviceCapabilityStatus.encode(
                            message.capabilities[i],
                            writer.uint32(/* id 3, wireType 2 =*/ 26).fork(),
                        ).ldelim();
                return writer;
            };

            /**
             * Encodes the specified Device message, length delimited. Does not implicitly {@link frontend.registration.Device.verify|verify} messages.
             * @function encodeDelimited
             * @memberof frontend.registration.Device
             * @static
             * @param {frontend.registration.IDevice} message Device message or plain object to encode
             * @param {$protobuf.Writer} [writer] Writer to encode to
             * @returns {$protobuf.Writer} Writer
             */
            Device.encodeDelimited = function encodeDelimited(message, writer) {
                return this.encode(message, writer).ldelim();
            };

            /**
             * Decodes a Device message from the specified reader or buffer.
             * @function decode
             * @memberof frontend.registration.Device
             * @static
             * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
             * @param {number} [length] Message length if known beforehand
             * @returns {frontend.registration.Device} Device
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            Device.decode = function decode(reader, length) {
                if (!(reader instanceof $Reader))
                    reader = $Reader.create(reader);
                let end =
                        length === undefined ? reader.len : reader.pos + length,
                    message = new $root.frontend.registration.Device();
                while (reader.pos < end) {
                    let tag = reader.uint32();
                    switch (tag >>> 3) {
                        case 1: {
                            message.deviceName = reader.string();
                            break;
                        }
                        case 2: {
                            message.deviceUuid = reader.string();
                            break;
                        }
                        case 3: {
                            if (
                                !(
                                    message.capabilities &&
                                    message.capabilities.length
                                )
                            )
                                message.capabilities = [];
                            message.capabilities.push(
                                $root.frontend.types.DeviceCapabilityStatus.decode(
                                    reader,
                                    reader.uint32(),
                                ),
                            );
                            break;
                        }
                        default:
                            reader.skipType(tag & 7);
                            break;
                    }
                }
                return message;
            };

            /**
             * Decodes a Device message from the specified reader or buffer, length delimited.
             * @function decodeDelimited
             * @memberof frontend.registration.Device
             * @static
             * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
             * @returns {frontend.registration.Device} Device
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            Device.decodeDelimited = function decodeDelimited(reader) {
                if (!(reader instanceof $Reader)) reader = new $Reader(reader);
                return this.decode(reader, reader.uint32());
            };

            /**
             * Verifies a Device message.
             * @function verify
             * @memberof frontend.registration.Device
             * @static
             * @param {Object.<string,*>} message Plain object to verify
             * @returns {string|null} `null` if valid, otherwise the reason why it is not
             */
            Device.verify = function verify(message) {
                if (typeof message !== "object" || message === null)
                    return "object expected";
                if (
                    message.deviceName != null &&
                    message.hasOwnProperty("deviceName")
                )
                    if (!$util.isString(message.deviceName))
                        return "deviceName: string expected";
                if (
                    message.deviceUuid != null &&
                    message.hasOwnProperty("deviceUuid")
                )
                    if (!$util.isString(message.deviceUuid))
                        return "deviceUuid: string expected";
                if (
                    message.capabilities != null &&
                    message.hasOwnProperty("capabilities")
                ) {
                    if (!Array.isArray(message.capabilities))
                        return "capabilities: array expected";
                    for (let i = 0; i < message.capabilities.length; ++i) {
                        let error =
                            $root.frontend.types.DeviceCapabilityStatus.verify(
                                message.capabilities[i],
                            );
                        if (error) return "capabilities." + error;
                    }
                }
                return null;
            };

            /**
             * Creates a Device message from a plain object. Also converts values to their respective internal types.
             * @function fromObject
             * @memberof frontend.registration.Device
             * @static
             * @param {Object.<string,*>} object Plain object
             * @returns {frontend.registration.Device} Device
             */
            Device.fromObject = function fromObject(object) {
                if (object instanceof $root.frontend.registration.Device)
                    return object;
                let message = new $root.frontend.registration.Device();
                if (object.deviceName != null)
                    message.deviceName = String(object.deviceName);
                if (object.deviceUuid != null)
                    message.deviceUuid = String(object.deviceUuid);
                if (object.capabilities) {
                    if (!Array.isArray(object.capabilities))
                        throw TypeError(
                            ".frontend.registration.Device.capabilities: array expected",
                        );
                    message.capabilities = [];
                    for (let i = 0; i < object.capabilities.length; ++i) {
                        if (typeof object.capabilities[i] !== "object")
                            throw TypeError(
                                ".frontend.registration.Device.capabilities: object expected",
                            );
                        message.capabilities[i] =
                            $root.frontend.types.DeviceCapabilityStatus.fromObject(
                                object.capabilities[i],
                            );
                    }
                }
                return message;
            };

            /**
             * Creates a plain object from a Device message. Also converts values to other types if specified.
             * @function toObject
             * @memberof frontend.registration.Device
             * @static
             * @param {frontend.registration.Device} message Device
             * @param {$protobuf.IConversionOptions} [options] Conversion options
             * @returns {Object.<string,*>} Plain object
             */
            Device.toObject = function toObject(message, options) {
                if (!options) options = {};
                let object = {};
                if (options.arrays || options.defaults)
                    object.capabilities = [];
                if (options.defaults) {
                    object.deviceName = "";
                    object.deviceUuid = "";
                }
                if (
                    message.deviceName != null &&
                    message.hasOwnProperty("deviceName")
                )
                    object.deviceName = message.deviceName;
                if (
                    message.deviceUuid != null &&
                    message.hasOwnProperty("deviceUuid")
                )
                    object.deviceUuid = message.deviceUuid;
                if (message.capabilities && message.capabilities.length) {
                    object.capabilities = [];
                    for (let j = 0; j < message.capabilities.length; ++j)
                        object.capabilities[j] =
                            $root.frontend.types.DeviceCapabilityStatus.toObject(
                                message.capabilities[j],
                                options,
                            );
                }
                return object;
            };

            /**
             * Converts this Device to JSON.
             * @function toJSON
             * @memberof frontend.registration.Device
             * @instance
             * @returns {Object.<string,*>} JSON object
             */
            Device.prototype.toJSON = function toJSON() {
                return this.constructor.toObject(
                    this,
                    $protobuf.util.toJSONOptions,
                );
            };

            /**
             * Gets the default type url for Device
             * @function getTypeUrl
             * @memberof frontend.registration.Device
             * @static
             * @param {string} [typeUrlPrefix] your custom typeUrlPrefix(default "type.googleapis.com")
             * @returns {string} The default type url
             */
            Device.getTypeUrl = function getTypeUrl(typeUrlPrefix) {
                if (typeUrlPrefix === undefined) {
                    typeUrlPrefix = "type.googleapis.com";
                }
                return typeUrlPrefix + "/frontend.registration.Device";
            };

            return Device;
        })();

        return registration;
    })();

    return frontend;
})());

export { $root as default };
