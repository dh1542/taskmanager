import { createApp } from "vue";
import { aliases, mdi } from 'vuetify/iconsets/mdi'
import App from "./App.vue";

// Vuetify
import 'vuetify/styles'
import { createVuetify } from 'vuetify'
import * as components from 'vuetify/components'
import * as directives from 'vuetify/directives'

const vuetify = createVuetify({
    icons: {
        defaultSet: 'mdi',
        aliases,
        sets: {
            mdi
        }
    },
    components,
    directives,
})

const app = createApp(App);



app.use(vuetify).mount("#app");
