<script setup lang="ts">
import { controlDevice } from "@/backend_calls/deviceControl";
import { frontend } from "@/generated/generated";
import { onMounted, ref, watch } from "vue";

const props = defineProps({
    deviceName: {
        required: true,
        type: String,
    },
    capabilities: {
        required: true,
        type: Array<frontend.types.DeviceCapabilityStatus>,
    },
    deviceUuid: {
        required: true,
        type: String,
    }
});

const activeCapabilities = ref<frontend.types.DeviceCapabilityStatus[]>([]);

onMounted(async () => {
    activeCapabilities.value = await calculateActiveCapabilities(
        props.capabilities,
    );

    watch(props.capabilities, async (newCapabilities) => {
        activeCapabilities.value =
            await calculateActiveCapabilities(newCapabilities);
    });
});

async function calculateActiveCapabilities(
    capabilities: frontend.types.DeviceCapabilityStatus[],
) {
    let activeCapilities = capabilities.filter((capability) => {
        if (capability.available) {
            return capability;
        }
    });

    return activeCapilities;
}

async function activateCapability(capability: string) {
    let result = await controlDevice(props.deviceUuid, capability);
    if (result.isOk()) {
        console.log(`${capability} activated succesfully`);
    }
    else if (result.isErr()) {
        console.error(`${props.deviceName} had error: ${result.error} while trying to activate capability ${capability}`);
    }
}
</script>

<template>
    <div class="lamp-control">
        <h2 class="title">{{ deviceName }}</h2>
        <div
            v-for="capability in activeCapabilities"
            v-bind:key="capability.capability"
            class="button-container"
        >
            <button @click="() => activateCapability(capability.capability)">{{ capability.capability }}</button>
        </div>
    </div>
</template>

<style scoped>
.lamp-control {
    display: flex;
    justify-content: center;
    flex-direction: column;
    padding: 20px;

    outline-style: solid;
    outline-color: hsla(160, 100%, 37%, 1);
    border-radius: 10px;
}
.title {
    color: white;
    padding-bottom: 20px;
}
.button-container {
    display: flex;
    justify-content: center;
    padding: 5px;
}
button {
    align-self: center;
}
</style>
