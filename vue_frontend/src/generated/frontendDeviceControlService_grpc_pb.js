// GENERATED CODE -- DO NOT EDIT!

'use strict';
var grpc = require('grpc');
var frontendDeviceControlService_pb = require('./frontendDeviceControlService_pb.js');
var frontend_types_pb = require('./frontend_types_pb.js');

function serialize_frontend_devicecontrol_DeviceControlRequest(arg) {
  if (!(arg instanceof frontendDeviceControlService_pb.DeviceControlRequest)) {
    throw new Error('Expected argument of type frontend.devicecontrol.DeviceControlRequest');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_frontend_devicecontrol_DeviceControlRequest(buffer_arg) {
  return frontendDeviceControlService_pb.DeviceControlRequest.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_frontend_devicecontrol_DeviceControlResponse(arg) {
  if (!(arg instanceof frontendDeviceControlService_pb.DeviceControlResponse)) {
    throw new Error('Expected argument of type frontend.devicecontrol.DeviceControlResponse');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_frontend_devicecontrol_DeviceControlResponse(buffer_arg) {
  return frontendDeviceControlService_pb.DeviceControlResponse.deserializeBinary(new Uint8Array(buffer_arg));
}


var FrontendDeviceControlServiceService = exports.FrontendDeviceControlServiceService = {
  controlDevice: {
    path: '/frontend.devicecontrol.FrontendDeviceControlService/ControlDevice',
    requestStream: false,
    responseStream: false,
    requestType: frontendDeviceControlService_pb.DeviceControlRequest,
    responseType: frontendDeviceControlService_pb.DeviceControlResponse,
    requestSerialize: serialize_frontend_devicecontrol_DeviceControlRequest,
    requestDeserialize: deserialize_frontend_devicecontrol_DeviceControlRequest,
    responseSerialize: serialize_frontend_devicecontrol_DeviceControlResponse,
    responseDeserialize: deserialize_frontend_devicecontrol_DeviceControlResponse,
  },
};

exports.FrontendDeviceControlServiceClient = grpc.makeGenericClientConstructor(FrontendDeviceControlServiceService);
