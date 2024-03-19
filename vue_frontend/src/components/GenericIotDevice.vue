<script lang="ts">
    class ActiveCapabilities {
        buttons: frontend.types.DeviceCapabilityStatus[] = []
        sliders: frontend.types.DeviceCapabilityStatus[] = []
    }
</script>

<script setup lang="ts">
import { controlDevice } from "@/backend_calls/deviceControl";
import { frontend } from "@/generated/generated";
import { onMounted, ref, watch } from "vue";

const props = defineProps({
    deviceInfo: {
        required: true,
        type: frontend.registration.Device,
    },
});

const activeCapabilities = ref(new ActiveCapabilities());
const deviceName = ref<String>("");
const capabilities = ref<Array<frontend.types.DeviceCapabilityStatus>>([])
const deviceUuid = ref<String>("");

const capabilitiesValueMap = ref<Map<String, number>>(new Map());

onMounted(async () => {
    deviceName.value =  props.deviceInfo.device_name
    capabilities.value =  props.deviceInfo.capabilities
    deviceUuid.value =  props.deviceInfo.device_uuid

    activeCapabilities.value = await calculateActiveCapabilities(
        capabilities.value,
    );

    watch(() => props.deviceInfo, async (newDeviceInfo) => {
        const newCapabilities = newDeviceInfo.capabilities;
        activeCapabilities.value = await calculateActiveCapabilities(newCapabilities);
    });
});

async function calculateActiveCapabilities(
    capabilities: frontend.types.DeviceCapabilityStatus[],
) {
    let activeCapability = capabilities.filter((capability) => {
        if (capability.available) {
            return capability;
        }
    });
    
    let toReturn = new ActiveCapabilities(); 
    
    for (const capability of activeCapability) {
        switch (capability.type)  {
            case frontend.types.DeviceCapabilityType.BUTTON:
                toReturn.buttons.push(capability);
                break;
            case frontend.types.DeviceCapabilityType.SLIDER:
                toReturn.sliders.push(capability);
                let capabilityValue = capabilitiesValueMap.value.get(capability.capability);
                if (capabilityValue === null) {
                    capabilityValue = 50;
                    capabilitiesValueMap.value.set(capability.capability, capabilityValue);
                }

                capability.value = capabilityValue

                break;
            default:
                console.error("Unknown Capability type " + capability.type)
        }
    }

    return toReturn;
}

async function activateCapability(capability: DeviceCapabilityStatus) {
    let result = await controlDevice(deviceUuid.value, capability.capability, capabilitiesValueMap.value[capability.capability]);
    if (result.isOk()) {
        console.log(`${capability.capability} activated succesfully`);
    } else if (result.isErr()) {
        console.error(
            `${props.deviceName} had error: ${result.error} while trying to activate capability ${capability}`,
        );
    }
}
</script>

<template>
    <div class="device-container">
        <h2 class="title">{{ deviceName }}</h2>
        <div
            v-for="capability in activeCapabilities.buttons"
            v-bind:key="capability.capability"
            class="button-container"
        >
            <button class="capability-button" @click="activateCapability(capability)">
                <p style="color: black">{{ capability.capability }}</p>
            </button>
        </div>
        <div
            v-for="capability in activeCapabilities.sliders"
            v-bind:key="capability.capability"
            class="slider-container"
        >
            <p style="color: black; text-align: center; padding-bottom: 3px;">{{ capability.capability }} Slider</p>
            <div style="display: flex;">
                <input type="range" min="1" max="100" value="50" class="slider" v-model="capabilitiesValueMap[capability.capability]">
                <button style="margin-left: 5px;" @click="activateCapability(capability)">Submit</button>
            </div>
        </div>

    </div>
</template>

<style scoped>
.device-container {
    display: flex;
    min-width: 200px;
    justify-content: center;
    flex-direction: column;
    padding: 20px;
    justify-content: center;
    outline-style: solid;
    outline-color: hsla(160, 100%, 37%, 1);
    border-radius: 10px;
}
.title {
    text-align: center;
    color: white;
    padding-bottom: 20px;
    color: white;
}
.button-container {
    display: flex;
    justify-content: center;
    padding: 5px;
    padding-bottom: 15px;
}
button {
    align-self: center;
}
.capability-button {
    background-color: white;

    padding: 9px 14px;

    text-align: center;
    text-decoration: none;
    display: inline-block;
    font-size: 16px;

    border: 4px solid #00BD7E;
    border-radius: 10px;
}
button:active { 
    transform: scale(0.95); 
}
.slider-container {
    text-align: center;
    text-decoration: none;
    display: inline-block;
    font-size: 16px;

    display: flex;
    flex-direction: column;
    justify-content: center;
    padding-bottom: 15px;

    border: 4px solid #00BD7E;
    border-radius: 10px;
    background-color: white;
    padding: 5px;
}
</style>
