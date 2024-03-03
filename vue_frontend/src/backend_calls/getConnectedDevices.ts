import { API_GET_CONNECTED_DEVICES_ADDRESS } from "@/api_call_links";
import { frontend } from "@/generated/generated";
import { errAsync, okAsync, type Result } from "neverthrow";

export async function getConnectedDevices(
    deviceId: string,
): Promise<Result<frontend.registration.ConnectedDevicesResponse, Error>> {
    const jsonRequest = frontend.registration.ConnectedDevicesRequest.create({
        client_id: deviceId,
    }).toJSON();

    const response = await fetch(API_GET_CONNECTED_DEVICES_ADDRESS, {
        method: "POST",
        body: JSON.stringify(jsonRequest),
        headers: {
            "Content-type": "application/json; charset=UTF-8",
        },
    });

    try {
        const parsed_response: frontend.registration.ConnectedDevicesResponse =
            await JSON.parse(await response.text());
        return okAsync(parsed_response);
    } catch (e) {
        return errAsync(new Error("Received a Malformed Api Response"));
    }
}
