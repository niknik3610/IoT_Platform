<script setup lang="ts">
import type { IFrontendRegistrationServiceService_IRegister } from "@/generated/registrationService_grpc_pb";
import { RegistrationRequest } from "@/generated/registrationService_pb"

class RegistrationServer implements IFrontendRegistrationServiceService_IRegister {

}
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
