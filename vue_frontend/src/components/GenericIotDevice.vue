<script setup lang="ts">
import { frontend } from "@/generated/generated"
import { onMounted, ref } from "vue";

const props = defineProps({
    deviceName: {
        required: true,
        type: String,
    },
    capabilities: {
        required: true,
        type: Array<frontend.types.DeviceCapabilityStatus>
    }
});

const activeCapabilities = ref<frontend.types.DeviceCapabilityStatus[]>([])

onMounted(() => {
    props.capabilities.forEach((capability) => {
        if (capability.available) {
            console.log(capability);
            activeCapabilities.value!.push(capability);
        }
    });
});
</script>

<template>
    <div class="lamp-control">
        <h2 class="title">{{ deviceName }}</h2>
        <div v-for="capability in activeCapabilities" v-bind:key="capability.capability" class="button-container">
            <button @click="() => {}">{{ capability.capability }}</button>
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
