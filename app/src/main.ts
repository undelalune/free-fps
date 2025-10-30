import {createApp} from "vue";
import App from "./App.vue";
import {router} from "./router";
import i18n from "./i18n";
import {createPinia} from 'pinia'
import 'vfonts/OpenSans.css'

const app = createApp(App);
const pinia = createPinia();

app.use(pinia);
app.use(i18n);
app.use(router);

if (import.meta.env.PROD) {
    window.addEventListener('contextmenu', (e) => {
        e.preventDefault();
    });
}


app.mount('#app');

