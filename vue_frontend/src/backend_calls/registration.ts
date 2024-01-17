import { API_REGISTRATION_ADDRESS } from "@/api_call_links";
import { frontend } from "@/generated/generated"

export async function registerSelf(deviceName: String): Promise<frontend.registration.RegistrationResponse | undefined> {
    let response = await fetch(API_REGISTRATION_ADDRESS, {
        method: "POST",
        body: JSON.stringify({
            "device_name":  deviceName,
        }),
        headers: {
            "Content-type": "application/json; charset=UTF-8"
        }
    });

    try {
        let parsed_response: frontend.registration.RegistrationResponse = await JSON.parse(await response.text());
        return parsed_response;
    } catch (e) {
        console.error(e);
        return undefined;
    }
}
