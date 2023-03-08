<script setup>
import { useRouter } from "vue-router";
const router = useRouter();
</script>

<template>
    <div class="py-8 flex flex-col justify-center items-center text-center">
        <h1 class="text-6xl text-gray-800">Welcome to NutriNow</h1>
        <h2 class="text-4xl text-gray-700">Make the diet of your dreams</h2>
        <!--
            File: healthy-foods.png
            License: CC BY-NC 4.0 - https://creativecommons.org/licenses/by-nc/4.0/
            Source: https://www.pngall.com/diet-png/download/53680
        -->
        <img src="@/assets/imgs/healthy-foods.png" class="w-96 py-4"/>
        <button @click="router.push({ name: 'login' })" class="text-gray-800 border-2 border-gray-700 uppercase text-2xl bg-primary-100 px-4 py-4 rounded-md hover:bg-secondary-100">Get Started</button>
    </div>
</template>

<style scoped>
button {
    transition: all 0.2s ease-in-out;
}

button:hover {
    @apply bg-secondary-100 scale-125;
}
</style>
