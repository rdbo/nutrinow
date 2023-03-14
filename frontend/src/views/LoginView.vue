<script setup>
import { ref } from "vue";
import { useRouter } from "vue-router";
import axios from "axios";

const router = useRouter();
const emailForm = ref(null);
const passwordForm = ref(null);
const waitingLogin = ref(false);

function loginHandler(e) {
    e.preventDefault(); // prevent redirection
    waitingLogin.value = true;

    let loginData = new FormData();
    loginData.append("email", emailForm.value.value);
    loginData.append("password", passwordForm.value.value);

    axios.post("/api/login", loginData)
    .then(function (response) {
        if (response.data.session_id) {
            $cookies.set("session_id", response.data.session_id, "1y");
            router.push({ name: "home" });
        } else {
            // TODO: Show error message to user
            console.log("failed to log in");
            waitingLogin.value = false;
        }
    }).catch(function (err) {
        // TODO: Handle error
        console.log("/api/login request error: " + err);
        waitingLogin.value = false;
    });
}
</script>

<template>
    <div class="max-w-lg mx-auto my-4 flex flex-col justify-center items-center bg-secondary-100 border-2 border-gray-700 px-4 py-4 rounded-md text-gray-700">
        <h1 class="text-4xl">Login</h1>
        <form @submit="loginHandler" method="POST" action="/api/login" class="flex flex-col">
            <div>
                <label>E-Mail:</label>
                <input ref="emailForm" name="email" type="email" required/>
            </div>
            <div>
                <label>Password:</label>
                <input ref="passwordForm" name="password" type="password" required/>
            </div>
            <button class="text-2xl py-2 px-2 my-2 border-2 border-gray-700 rounded-md bg-amber-500" :class="{ 'btn-waiting': waitingLogin }" :disabled="waitingLogin">
                <span v-if="!waitingLogin">Log-in</span>
                <span v-else>Logging in...</span>
            </button>
            <a @click="router.push({ name: 'register' })" class="text-xl text-blue-500 cursor-pointer border-secondary-100 border-b-2">Don't have an account? Register now!</a>
        </form>
    </div>
</template>

<style scoped>
label {
    @apply text-2xl;
}

input {
    @apply text-xl border-2 border-gray-700 px-1 rounded-md;
}

form div {
    @apply my-2 flex flex-col;
}

.btn-waiting {
    @apply bg-gray-200;
}

.btn-waiting:hover {
    @apply bg-gray-400;
}

button {
    transition: all 0.2s ease-in-out;
}

button:hover {
    @apply bg-amber-400 text-gray-600;
}

a {
    transition: all 0.2s ease-in-out;
}

a:hover {
    @apply border-blue-500;
}
</style>
