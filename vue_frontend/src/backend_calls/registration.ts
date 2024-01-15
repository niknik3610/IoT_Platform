import { RegistrationResponse } from "generated/registrationService_pb";
import { registrationService } from "@/grpc_clients";

export function registerWithServer(): Promise<RegistrationResponse> {
    return registrationService.register(new RegistrationRequest(), null);
}
