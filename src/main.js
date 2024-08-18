import { createApp } from "vue";
import "./style.css";
import App from "./App.vue";
import router from "./routers.js";
import store from "./store.js";

// Vuetify
import "@fortawesome/fontawesome-free/css/all.css";
import "vuetify/styles";
import "./assets/global.css";
import { createVuetify } from "vuetify";
import { aliases, fa } from "vuetify/iconsets/fa";
import * as components from "vuetify/components";
import * as directives from "vuetify/directives";

const vuetify = createVuetify({
  icons: {
    defaultSet: "fa",
    aliases,
    sets: {
      fa,
    },
  },
  components,
  directives,
});

const app = createApp(App);
app.use(vuetify);
app.use(router);
app.use(store);
app.mount("#app");
