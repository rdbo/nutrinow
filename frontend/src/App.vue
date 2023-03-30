<script setup>
import { ref } from "vue";
import { RouterLink, RouterView } from "vue-router";
import { Bars3Icon } from "@heroicons/vue/20/solid";
import axios from "axios";

const session_id = ref(null);
const showNavItems = ref(false); // show navigation items on mobile

function logout() {
    let delete_session = () => {
        // remove session from client side
        $cookies.remove("session_id");
        session_id.value = null;
    }

    // tell the server to delete the session
    axios.post("/api/logout")
    .then(delete_session)
    .catch(delete_session);
}

function updateSession() {
    session_id.value = $cookies.get("session_id");
}

updateSession();

// Periodically update session cookie (the server can force a logout)
setInterval(updateSession, 100);
</script>

<template>
    <div class="min-h-screen flex flex-col">
        <header class="sticky top-0 z-50">
            <!-- click overlay for mobile -->
            <div @click="showNavItems = false" id="nav_overlay" :class="[showNavItems ? 'opacity-40' : 'opacity-0', { 'hidden': !showNavItems }]" class="md:hidden absolute top-0 h-screen w-screen bg-black"></div>

	    <nav class="relative bg-gray-100 border-gray-200 md:static">
		<div class="max-w-screen-xl flex flex-wrap items-center justify-between mx-auto md:flex-col">
                    <div class="flex flex-wrap items-center justify-between w-full mx-auto md:items-stretch md:p-0 md:flex-row">
                        <RouterLink :to="{ name: 'home' }" class="text-4xl p-2 drop-shadow-bold-sm flex items-center md:flex-col md:justify-center"><div class="flex"><img class="w-16 mr-1" src="@/assets/imgs/apple.svg"/> <span class="text-primary-100 flex items-center">Nutri</span><span class="text-secondary-100 flex items-center">Now</span></div></RouterLink>
                        <button @click="showNavItems = !showNavItems" data-collapse-toggle="navbar-default" type="button" class="inline-flex items-center p-2 ml-3 mr-4 text-sm text-gray-500 rounded-lg md:hidden hover:bg-gray-100 focus:outline-none focus:ring-2 focus:ring-gray-200" aria-controls="navbar-default" aria-expanded="false">
                            <Bars3Icon class="w-6 h-6"/>
                        </button>
                        <div :class="[showNavItems ? 'show-nav-items' : 'hide-nav-items']" class="p-4 bg-gray-100 w-full absolute top-full md:p-0 md:static md:flex md:w-auto text-lg md:flex md:flex-col" id="navbar-default">
                            <ul class="font-medium flex flex-col p-4 md:p-0 md:px-4 border border-gray-100 rounded-lg bg-white md:flex-row md:space-x-4 md:mt-0 md:border-0 md:bg-gray-100 md:grow">
                                <li>
                                    <RouterLink :session_id="test" :to="{ name: 'home' }" class="nav-item" aria-current="page">Home</RouterLink>
                                </li>
                                <li>
                                    <RouterLink :session_id="test" :to="{ name: 'about' }" class="nav-item" aria-current="page">About</RouterLink>
                                </li>
                                <li>
                                    <RouterLink :session_id="test" :to="{ name: 'foods' }" class="nav-item" aria-current="page">Foods</RouterLink>
                                </li>
                                <li v-if="!session_id">
                                    <RouterLink :session_id="test" :to="{ name: 'login' }" class="nav-item" aria-current="page">Login</RouterLink>
                                </li>
                                <li v-if="!session_id">
                                    <RouterLink :session_id="test" :to="{ name: 'register' }" class="nav-item" aria-current="page">Register</RouterLink>
                                </li>
                                <li v-if="session_id">
                                    <div @click="logout" class="cursor-pointer nav-item">Logout</div>
                                </li>
                            </ul>
                        </div>
                    </div>
		</div>
	    </nav>
        </header>

        <main class="grow flex flex-col">
            <RouterView @update-session="updateSession" :session_id="session_id"/>
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
    @apply md:flex md:flex-col;
}

.nav-item {
    @apply uppercase block py-2 pl-3 pr-4 text-gray-700 rounded md:rounded-none md:p-0 md:px-2 md:grow md:flex md:items-center md:border-b-8 md:border-gray-100;
    transition: all 0.2s ease-in-out;
}

.nav-item:hover {
    @apply border-gray-300 text-gray-500;
}

.nav-item.router-link-active {
    @apply bg-lime-400 text-gray-800 md:bg-transparent md:text-lime-500 md:border-lime-400;
}

#nav_overlay {
    transition: 0.2 all ease-in-out;
}

/* TODO: Ensure that 'max-width' is always the same as the size of an 'md' screen */
/* TODO: Only play scale down animation after scale up */
@media (max-width: 768px) {
    .show-nav-items {
        transform-origin: top;
        animation: nav-items-scale-up 0.4s ease-in-out forwards;
    }

    .hide-nav-items {
        transform-origin: top;
        animation: nav-items-scale-down 0.4s ease-in-out forwards;
    }
}

@keyframes nav-items-scale-up {
    0% {
        display: block;
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
        display: hidden;
    }
}
</style>
