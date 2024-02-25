<script setup lang="ts">
import { RouterView } from "vue-router";
function hello() {
    let hello = "hi";
}
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
    <footer>
        <nav class="footer-nav">
            <a
                href="https://github.com/niknik3610/IoT_Platform"
                target="_blank"
            >
                <img
                    src="@/assets/github-mark-white.svg"
                    width="40px"
                    height="40px"
                />
            </a>
        </nav>
    </footer>
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
header {
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
footer {
    position: absolute;
    bottom: 0px;
    left: 0px;
    width: 100%;

    display: flex;
}
.footer-nav {
    display: flex;
    justify-content: end;
    width: 100%;
    padding-right: 10px;
    padding-bottom: 10px;
}
</style>
