import { createRouter, createWebHistory } from "vue-router";
import HomeView from "../views/HomeView.vue";

const routes = [
    {
        path: "/",
        name: "home",
        component: HomeView
    },
    {
        path: "/stats",
        name: "stats",
        component: () => import("../views/StatsView.vue")
    },
    {
        path: "/skills",
        name: "skills",
        component: () => import("../views/SkillsView.vue")
    },
    {
        path: "/items",
        name: "items",
        component: () => import("../views/ItemsView.vue")
    },
    {
        path: "/quests",
        name: "quests",
        component: () => import("../views/QuestsView.vue")
    },
    {
        path: "/waypoints",
        name: "waypoints",
        component: () => import("../views/WaypointsView.vue")
    }
];

const router = createRouter({
    history: createWebHistory(),
    routes
});

export default router;
