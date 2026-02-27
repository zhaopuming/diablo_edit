import { createApp } from "vue";
import { createPinia } from "pinia";
import router from "./router";
import App from "./App.vue";
import { attachConsole } from "@tauri-apps/plugin-log";

// Stream console logs to the terminal
attachConsole();
console.log("Diablo Edit2 Frontend Initialized");

const app = createApp(App);
app.use(createPinia());
app.use(router);
app.mount("#app");
