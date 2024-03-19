import * as $protobuf from "protobufjs";
import Long = require("long");
export namespace frontend {
    namespace devicecontrol {
        class FrontendDeviceControlService extends $protobuf.rpc.Service {
            constructor(
                rpcImpl: $protobuf.RPCImpl,
                requestDelimited?: boolean,
                responseDelimited?: boolean,
            );
            public static create(
                rpcImpl: $protobuf.RPCImpl,
                requestDelimited?: boolean,
                responseDelimited?: boolean,
            ): FrontendDeviceControlService;
            public controlDevice(
                request: frontend.devicecontrol.IDeviceControlRequest,
                callback: frontend.devicecontrol.FrontendDeviceControlService.ControlDeviceCallback,
            ): void;
            public controlDevice(
                request: frontend.devicecontrol.IDeviceControlRequest,
            ): Promise<frontend.devicecontrol.DeviceControlResponse>;
        }

        namespace FrontendDeviceControlService {
            type ControlDeviceCallback = (
                error: Error | null,
                response?: frontend.devicecontrol.DeviceControlResponse,
            ) => void;
        }

        interface IDeviceControlRequest {
            device_uuid?: string | null;
            capability?: string | null;
            timestamp?: number | Long | null;
            value?: number | null;
        }

        class DeviceControlRequest implements IDeviceControlRequest {
            constructor(
                properties?: frontend.devicecontrol.IDeviceControlRequest,
            );
            public device_uuid: string;
            public capability: string;
            public timestamp: number | Long;
            public value?: number | null;
            public _value?: "value";
            public static create(
                properties?: frontend.devicecontrol.IDeviceControlRequest,
            ): frontend.devicecontrol.DeviceControlRequest;
            public static encode(
                message: frontend.devicecontrol.IDeviceControlRequest,
                writer?: $protobuf.Writer,
            ): $protobuf.Writer;
            public static encodeDelimited(
                message: frontend.devicecontrol.IDeviceControlRequest,
                writer?: $protobuf.Writer,
            ): $protobuf.Writer;
            public static decode(
                reader: $protobuf.Reader | Uint8Array,
                length?: number,
            ): frontend.devicecontrol.DeviceControlRequest;
            public static decodeDelimited(
                reader: $protobuf.Reader | Uint8Array,
            ): frontend.devicecontrol.DeviceControlRequest;
            public static verify(message: { [k: string]: any }): string | null;
            public static fromObject(object: {
                [k: string]: any;
            }): frontend.devicecontrol.DeviceControlRequest;
            public static toObject(
                message: frontend.devicecontrol.DeviceControlRequest,
                options?: $protobuf.IConversionOptions,
            ): { [k: string]: any };
            public toJSON(): { [k: string]: any };
            public static getTypeUrl(typeUrlPrefix?: string): string;
        }

        interface IDeviceControlResponse {
            result?: frontend.devicecontrol.DeviceControlResult | null;
        }

        class DeviceControlResponse implements IDeviceControlResponse {
            constructor(
                properties?: frontend.devicecontrol.IDeviceControlResponse,
            );
            public result: frontend.devicecontrol.DeviceControlResult;
            public static create(
                properties?: frontend.devicecontrol.IDeviceControlResponse,
            ): frontend.devicecontrol.DeviceControlResponse;
            public static encode(
                message: frontend.devicecontrol.IDeviceControlResponse,
                writer?: $protobuf.Writer,
            ): $protobuf.Writer;
            public static encodeDelimited(
                message: frontend.devicecontrol.IDeviceControlResponse,
                writer?: $protobuf.Writer,
            ): $protobuf.Writer;
            public static decode(
                reader: $protobuf.Reader | Uint8Array,
                length?: number,
            ): frontend.devicecontrol.DeviceControlResponse;
            public static decodeDelimited(
                reader: $protobuf.Reader | Uint8Array,
            ): frontend.devicecontrol.DeviceControlResponse;
            public static verify(message: { [k: string]: any }): string | null;
            public static fromObject(object: {
                [k: string]: any;
            }): frontend.devicecontrol.DeviceControlResponse;
            public static toObject(
                message: frontend.devicecontrol.DeviceControlResponse,
                options?: $protobuf.IConversionOptions,
            ): { [k: string]: any };
            public toJSON(): { [k: string]: any };
            public static getTypeUrl(typeUrlPrefix?: string): string;
        }

        enum DeviceControlResult {
            UNKNOWN = 0,
            SUCCESS = 1,
        }
    }

    namespace types {
        interface IDeviceCapabilityStatus {
            available?: boolean | null;
            capability?: string | null;
            type?: frontend.types.DeviceCapabilityType | null;
            value?: number | null;
        }

