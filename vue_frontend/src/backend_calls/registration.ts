import { API_REGISTRATION_ADDRESS } from "@/api_call_links";
import { frontend } from "@/generated/generated";
import { Result, errAsync, okAsync } from "neverthrow";

export async function registerSelf(
    deviceName: String,
): Promise<Result<frontend.registration.RegistrationResponse, Error>> {
    const response = await fetch(API_REGISTRATION_ADDRESS, {
        method: "POST",
        body: JSON.stringify({
            device_name: deviceName,
        }),
        headers: {
            "Content-type": "application/json; charset=UTF-8",
        },
    });

    try {
        const parsed_response: frontend.registration.RegistrationResponse =
            await JSON.parse(await response.text());
        return okAsync(parsed_response);
    } catch (e) {
        return errAsync(new Error("Received a Malformed Api Response"));
    }
}
