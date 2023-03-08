<script setup>
import { useRouter } from 'vue-router';
const router = useRouter();
</script>

<template>
    <div class="flex flex-col justify-center items-center">
        <h1 class="text-gray-700 text-6xl my-4">Page Not Found!</h1>
        <button @click="router.push({ name: 'home' })" class="text-4xl bg-primary-100 px-4 py-4 rounded-lg border-2 border-gray-700 text-gray-700">Return to home page</button>
    </div>
</template>

<style scoped>
button {
    transition: all 0.2s ease-in-out;
}

button:hover {
    @apply bg-secondary-100;
}
</style>
