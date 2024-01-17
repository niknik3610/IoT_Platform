<script setup lang="ts">
import { onMounted, ref } from "vue";
import { useDeviceIdStore } from "@/stores/deviceIdStore";

const deviceIdStore = useDeviceIdStore();

onMounted(async () => {
    if (!deviceIdStore.device_id_valid) {
        deviceIdStore.registerSelf();
    }
});
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
