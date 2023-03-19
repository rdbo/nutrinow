<script setup>
import { ref } from "vue";
import { RouterLink, RouterView } from "vue-router";
import axios from "axios";

const session_id = ref(null);

function logout() {
    // tell the server to delete the session
    let logoutData = new FormData();
    logoutData.append("session_id", session_id.value);
    axios.post("/api/logout", logoutData)
    .catch(function (err) {
        console.log("/api/logout request error: " + err);
    });

    // remove session from client side
    $cookies.remove("session_id");
    session_id.value = null;
}

function updateSession() {
    session_id.value = $cookies.get("session_id");
}

updateSession();

// Periodically update session cookie
setInterval(() => {
    updateSession();
}, 100);
</script>

<template>
    <div class="min-h-screen">
        <header class="sticky top-0 z-50">
                <nav class="flex font-bold bg-gray-100 items-center flex-col md:flex-row">
                    <RouterLink :to="{ name: 'home' }" class="text-4xl px-2 drop-shadow-bold-sm flex items-center"><img class="w-16 mr-1" src="@/assets/imgs/apple.svg"/> <span class="text-primary-100">Nutri</span><span class="text-secondary-100">Now</span></RouterLink>
                    <div class="grow text-lg flex justify-center">
                        <RouterLink :session_id="test" :to="{ name: 'home' }" class="nav-item">Home</RouterLink>
                        <RouterLink :to="{ name: 'about' }" class="nav-item">About</RouterLink>
                        <!-- TODO: Groud menus that don't show when user is logged in and vice-versa -->
                        <RouterLink v-if="!session_id" :to="{ name: 'login' }" class="nav-item">Login</RouterLink>
                        <RouterLink v-if="!session_id" :to="{ name: 'register' }" class="nav-item">Register</RouterLink>
                        <button v-if="session_id" @click="logout" class="nav-item">Logout</button>
                    </div>
                </nav>
        </header>

        <main>
            <RouterView @update-session="updateSession" :session_id="session_id"/>
        </main>
    </div>

    <footer class="py-4 bg-gray-300 flex flex-col justify-center items-center text-center">
        <p>Copyright © Rdbo</p>
        <p>All Rights Reserved</p>
        <p>This website is licensed under the <a class="text-blue-500" href="https://www.gnu.org/licenses/agpl-3.0.en.html">GNU AGPLv3.0</a></p>
    </footer>
</template>

<style scoped>
.nav-item {
    @apply px-4 py-4 text-gray-700 border-b-8 border-gray-100 uppercase;
    transition: all 0.2s ease-in-out;
}

.nav-item:hover {
    @apply text-gray-500 border-gray-200;
}

.nav-item.router-link-active {
    @apply text-gray-700 border-b-8 border-primary-100;
}
</style>
