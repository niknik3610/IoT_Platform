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
    font-weight: bold;
    font-size: large;
    align-items: center;
    display: flex;
}
.router-item {
    padding: 10px;
}
</style>
