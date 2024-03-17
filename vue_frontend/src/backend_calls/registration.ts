import { API_REGISTRATION_ADDRESS } from "@/api_call_links";
import { Result, errAsync, okAsync } from "neverthrow";
import { frontend } from "@/generated/generated";

export async function registerSelf(
    deviceName: string,
): Promise<Result<frontend.registration.RegistrationResponse, Error>> {
    const registrationRequest = new frontend.registration.RegistrationRequest({
        device_name: deviceName,
    });

    const response = await fetch(API_REGISTRATION_ADDRESS, {
        method: "POST",
        body: JSON.stringify(registrationRequest),
        headers: {
            "Content-type": "application/json; charset=UTF-8",
        },
    });

    try {
        const parsed_response: frontend.registration.RegistrationResponse =
            await JSON.parse(await response.text());
        console.log(parsed_response)
        return okAsync(parsed_response);
    } catch (e) {
        return errAsync(new Error("Received a Malformed Api Response"));
    }
}
