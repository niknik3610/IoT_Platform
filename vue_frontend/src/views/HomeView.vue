<script setup lang="ts">
import { onMounted } from "vue";
import { registerSelf } from "@/backend_calls/registration"

onMounted(async () => {
    //todo: add to a definitions file
    let server_ip = "127.0.0.1:50052";
    let response = await registerSelf(server_ip, "testDevice");
    console.log(response);
});
</script>

<template>
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
