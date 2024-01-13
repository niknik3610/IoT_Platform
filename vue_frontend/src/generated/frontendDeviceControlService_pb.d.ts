// package: frontend.devicecontrol
// file: frontendDeviceControlService.proto

/* tslint:disable */
/* eslint-disable */

import * as jspb from "google-protobuf";
import * as frontend_types_pb from "./frontend_types_pb";

export class DeviceControlRequest extends jspb.Message { 
    getDeviceUuid(): string;
    setDeviceUuid(value: string): DeviceControlRequest;
    getCapability(): string;
    setCapability(value: string): DeviceControlRequest;

    serializeBinary(): Uint8Array;
    toObject(includeInstance?: boolean): DeviceControlRequest.AsObject;
    static toObject(includeInstance: boolean, msg: DeviceControlRequest): DeviceControlRequest.AsObject;
    static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
    static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
    static serializeBinaryToWriter(message: DeviceControlRequest, writer: jspb.BinaryWriter): void;
    static deserializeBinary(bytes: Uint8Array): DeviceControlRequest;
    static deserializeBinaryFromReader(message: DeviceControlRequest, reader: jspb.BinaryReader): DeviceControlRequest;
}

export namespace DeviceControlRequest {
    export type AsObject = {
        deviceUuid: string,
        capability: string,
    }
}

export class DeviceControlResponse extends jspb.Message { 
    getResult(): DeviceControlResult;
    setResult(value: DeviceControlResult): DeviceControlResponse;

    serializeBinary(): Uint8Array;
    toObject(includeInstance?: boolean): DeviceControlResponse.AsObject;
    static toObject(includeInstance: boolean, msg: DeviceControlResponse): DeviceControlResponse.AsObject;
    static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
    static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
    static serializeBinaryToWriter(message: DeviceControlResponse, writer: jspb.BinaryWriter): void;
    static deserializeBinary(bytes: Uint8Array): DeviceControlResponse;
    static deserializeBinaryFromReader(message: DeviceControlResponse, reader: jspb.BinaryReader): DeviceControlResponse;
}

export namespace DeviceControlResponse {
    export type AsObject = {
        result: DeviceControlResult,
    }
}

export enum DeviceControlResult {
    UNKNOWN = 0,
    SUCCESS = 1,
}
