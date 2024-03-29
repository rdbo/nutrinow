import { createRouter, createWebHistory } from 'vue-router';
import HomeView from '../views/HomeView.vue';
import AboutView from '../views/AboutView.vue';
import LoginView from '../views/LoginView.vue';
import PageNotFound from '../views/PageNotFound.vue';
import RegisterView from '../views/RegisterView.vue';
import FoodsView from '../views/FoodsView.vue';

const router = createRouter({
    history: createWebHistory(import.meta.env.BASE_URL),
    routes: [
        {
            path: '/',
            name: 'home',
            component: HomeView,
            props: true
        },
        {
            path: '/about',
            name: 'about',
            component: AboutView
        },
        {
            path: '/login',
            name: 'login',
            component: LoginView
        },
        {
            path: '/register',
            name: 'register',
            component: RegisterView
        },
        {
            path: '/foods/:foodName(.*)*',
            name: 'foods',
            component: FoodsView
        },
        {
            path: '/:invalidPage(.*)',
            name: 'not_found',
            component: PageNotFound
        }
    ]
});

export default router;