        class DeviceCapabilityStatus implements IDeviceCapabilityStatus {
            constructor(properties?: frontend.types.IDeviceCapabilityStatus);
            public available: boolean;
            public capability: string;
            public type: frontend.types.DeviceCapabilityType;
            public value?: number | null;
            public _value?: "value";
            public static create(
                properties?: frontend.types.IDeviceCapabilityStatus,
            ): frontend.types.DeviceCapabilityStatus;
            public static encode(
                message: frontend.types.IDeviceCapabilityStatus,
                writer?: $protobuf.Writer,
            ): $protobuf.Writer;
            public static encodeDelimited(
                message: frontend.types.IDeviceCapabilityStatus,
                writer?: $protobuf.Writer,
            ): $protobuf.Writer;
            public static decode(
                reader: $protobuf.Reader | Uint8Array,
                length?: number,
            ): frontend.types.DeviceCapabilityStatus;
            public static decodeDelimited(
                reader: $protobuf.Reader | Uint8Array,
            ): frontend.types.DeviceCapabilityStatus;
            public static verify(message: { [k: string]: any }): string | null;
            public static fromObject(object: {
                [k: string]: any;
            }): frontend.types.DeviceCapabilityStatus;
            public static toObject(
                message: frontend.types.DeviceCapabilityStatus,
                options?: $protobuf.IConversionOptions,
            ): { [k: string]: any };
            public toJSON(): { [k: string]: any };
            public static getTypeUrl(typeUrlPrefix?: string): string;
        }

        enum DeviceCapabilityType {
            BUTTON = 0,
            SLIDER = 1,
        }
    }

    namespace registration {
        class FrontendRegistrationService extends $protobuf.rpc.Service {
            constructor(
                rpcImpl: $protobuf.RPCImpl,
                requestDelimited?: boolean,
                responseDelimited?: boolean,
            );
            public static create(
                rpcImpl: $protobuf.RPCImpl,
                requestDelimited?: boolean,
                responseDelimited?: boolean,
            ): FrontendRegistrationService;
            public register(
                request: frontend.registration.IRegistrationRequest,
                callback: frontend.registration.FrontendRegistrationService.RegisterCallback,
            ): void;
            public register(
                request: frontend.registration.IRegistrationRequest,
            ): Promise<frontend.registration.RegistrationResponse>;
            public getConnectedDevices(
                request: frontend.registration.IConnectedDevicesRequest,
                callback: frontend.registration.FrontendRegistrationService.GetConnectedDevicesCallback,
            ): void;
            public getConnectedDevices(
                request: frontend.registration.IConnectedDevicesRequest,
            ): Promise<frontend.registration.ConnectedDevicesResponse>;
        }

        namespace FrontendRegistrationService {
            type RegisterCallback = (
                error: Error | null,
                response?: frontend.registration.RegistrationResponse,
            ) => void;

            type GetConnectedDevicesCallback = (
                error: Error | null,
                response?: frontend.registration.ConnectedDevicesResponse,
            ) => void;
        }

        interface IRegistrationRequest {
            device_name?: string | null;
        }

        class RegistrationRequest implements IRegistrationRequest {
            constructor(
                properties?: frontend.registration.IRegistrationRequest,
            );
            public device_name: string;
            public static create(
                properties?: frontend.registration.IRegistrationRequest,
            ): frontend.registration.RegistrationRequest;
            public static encode(
                message: frontend.registration.IRegistrationRequest,
                writer?: $protobuf.Writer,
            ): $protobuf.Writer;
            public static encodeDelimited(
                message: frontend.registration.IRegistrationRequest,
                writer?: $protobuf.Writer,
            ): $protobuf.Writer;
            public static decode(
                reader: $protobuf.Reader | Uint8Array,
                length?: number,
            ): frontend.registration.RegistrationRequest;
            public static decodeDelimited(
                reader: $protobuf.Reader | Uint8Array,
            ): frontend.registration.RegistrationRequest;
            public static verify(message: { [k: string]: any }): string | null;
            public static fromObject(object: {
                [k: string]: any;
            }): frontend.registration.RegistrationRequest;
            public static toObject(
                message: frontend.registration.RegistrationRequest,
                options?: $protobuf.IConversionOptions,
            ): { [k: string]: any };
            public toJSON(): { [k: string]: any };
            public static getTypeUrl(typeUrlPrefix?: string): string;
        }

        interface IRegistrationResponse {
            client_id?: string | null;
        }

        class RegistrationResponse implements IRegistrationResponse {
            constructor(
                properties?: frontend.registration.IRegistrationResponse,
            );
            public client_id: string;
            public static create(
                properties?: frontend.registration.IRegistrationResponse,
            ): frontend.registration.RegistrationResponse;
            public static encode(
                message: frontend.registration.IRegistrationResponse,
                writer?: $protobuf.Writer,
            ): $protobuf.Writer;
            public static encodeDelimited(
                message: frontend.registration.IRegistrationResponse,
                writer?: $protobuf.Writer,
            ): $protobuf.Writer;
            public static decode(
                reader: $protobuf.Reader | Uint8Array,
                length?: number,
            ): frontend.registration.RegistrationResponse;
            public static decodeDelimited(
                reader: $protobuf.Reader | Uint8Array,
            ): frontend.registration.RegistrationResponse;
            public static verify(message: { [k: string]: any }): string | null;
            public static fromObject(object: {
                [k: string]: any;
            }): frontend.registration.RegistrationResponse;
            public static toObject(
                message: frontend.registration.RegistrationResponse,
                options?: $protobuf.IConversionOptions,
            ): { [k: string]: any };
            public toJSON(): { [k: string]: any };
            public static getTypeUrl(typeUrlPrefix?: string): string;
        }

