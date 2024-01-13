<script setup lang="ts">
import { RouterView } from "vue-router";
</script>

<template>
    <div class="screen">
        <header>
            <nav>
                <NavBar />
            </nav>
        </header>
        <div class="router-view">
            <RouterView></RouterView>
        </div>
    </div>
</template>

<style scoped>
.screen {
    display: flex;
    flex-direction: column;
    width: 100%;
    height: 100%;
    position: absolute;
    top: 0px;
    left: 0px;
}
nav {
    width: 100%;
    align-self: left;
    padding-top: 20px;
    padding-bottom: 20px;
}
.router-view {
    display: flex;
    flex-direction: column;
    width: 100%;
    height: 100%;
    justify-items: center;
}
</style>
