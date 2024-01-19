<script setup lang="ts">
import { onMounted, ref } from "vue";
import { useDeviceIdStore } from "@/stores/deviceIdStore";
import { getConnectedDevices } from "@/backend_calls/getConnectedDevices";
import { errAsync, okAsync, type Result } from "neverthrow";
import { RegistrationTypes } from "@/types"

const deviceIdStore = useDeviceIdStore();
const connectedDevices = ref<RegistrationTypes.ConnectedDevice[]>();

onMounted(async () => {
    if (!deviceIdStore.device_id_valid) {
        await deviceIdStore.registerSelf();
    }

    if (!deviceIdStore.device_id_valid) {
        console.error("device_id invalid");
        return;
    }
    
    proccessConnectedDevices().then((result) => {
        if (result.isOk()) {
            connectedDevices.value = result.value;
            console.log(connectedDevices.value[0]);
        }
        else if (result.isErr()) {
            console.error(result.error.message);
        }
        return [];
    });
});

async function proccessConnectedDevices(): Promise<Result<RegistrationTypes.ConnectedDevice[], Error>> {
    let response = await getConnectedDevices(deviceIdStore.device_id);

    if (response.isOk()) {
        let parsedDevices = response.value.devices.map((device) => new RegistrationTypes.ConnectedDevice(
        device.device_name!,
        device.capabilities!.map((capability) => {
            return capability.capability!;
        })));
        return okAsync(parsedDevices);
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
        <div v-for="device in connectedDevices">
            <p>device name: {{device.name}}</p>
            <p>device capabilities:</p>
            <p v-for="capability in device.capabilities">{{capability}}</p>
        </div>
        <div class="connected-devices-container">
            <BasicLampControl deviceName="Test Lamp Raspberry Pi 1" />
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
