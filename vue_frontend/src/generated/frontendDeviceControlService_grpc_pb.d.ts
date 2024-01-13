// package: frontend.devicecontrol
// file: frontendDeviceControlService.proto

/* tslint:disable */
/* eslint-disable */

import * as grpc from "grpc";
import * as frontendDeviceControlService_pb from "./frontendDeviceControlService_pb";
import * as frontend_types_pb from "./frontend_types_pb";

interface IFrontendDeviceControlServiceService extends grpc.ServiceDefinition<grpc.UntypedServiceImplementation> {
    controlDevice: IFrontendDeviceControlServiceService_IControlDevice;
}

interface IFrontendDeviceControlServiceService_IControlDevice extends grpc.MethodDefinition<frontendDeviceControlService_pb.DeviceControlRequest, frontendDeviceControlService_pb.DeviceControlResponse> {
    path: "/frontend.devicecontrol.FrontendDeviceControlService/ControlDevice";
    requestStream: false;
    responseStream: false;
    requestSerialize: grpc.serialize<frontendDeviceControlService_pb.DeviceControlRequest>;
    requestDeserialize: grpc.deserialize<frontendDeviceControlService_pb.DeviceControlRequest>;
    responseSerialize: grpc.serialize<frontendDeviceControlService_pb.DeviceControlResponse>;
    responseDeserialize: grpc.deserialize<frontendDeviceControlService_pb.DeviceControlResponse>;
}

export const FrontendDeviceControlServiceService: IFrontendDeviceControlServiceService;

export interface IFrontendDeviceControlServiceServer {
    controlDevice: grpc.handleUnaryCall<frontendDeviceControlService_pb.DeviceControlRequest, frontendDeviceControlService_pb.DeviceControlResponse>;
}

export interface IFrontendDeviceControlServiceClient {
    controlDevice(request: frontendDeviceControlService_pb.DeviceControlRequest, callback: (error: grpc.ServiceError | null, response: frontendDeviceControlService_pb.DeviceControlResponse) => void): grpc.ClientUnaryCall;
    controlDevice(request: frontendDeviceControlService_pb.DeviceControlRequest, metadata: grpc.Metadata, callback: (error: grpc.ServiceError | null, response: frontendDeviceControlService_pb.DeviceControlResponse) => void): grpc.ClientUnaryCall;
    controlDevice(request: frontendDeviceControlService_pb.DeviceControlRequest, metadata: grpc.Metadata, options: Partial<grpc.CallOptions>, callback: (error: grpc.ServiceError | null, response: frontendDeviceControlService_pb.DeviceControlResponse) => void): grpc.ClientUnaryCall;
}

export class FrontendDeviceControlServiceClient extends grpc.Client implements IFrontendDeviceControlServiceClient {
    constructor(address: string, credentials: grpc.ChannelCredentials, options?: object);
    public controlDevice(request: frontendDeviceControlService_pb.DeviceControlRequest, callback: (error: grpc.ServiceError | null, response: frontendDeviceControlService_pb.DeviceControlResponse) => void): grpc.ClientUnaryCall;
    public controlDevice(request: frontendDeviceControlService_pb.DeviceControlRequest, metadata: grpc.Metadata, callback: (error: grpc.ServiceError | null, response: frontendDeviceControlService_pb.DeviceControlResponse) => void): grpc.ClientUnaryCall;
    public controlDevice(request: frontendDeviceControlService_pb.DeviceControlRequest, metadata: grpc.Metadata, options: Partial<grpc.CallOptions>, callback: (error: grpc.ServiceError | null, response: frontendDeviceControlService_pb.DeviceControlResponse) => void): grpc.ClientUnaryCall;
}
