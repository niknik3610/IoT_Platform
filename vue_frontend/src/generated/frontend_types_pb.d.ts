// package: frontend.types
// file: frontend_types.proto

/* tslint:disable */
/* eslint-disable */

import * as jspb from "google-protobuf";

export class DeviceCapabilityStatus extends jspb.Message { 
    getAvailable(): boolean;
    setAvailable(value: boolean): DeviceCapabilityStatus;
    getCapability(): string;
    setCapability(value: string): DeviceCapabilityStatus;

    serializeBinary(): Uint8Array;
    toObject(includeInstance?: boolean): DeviceCapabilityStatus.AsObject;
    static toObject(includeInstance: boolean, msg: DeviceCapabilityStatus): DeviceCapabilityStatus.AsObject;
    static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
    static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
    static serializeBinaryToWriter(message: DeviceCapabilityStatus, writer: jspb.BinaryWriter): void;
    static deserializeBinary(bytes: Uint8Array): DeviceCapabilityStatus;
    static deserializeBinaryFromReader(message: DeviceCapabilityStatus, reader: jspb.BinaryReader): DeviceCapabilityStatus;
}

export namespace DeviceCapabilityStatus {
    export type AsObject = {
        available: boolean,
        capability: string,
    }
}
