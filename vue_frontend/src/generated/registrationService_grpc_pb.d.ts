// package: frontend.registration
// file: registrationService.proto

/* tslint:disable */
/* eslint-disable */

import * as grpc from "grpc";
import * as registrationService_pb from "./registrationService_pb";
import * as frontend_types_pb from "./frontend_types_pb";

interface IFrontendRegistrationServiceService extends grpc.ServiceDefinition<grpc.UntypedServiceImplementation> {
    register: IFrontendRegistrationServiceService_IRegister;
    getConnectedDevices: IFrontendRegistrationServiceService_IGetConnectedDevices;
}

interface IFrontendRegistrationServiceService_IRegister extends grpc.MethodDefinition<registrationService_pb.RegistrationRequest, registrationService_pb.RegistrationResponse> {
    path: "/frontend.registration.FrontendRegistrationService/Register";
    requestStream: false;
    responseStream: false;
    requestSerialize: grpc.serialize<registrationService_pb.RegistrationRequest>;
    requestDeserialize: grpc.deserialize<registrationService_pb.RegistrationRequest>;
    responseSerialize: grpc.serialize<registrationService_pb.RegistrationResponse>;
    responseDeserialize: grpc.deserialize<registrationService_pb.RegistrationResponse>;
}
interface IFrontendRegistrationServiceService_IGetConnectedDevices extends grpc.MethodDefinition<registrationService_pb.ConnectedDevicesRequest, registrationService_pb.ConnectedDevicesResponse> {
    path: "/frontend.registration.FrontendRegistrationService/GetConnectedDevices";
    requestStream: false;
    responseStream: false;
    requestSerialize: grpc.serialize<registrationService_pb.ConnectedDevicesRequest>;
    requestDeserialize: grpc.deserialize<registrationService_pb.ConnectedDevicesRequest>;
    responseSerialize: grpc.serialize<registrationService_pb.ConnectedDevicesResponse>;
    responseDeserialize: grpc.deserialize<registrationService_pb.ConnectedDevicesResponse>;
}

export const FrontendRegistrationServiceService: IFrontendRegistrationServiceService;

export interface IFrontendRegistrationServiceServer {
    register: grpc.handleUnaryCall<registrationService_pb.RegistrationRequest, registrationService_pb.RegistrationResponse>;
    getConnectedDevices: grpc.handleUnaryCall<registrationService_pb.ConnectedDevicesRequest, registrationService_pb.ConnectedDevicesResponse>;
}

export interface IFrontendRegistrationServiceClient {
    register(request: registrationService_pb.RegistrationRequest, callback: (error: grpc.ServiceError | null, response: registrationService_pb.RegistrationResponse) => void): grpc.ClientUnaryCall;
    register(request: registrationService_pb.RegistrationRequest, metadata: grpc.Metadata, callback: (error: grpc.ServiceError | null, response: registrationService_pb.RegistrationResponse) => void): grpc.ClientUnaryCall;
    register(request: registrationService_pb.RegistrationRequest, metadata: grpc.Metadata, options: Partial<grpc.CallOptions>, callback: (error: grpc.ServiceError | null, response: registrationService_pb.RegistrationResponse) => void): grpc.ClientUnaryCall;
    getConnectedDevices(request: registrationService_pb.ConnectedDevicesRequest, callback: (error: grpc.ServiceError | null, response: registrationService_pb.ConnectedDevicesResponse) => void): grpc.ClientUnaryCall;
    getConnectedDevices(request: registrationService_pb.ConnectedDevicesRequest, metadata: grpc.Metadata, callback: (error: grpc.ServiceError | null, response: registrationService_pb.ConnectedDevicesResponse) => void): grpc.ClientUnaryCall;
    getConnectedDevices(request: registrationService_pb.ConnectedDevicesRequest, metadata: grpc.Metadata, options: Partial<grpc.CallOptions>, callback: (error: grpc.ServiceError | null, response: registrationService_pb.ConnectedDevicesResponse) => void): grpc.ClientUnaryCall;
}

export class FrontendRegistrationServiceClient extends grpc.Client implements IFrontendRegistrationServiceClient {
    constructor(address: string, credentials: grpc.ChannelCredentials, options?: object);
    public register(request: registrationService_pb.RegistrationRequest, callback: (error: grpc.ServiceError | null, response: registrationService_pb.RegistrationResponse) => void): grpc.ClientUnaryCall;
    public register(request: registrationService_pb.RegistrationRequest, metadata: grpc.Metadata, callback: (error: grpc.ServiceError | null, response: registrationService_pb.RegistrationResponse) => void): grpc.ClientUnaryCall;
    public register(request: registrationService_pb.RegistrationRequest, metadata: grpc.Metadata, options: Partial<grpc.CallOptions>, callback: (error: grpc.ServiceError | null, response: registrationService_pb.RegistrationResponse) => void): grpc.ClientUnaryCall;
    public getConnectedDevices(request: registrationService_pb.ConnectedDevicesRequest, callback: (error: grpc.ServiceError | null, response: registrationService_pb.ConnectedDevicesResponse) => void): grpc.ClientUnaryCall;
    public getConnectedDevices(request: registrationService_pb.ConnectedDevicesRequest, metadata: grpc.Metadata, callback: (error: grpc.ServiceError | null, response: registrationService_pb.ConnectedDevicesResponse) => void): grpc.ClientUnaryCall;
    public getConnectedDevices(request: registrationService_pb.ConnectedDevicesRequest, metadata: grpc.Metadata, options: Partial<grpc.CallOptions>, callback: (error: grpc.ServiceError | null, response: registrationService_pb.ConnectedDevicesResponse) => void): grpc.ClientUnaryCall;
}
