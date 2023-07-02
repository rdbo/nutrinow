<script setup>
import { ref } from "vue";
import { RouterLink, RouterView } from "vue-router";
import { useErrorStore } from "@/stores/error";
import { useSessionStore } from "@/stores/session";
import { useProfileStore } from "@/stores/profile";
import ErrorMessage from "@/components/ErrorMessage.vue";
import { Bars3Icon } from "@heroicons/vue/20/solid";
import { api_get, api_post } from "./composables/api_request.js";

const errorStore = useErrorStore();
const sessionStore = useSessionStore();
const profileStore = useProfileStore();

const showNavItems = ref(false); // show navigation items on mobile

function logout() {
    let delete_session = () => {
        // remove session from client side
        $cookies.remove("session_id");
        sessionStore.id = null;
    }

    // tell the server to delete the session
    api_post("logout", null, delete_session, delete_session);
}

function updateSession() {
    sessionStore.id = $cookies.get("session_id");
}


function updateUserInfo() {
    api_get("user", null,
        (data) => {
            profileStore.name = data.name;
            profileStore.birthdate = data.birthdate;
            profileStore.weight = data.weight;
            profileStore.gender = data.gender;
        }
    );
}

updateSession();
if (sessionStore.id) {
    updateUserInfo();
}

// Periodically update session cookie (the server can force a logout)
setInterval(updateSession, 100);
</script>

<template>
    <div class="min-h-screen flex flex-col">
        <header class="sticky top-0 z-50">
            <!-- click overlay for mobile -->
            <div @click="showNavItems = false" :class="[showNavItems ? 'show-nav-overlay' : 'hide-nav-overlay']" class="lg:hidden absolute top-0 h-screen w-screen bg-black"></div>

	    <nav class="relative bg-gray-100 border-gray-200 lg:static">
		<div class="max-w-screen-xl flex flex-wrap items-center justify-between mx-auto lg:flex-col">
                    <div class="flex flex-wrap items-center justify-between w-full mx-auto lg:items-stretch lg:p-0 lg:flex-row">
                        <RouterLink :to="{ name: 'home' }" class="text-4xl p-2 drop-shadow-bold-sm flex items-center lg:flex-col lg:justify-center"><div class="flex"><img class="w-16 mr-1" src="@/assets/imgs/apple.svg"/> <span class="text-primary-100 flex items-center">Nutri</span><span class="text-secondary-100 flex items-center">Now</span></div></RouterLink>
                        <button @click="showNavItems = !showNavItems" data-collapse-toggle="navbar-default" type="button" class="inline-flex items-center p-2 ml-3 mr-4 text-sm text-gray-500 rounded-lg lg:hidden hover:bg-gray-100 focus:outline-none focus:ring-2 focus:ring-gray-200" aria-controls="navbar-default" aria-expanded="false">
                            <Bars3Icon class="w-6 h-6"/>
                        </button>
                        <div :class="[showNavItems ? 'show-nav-items' : 'hide-nav-items']" class="p-4 bg-gray-100 w-full absolute top-full lg:p-0 lg:static lg:flex lg:w-auto text-lg lg:flex lg:flex-col" id="navbar-default">
                            <ul class="font-medium flex flex-col p-4 lg:p-0 lg:px-4 border border-gray-100 rounded-lg bg-white lg:flex-row lg:space-x-4 lg:mt-0 lg:border-0 lg:bg-gray-100 lg:grow">
                                <li>
                                    <RouterLink :to="{ name: 'home' }" class="nav-item" aria-current="page">
                                        <div class="flex items-center w-fit">
                                            <div class="h-10 w-10 rounded-full overflow-hidden mx-2">
                                                <img src="@/assets/imgs/home.svg"/>
                                            </div>
                                            <p>Home</p>
                                        </div>
                                    </RouterLink>
                                </li>
                                <li>
                                    <RouterLink :to="{ name: 'foods' }" class="nav-item" aria-current="page">
                                        <div class="flex items-center w-fit">
                                            <div class="h-10 w-10 rounded-full overflow-hidden mx-2">
                                                <img src="@/assets/imgs/strawberry.svg"/>
                                            </div>
                                            <p>Foods</p>
                                        </div>
                                    </RouterLink>
                                </li>
                                <li v-if="!sessionStore.id">
                                    <RouterLink :to="{ name: 'about' }" class="nav-item" aria-current="page">
                                        <div class="flex items-center w-fit">
                                            <div class="h-10 w-10 rounded-full overflow-hidden mx-2">
                                                <img src="@/assets/imgs/question.svg"/>
                                            </div>
                                            <p>About</p>
                                        </div>
                                    </RouterLink>
                                </li>
                                <li v-if="!sessionStore.id">
                                    <RouterLink :to="{ name: 'login' }" class="nav-item" aria-current="page">
                                        <div class="flex items-center w-fit">
                                            <div class="h-10 w-10 overflow-hidden mx-2">
                                                <img src="@/assets/imgs/login.svg"/>
                                            </div>
                                            <p>Login</p>
                                        </div>
                                    </RouterLink>
                                </li>
                                <li v-if="!sessionStore.id">
                                    <RouterLink :to="{ name: 'register' }" class="nav-item" aria-current="page">
                                        <div class="flex items-center w-fit">
                                            <div class="h-10 w-10 overflow-hidden mx-2">
                                                <img src="@/assets/imgs/create.svg"/>
                                            </div>
                                            <p>Register</p>
                                        </div>                                    
                                    </RouterLink>
                                </li>
                                <li v-if="sessionStore.id">
                                    <RouterLink :to="{ name: 'about' }" class="nav-item" aria-current="page">
                                        <div class="flex items-center w-fit">
                                            <div class="h-10 w-10 rounded-full overflow-hidden border-2 border-blue-300 mx-2">
                                                <img v-if="profileStore.gender == 'F'" src="@/assets/imgs/female.svg" class="origin-top scale-150"/>
                                                <img v-else src="@/assets/imgs/male.svg" class="origin-top scale-150"/>
                                            </div>
                                            <p>{{ profileStore.name }}</p>
                                        </div>
                                    </RouterLink>
                                </li>
                                <li v-if="sessionStore.id">
                                    <div class="nav-item cursor-pointer">
                                        <div @click="logout" class="flex items-center w-fit">
                                            <div class="h-10 w-10 rounded-full overflow-hidden mx-2">
                                                <img src="@/assets/imgs/leave.svg"/>
                                            </div>
                                            <p>Logout</p>
                                        </div>
                                    </div>
                                </li>
                            </ul>
                        </div>
                    </div>
		</div>
	    </nav>
        </header>

        <main class="grow flex flex-col">
            <RouterView/>
            <ErrorMessage v-if="errorStore.msgs.length > 0" @close="errorStore.msgs = []" :msgs="errorStore.msgs"/>
        </main>
    </div>

    <footer class="py-4 bg-gray-300 flex flex-col justify-center items-center text-center">
        <p>Copyright © Rdbo</p>
        <p>All Rights Reserved</p>
        <p>This website is licensed under the <a class="text-green-500" href="https://www.gnu.org/licenses/agpl-3.0.en.html">GNU AGPLv3.0</a></p>
    </footer>
