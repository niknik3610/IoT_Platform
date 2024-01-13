// package: frontend.registration
// file: registrationService.proto

/* tslint:disable */
/* eslint-disable */

import * as jspb from "google-protobuf";
import * as frontend_types_pb from "./frontend_types_pb";

export class RegistrationRequest extends jspb.Message { 
    getDeviceName(): string;
    setDeviceName(value: string): RegistrationRequest;

    serializeBinary(): Uint8Array;
    toObject(includeInstance?: boolean): RegistrationRequest.AsObject;
    static toObject(includeInstance: boolean, msg: RegistrationRequest): RegistrationRequest.AsObject;
    static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
    static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
    static serializeBinaryToWriter(message: RegistrationRequest, writer: jspb.BinaryWriter): void;
    static deserializeBinary(bytes: Uint8Array): RegistrationRequest;
    static deserializeBinaryFromReader(message: RegistrationRequest, reader: jspb.BinaryReader): RegistrationRequest;
}

export namespace RegistrationRequest {
    export type AsObject = {
        deviceName: string,
    }
}

export class RegistrationResponse extends jspb.Message { 
    getClientId(): string;
    setClientId(value: string): RegistrationResponse;

    serializeBinary(): Uint8Array;
    toObject(includeInstance?: boolean): RegistrationResponse.AsObject;
    static toObject(includeInstance: boolean, msg: RegistrationResponse): RegistrationResponse.AsObject;
    static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
    static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
    static serializeBinaryToWriter(message: RegistrationResponse, writer: jspb.BinaryWriter): void;
    static deserializeBinary(bytes: Uint8Array): RegistrationResponse;
    static deserializeBinaryFromReader(message: RegistrationResponse, reader: jspb.BinaryReader): RegistrationResponse;
}

export namespace RegistrationResponse {
    export type AsObject = {
        clientId: string,
    }
}

export class ConnectedDevicesRequest extends jspb.Message { 
    getClientId(): string;
    setClientId(value: string): ConnectedDevicesRequest;

    serializeBinary(): Uint8Array;
    toObject(includeInstance?: boolean): ConnectedDevicesRequest.AsObject;
    static toObject(includeInstance: boolean, msg: ConnectedDevicesRequest): ConnectedDevicesRequest.AsObject;
    static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
    static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
    static serializeBinaryToWriter(message: ConnectedDevicesRequest, writer: jspb.BinaryWriter): void;
    static deserializeBinary(bytes: Uint8Array): ConnectedDevicesRequest;
    static deserializeBinaryFromReader(message: ConnectedDevicesRequest, reader: jspb.BinaryReader): ConnectedDevicesRequest;
}

export namespace ConnectedDevicesRequest {
    export type AsObject = {
        clientId: string,
    }
}

export class ConnectedDevicesResponse extends jspb.Message { 
    clearDevicesList(): void;
    getDevicesList(): Array<Device>;
    setDevicesList(value: Array<Device>): ConnectedDevicesResponse;
    addDevices(value?: Device, index?: number): Device;

    serializeBinary(): Uint8Array;
    toObject(includeInstance?: boolean): ConnectedDevicesResponse.AsObject;
    static toObject(includeInstance: boolean, msg: ConnectedDevicesResponse): ConnectedDevicesResponse.AsObject;
    static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
    static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
    static serializeBinaryToWriter(message: ConnectedDevicesResponse, writer: jspb.BinaryWriter): void;
    static deserializeBinary(bytes: Uint8Array): ConnectedDevicesResponse;
    static deserializeBinaryFromReader(message: ConnectedDevicesResponse, reader: jspb.BinaryReader): ConnectedDevicesResponse;
}

export namespace ConnectedDevicesResponse {
    export type AsObject = {
        devicesList: Array<Device.AsObject>,
    }
}

export class Device extends jspb.Message { 
    getDeviceName(): string;
    setDeviceName(value: string): Device;
    getDeviceUuid(): string;
    setDeviceUuid(value: string): Device;
    clearCapabilitiesList(): void;
    getCapabilitiesList(): Array<frontend_types_pb.DeviceCapabilityStatus>;
    setCapabilitiesList(value: Array<frontend_types_pb.DeviceCapabilityStatus>): Device;
    addCapabilities(value?: frontend_types_pb.DeviceCapabilityStatus, index?: number): frontend_types_pb.DeviceCapabilityStatus;

    serializeBinary(): Uint8Array;
    toObject(includeInstance?: boolean): Device.AsObject;
    static toObject(includeInstance: boolean, msg: Device): Device.AsObject;
    static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
    static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
    static serializeBinaryToWriter(message: Device, writer: jspb.BinaryWriter): void;
    static deserializeBinary(bytes: Uint8Array): Device;
    static deserializeBinaryFromReader(message: Device, reader: jspb.BinaryReader): Device;
}

export namespace Device {
    export type AsObject = {
        deviceName: string,
        deviceUuid: string,
        capabilitiesList: Array<frontend_types_pb.DeviceCapabilityStatus.AsObject>,
    }
}
