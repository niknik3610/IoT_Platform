import { API_DEVICE_CONTROL_ADDRESS } from "@/api_call_links";
import { frontend } from "@/generated/generated";
import { errAsync, okAsync, type Result } from "neverthrow";

export async function controlDevice(
    to_control_uuid: string,
    capability_to_control: string,
): Promise<Result<frontend.devicecontrol.DeviceControlResponse, Error>> {
    const jsonRequest = frontend.devicecontrol.DeviceControlRequest.create({
        device_uuid: to_control_uuid,
        capability: capability_to_control,
    }).toJSON();

    const response = await fetch(API_DEVICE_CONTROL_ADDRESS, {
        method: "POST",
        body: JSON.stringify(jsonRequest),
        headers: {
            "Content-type": "application/json; charset=UTF-8",
        },
    });

    try {
        const parsed_response: frontend.devicecontrol.DeviceControlResponse =
            await JSON.parse(await response.text());
        return okAsync(parsed_response);
    } catch (e) {
        return errAsync(new Error("Received a Malformed Api Response"));
    }
}
