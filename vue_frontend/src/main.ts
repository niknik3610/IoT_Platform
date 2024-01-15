import "./assets/main.css";

import { createApp } from "vue";
import { createPinia } from "pinia";

import App from "@/App.vue";
import NavBar from "@/components/base/NavBar.vue";
import BasicLampControl from "@/components/device_controls/BasicLampControl.vue";
import router from "@/router";

const app = createApp(App);

app.use(createPinia());
app.use(router);

app.component("NavBar", NavBar);
app.component("BasicLampControl", BasicLampControl);

app.mount("#app");