</template>

<style scoped>
li {
    @apply lg:flex lg:flex-col;
}

.nav-item {
    @apply uppercase block py-2 pl-3 pr-4 text-gray-700 rounded lg:rounded-none lg:p-0 lg:px-2 lg:grow lg:flex lg:items-center lg:border-b-8 lg:border-gray-100;
    transition: all 0.2s ease-in-out;
}

.nav-item:hover {
    @apply border-gray-300 text-gray-500;
}

.nav-item.router-link-active {
    @apply bg-orange-100 text-gray-800 lg:bg-transparent lg:border-orange-300;
}

/* TODO: Ensure that 'max-width' is always the same as the size of an 'lg' screen */
/* TODO: Only play animations after the menu has been already activated (not on page load) */
@media (max-width: 1023px) {
    .show-nav-items {
        transform-origin: top;
        animation: nav-items-scale-up 0.4s ease-in-out forwards;
    }

    .hide-nav-items {
        transform-origin: top;
        animation: nav-items-scale-down 0.4s ease-in-out forwards;
    }

    .show-nav-overlay {
        animation: nav-overlay-fade-in 0.4s ease-in-out forwards;
    }

    .hide-nav-overlay {
        animation: nav-overlay-fade-out 0.4s ease-in-out forwards;
    }
}

@keyframes nav-items-scale-up {
    0% {
        transform: scale(100%, 0%);
    }
    100% {
        transform: scale(100%, 100%);
    }
}

@keyframes nav-items-scale-down {
    0% {
        transform: scale(100%, 100%);
    }
    100% {
        transform: scale(100%, 0%);
    }
}

@keyframes nav-overlay-fade-in {
    0% {
        transform: scale(100%);
        display: block;
        opacity: 0%;
    }
    100% {
        opacity: 40%;
    }
}

@keyframes nav-overlay-fade-out {
    0% {
        opacity: 40%;
    }
    99% {
        opacity: 0%;
        transform: scale(100%);
    }
    100% {
        transform: scale(0%);
    }
}
</style>
