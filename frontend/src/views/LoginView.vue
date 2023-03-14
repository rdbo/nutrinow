<script setup>
import { useRouter } from 'vue-router';

const router = useRouter();
</script>

<template>
    <div class="max-w-lg mx-auto my-4 flex flex-col justify-center items-center bg-secondary-100 border-2 border-gray-700 px-4 py-4 rounded-md text-gray-700">
        <h1 class="text-4xl">Login</h1>
        <form method="POST" action="/api/login" class="flex flex-col">
            <div>
                <label>E-Mail:</label>
                <input name="email" type="email" required/>
            </div>
            <div>
                <label>Password:</label>
                <input name="password" type="password" required/>
            </div>
            <button class="text-2xl py-2 px-2 my-2 border-2 border-gray-700 rounded-md bg-amber-500">Log-in</button>
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
