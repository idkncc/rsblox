import "victormono"
import 'tippy.js/animations/shift-away-subtle.css';

import {createPinia} from "pinia";
import {createRouter, createWebHashHistory} from "vue-router";
import {createApp} from "vue";


import IndexView from "./pages/index.vue";
import GamePage from "./pages/game-page.vue";

import App from "./App.vue";

const routes = [
    {
        path: '/',
        component: IndexView,
    },
    {
        path: '/games/:id',
        component: GamePage,
    }
]

const pinia = createPinia()
const router = createRouter({
    history: createWebHashHistory(),
    routes,
})

createApp(App)
    .use(pinia)
    .use(router)
    .mount("#app");
