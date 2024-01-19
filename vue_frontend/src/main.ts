import "./assets/main.css";

import { createApp } from "vue";
import { createPinia } from "pinia";

import App from "@/App.vue";
import NavBar from "@/components/base/NavBar.vue";
import GenericIotDevice from "@/components/GenericIotDevice.vue"
import router from "@/router";

const app = createApp(App);

app.use(createPinia());
app.use(router);

app.component("NavBar", NavBar);
app.component("GenericIotDevice", GenericIotDevice);

app.mount("#app");
