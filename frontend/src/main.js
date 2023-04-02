import { createApp } from "vue";
import VueCookies from "vue-cookies";
import App from "./App.vue";
import router from "./router";
import { createPinia } from "pinia";

import "./assets/main.css";

const pinia = createPinia();
const app = createApp(App);

app.use(router);
app.use(VueCookies);
app.use(pinia);

app.mount("#app");
