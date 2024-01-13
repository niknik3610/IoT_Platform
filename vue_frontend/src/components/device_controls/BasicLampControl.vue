<script setup lang="ts">
import { ref } from "vue";

const props = defineProps({
    deviceName: {
        type: String,
        required: true,
    },
});


const lightTurnedOn = ref(false);
const currentButtonText = ref("Turn On");

function toggleButton() {
    if (lightTurnedOn.value) {
        lightTurnedOn.value = false;
        currentButtonText.value = "Turn On";
        return;
    }
    lightTurnedOn.value = true;
    currentButtonText.value = "Turn Off";
}
</script>

<template>
    <div class="lamp-control">
        <h2 class="title">{{ deviceName }}</h2>
        <div class="button-container">
            <button @click="toggleButton">{{ currentButtonText }}</button>
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
