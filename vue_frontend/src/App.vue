<script setup lang="ts">
import { RouterLink, RouterView } from "vue-router";
</script>

<template>
    <div class="screen">
        <header>
            <nav class="nav-bar">
                <NavBar />
            </nav>
        </header>
        <div>
            <RouterView></RouterView>
        </div>
    </div>
</template>

<style scoped>
.screen {
    display: flex;
    align-items: start;
    flex-direction: column;
    width: 100%;
    height: 100%;
}
.nav-bar {
    width: 100%;
    position: absolute;
    top: 20px;
}
</style>
