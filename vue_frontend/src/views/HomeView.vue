<script setup lang="ts">
import { onMounted, ref } from "vue";
import { useDeviceIdStore } from "@/stores/deviceIdStore";
import { getConnectedDevices } from "@/backend_calls/getConnectedDevices";
import { errAsync, okAsync, type Result } from "neverthrow";
import { frontend } from "@/generated/generated";

const deviceIdStore = useDeviceIdStore();
const connectedDevices = ref<frontend.registration.Device[]>();

const REFRESH_TIMING_MS = 200;

onMounted(async () => {
    if (!deviceIdStore.device_id_valid) {
        await deviceIdStore.registerSelf();
    }

    if (!deviceIdStore.device_id_valid) {
        console.error("device_id invalid");
        return;
    }

    let result = await proccessConnectedDevices();
    if (result.isOk()) {
        connectedDevices.value = result.value;
    } else if (result.isErr()) {
        console.error(result.error.message);
        //todo: add proper error message here
        connectedDevices.value = [];
    }

    //device list refreshes every x seconds
    setInterval(async () => {
        let result = await proccessConnectedDevices();
        if (result.isOk()) {
            connectedDevices.value = result.value;
            console.log("refreshed devices");
        } else if (result.isErr()) {
            console.error(result.error.message);
            //todo: add proper error message here
            connectedDevices.value = [];
        }
    }, REFRESH_TIMING_MS);
});

async function proccessConnectedDevices(): Promise<
    Result<frontend.registration.Device[], Error>
> {
    let response = await getConnectedDevices(deviceIdStore.device_id);

    if (response.isOk()) {
        if (response.value.devices.length === 0) {
            return okAsync([]);
        }
        return okAsync(
            response.value.devices.map((device) => {
                return new frontend.registration.Device(device);
            }),
        );
    }
    return errAsync(new Error(response.error.message));
}
</script>

<template>
    <div>
        <p v-show="deviceIdStore.device_id_valid">
            Your Device Id: {{ deviceIdStore.device_id }}
        </p>
        <p v-show="!deviceIdStore.device_id_valid">
            Device Id was unable to be fetched
        </p>
    </div>
    <div class="connected-devices">
        <h1>Your Connected Devices:</h1>
        <!-- <div v-for="device in connectedDevices"> -->
        <!--     <p>device name: {{device.name}}</p> -->
        <!--     <p>device capabilities:</p> -->
        <!--     <p v-for="capability in device.capabilities">{{capability}}</p> -->
        <!-- </div> -->
        <div class="connected-devices-container">
            <GenericIotDevice
                v-for="device in connectedDevices"
                v-bind:key="device.device_name"
                :deviceName="device.device_name"
                :capabilities="device.capabilities"
                :deviceUuid="device.device_uuid"
            />
        </div>
    </div>
</template>

<style>
h1 {
    padding-bottom: 40px;
}
.connected-devices-container {
    display: flex;
    flex-wrap: wrap;
    flex-direction: row;

    justify-items: center;
    justify-content: center;
    padding: 10px;
    align-items: center;
    width: 100%;
    gap: 100px max(30px, 20%);
}
.connected-devices {
    display: flex;
    flex-direction: column;
    align-items: center;
}
</style>
