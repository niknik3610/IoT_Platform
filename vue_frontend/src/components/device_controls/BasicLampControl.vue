<script setup lang="ts">
import { ref } from "vue";

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
    <div>
        <button @click="toggleButton">{{ currentButtonText }}</button>
    </div>
</template>

<style></style>
