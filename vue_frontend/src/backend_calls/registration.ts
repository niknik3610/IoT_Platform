export async function registerSelf(serverAddress: String, deviceName: String) {
    //todo: move all api endpoints to one file of constants
    //todo: compile protos to have easy access to right structs
    return fetch("/api/frontend/registration", {
        method: "POST",
        body: JSON.stringify({
            "device_name":  deviceName,
        }),
        headers: {
            "Content-type": "application/json; charset=UTF-8"
        }
    });
}
