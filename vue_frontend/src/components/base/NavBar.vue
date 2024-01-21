<script setup lang="ts">
import { RouterLink } from "vue-router";
</script>

<template>
    <div class="nav-bar">
        <RouterLink to="/" class="active router-item">Home</RouterLink>
        <RouterLink to="/about" class="active router-item">About</RouterLink>
    </div>
</template>

<style scoped>
.active {
    text-decoration: none;
    color: hsla(160, 100%, 37%, 1);
    transition: 0.4s;
    padding: 3px;
}

@media (hover: hover) {
    a:hover {
        background-color: hsla(160, 100%, 37%, 0.2);
    }
}

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
