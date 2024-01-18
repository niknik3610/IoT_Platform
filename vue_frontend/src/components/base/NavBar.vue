<script setup lang="ts">
import { RouterLink } from "vue-router";
</script>

<template>
    <div class="nav-bar">
        <RouterLink to="/" class="router-item">Home</RouterLink>
        <RouterLink to="/about" class="router-item">About</RouterLink>
    </div>
</template>

<style scoped>
.nav-bar {
    display: flex;
    font-weight: bold;
    justify-content: center;
}
.router-item {
    padding: 10px;
    font-size: 24px;
}
</style>
