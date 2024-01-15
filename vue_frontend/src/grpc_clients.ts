import { FrontendRegistrationServiceClient } from "./generated/RegistrationServiceServiceClientPb";

const SERVER_ADDRESS = "http://[::1]:50051";

export const registrationService = new FrontendRegistrationServiceClient(
    SERVER_ADDRESS,
);