        interface IConnectedDevicesRequest {
            client_id?: string | null;
        }

        class ConnectedDevicesRequest implements IConnectedDevicesRequest {
            constructor(
                properties?: frontend.registration.IConnectedDevicesRequest,
            );
            public client_id: string;
            public static create(
                properties?: frontend.registration.IConnectedDevicesRequest,
            ): frontend.registration.ConnectedDevicesRequest;
            public static encode(
                message: frontend.registration.IConnectedDevicesRequest,
                writer?: $protobuf.Writer,
            ): $protobuf.Writer;
            public static encodeDelimited(
                message: frontend.registration.IConnectedDevicesRequest,
                writer?: $protobuf.Writer,
            ): $protobuf.Writer;
            public static decode(
                reader: $protobuf.Reader | Uint8Array,
                length?: number,
            ): frontend.registration.ConnectedDevicesRequest;
            public static decodeDelimited(
                reader: $protobuf.Reader | Uint8Array,
            ): frontend.registration.ConnectedDevicesRequest;
            public static verify(message: { [k: string]: any }): string | null;
            public static fromObject(object: {
                [k: string]: any;
            }): frontend.registration.ConnectedDevicesRequest;
            public static toObject(
                message: frontend.registration.ConnectedDevicesRequest,
                options?: $protobuf.IConversionOptions,
            ): { [k: string]: any };
            public toJSON(): { [k: string]: any };
            public static getTypeUrl(typeUrlPrefix?: string): string;
        }

        interface IConnectedDevicesResponse {
            devices?: frontend.registration.IDevice[] | null;
        }

        class ConnectedDevicesResponse implements IConnectedDevicesResponse {
            constructor(
                properties?: frontend.registration.IConnectedDevicesResponse,
            );
            public devices: frontend.registration.IDevice[];
            public static create(
                properties?: frontend.registration.IConnectedDevicesResponse,
            ): frontend.registration.ConnectedDevicesResponse;
            public static encode(
                message: frontend.registration.IConnectedDevicesResponse,
                writer?: $protobuf.Writer,
            ): $protobuf.Writer;
            public static encodeDelimited(
                message: frontend.registration.IConnectedDevicesResponse,
                writer?: $protobuf.Writer,
            ): $protobuf.Writer;
            public static decode(
                reader: $protobuf.Reader | Uint8Array,
                length?: number,
            ): frontend.registration.ConnectedDevicesResponse;
            public static decodeDelimited(
                reader: $protobuf.Reader | Uint8Array,
            ): frontend.registration.ConnectedDevicesResponse;
            public static verify(message: { [k: string]: any }): string | null;
            public static fromObject(object: {
                [k: string]: any;
            }): frontend.registration.ConnectedDevicesResponse;
            public static toObject(
                message: frontend.registration.ConnectedDevicesResponse,
                options?: $protobuf.IConversionOptions,
            ): { [k: string]: any };
            public toJSON(): { [k: string]: any };
            public static getTypeUrl(typeUrlPrefix?: string): string;
        }

        interface IDevice {
            device_name?: string | null;
            device_uuid?: string | null;
            capabilities?: frontend.types.IDeviceCapabilityStatus[] | null;
        }

        class Device implements IDevice {
            constructor(properties?: frontend.registration.IDevice);
            public device_name: string;
            public device_uuid: string;
            public capabilities: frontend.types.IDeviceCapabilityStatus[];
            public static create(
                properties?: frontend.registration.IDevice,
            ): frontend.registration.Device;
            public static encode(
                message: frontend.registration.IDevice,
                writer?: $protobuf.Writer,
            ): $protobuf.Writer;
            public static encodeDelimited(
                message: frontend.registration.IDevice,
                writer?: $protobuf.Writer,
            ): $protobuf.Writer;
            public static decode(
                reader: $protobuf.Reader | Uint8Array,
                length?: number,
            ): frontend.registration.Device;
            public static decodeDelimited(
                reader: $protobuf.Reader | Uint8Array,
            ): frontend.registration.Device;
            public static verify(message: { [k: string]: any }): string | null;
            public static fromObject(object: {
                [k: string]: any;
            }): frontend.registration.Device;
            public static toObject(
                message: frontend.registration.Device,
                options?: $protobuf.IConversionOptions,
            ): { [k: string]: any };
            public toJSON(): { [k: string]: any };
            public static getTypeUrl(typeUrlPrefix?: string): string;
        }
    }
}
