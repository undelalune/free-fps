import {createMemoryHistory, createRouter} from 'vue-router'
import {paths} from "@/router/paths.ts";
import MainView from "../views/MainView.vue";
import SettingsView from "@/views/SettingsView.vue";


const routes = [
    {path: paths.main, component: MainView},
    {path: paths.settings, component: SettingsView},
]

export const router = createRouter({
    history: createMemoryHistory(),
    routes,
})