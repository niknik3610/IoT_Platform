// GENERATED CODE -- DO NOT EDIT!

'use strict';
var grpc = require('grpc');
var registrationService_pb = require('./registrationService_pb.js');
var frontend_types_pb = require('./frontend_types_pb.js');

function serialize_frontend_registration_ConnectedDevicesRequest(arg) {
  if (!(arg instanceof registrationService_pb.ConnectedDevicesRequest)) {
    throw new Error('Expected argument of type frontend.registration.ConnectedDevicesRequest');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_frontend_registration_ConnectedDevicesRequest(buffer_arg) {
  return registrationService_pb.ConnectedDevicesRequest.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_frontend_registration_ConnectedDevicesResponse(arg) {
  if (!(arg instanceof registrationService_pb.ConnectedDevicesResponse)) {
    throw new Error('Expected argument of type frontend.registration.ConnectedDevicesResponse');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_frontend_registration_ConnectedDevicesResponse(buffer_arg) {
  return registrationService_pb.ConnectedDevicesResponse.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_frontend_registration_RegistrationRequest(arg) {
  if (!(arg instanceof registrationService_pb.RegistrationRequest)) {
    throw new Error('Expected argument of type frontend.registration.RegistrationRequest');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_frontend_registration_RegistrationRequest(buffer_arg) {
  return registrationService_pb.RegistrationRequest.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_frontend_registration_RegistrationResponse(arg) {
  if (!(arg instanceof registrationService_pb.RegistrationResponse)) {
    throw new Error('Expected argument of type frontend.registration.RegistrationResponse');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_frontend_registration_RegistrationResponse(buffer_arg) {
  return registrationService_pb.RegistrationResponse.deserializeBinary(new Uint8Array(buffer_arg));
}


var FrontendRegistrationServiceService = exports.FrontendRegistrationServiceService = {
  register: {
    path: '/frontend.registration.FrontendRegistrationService/Register',
    requestStream: false,
    responseStream: false,
    requestType: registrationService_pb.RegistrationRequest,
    responseType: registrationService_pb.RegistrationResponse,
    requestSerialize: serialize_frontend_registration_RegistrationRequest,
    requestDeserialize: deserialize_frontend_registration_RegistrationRequest,
    responseSerialize: serialize_frontend_registration_RegistrationResponse,
    responseDeserialize: deserialize_frontend_registration_RegistrationResponse,
  },
  getConnectedDevices: {
    path: '/frontend.registration.FrontendRegistrationService/GetConnectedDevices',
    requestStream: false,
    responseStream: false,
    requestType: registrationService_pb.ConnectedDevicesRequest,
    responseType: registrationService_pb.ConnectedDevicesResponse,
    requestSerialize: serialize_frontend_registration_ConnectedDevicesRequest,
    requestDeserialize: deserialize_frontend_registration_ConnectedDevicesRequest,
    responseSerialize: serialize_frontend_registration_ConnectedDevicesResponse,
    responseDeserialize: deserialize_frontend_registration_ConnectedDevicesResponse,
  },
};

exports.FrontendRegistrationServiceClient = grpc.makeGenericClientConstructor(FrontendRegistrationServiceService);
