<script setup>
import { RouterLink, RouterView } from 'vue-router'
</script>

<template>
    <header class="sticky top-0">
            <nav class="flex font-bold bg-gray-100 items-center">
                <RouterLink :to="{name: 'home'}" class="text-4xl px-2 drop-shadow-bold-sm flex items-center"><img class="w-16 mr-1" src="@/assets/imgs/apple.svg"/> <span class="text-primary-100">Nutri</span><span class="text-secondary-100">Now</span></RouterLink>
                <div class="grow uppercase text-lg flex justify-center">
                    <RouterLink :to="{name: 'home'}" class="nav-item">Home</RouterLink>
                    <RouterLink :to="{name: 'about'}" class="nav-item">About</RouterLink>
                </div>
            </nav>
    </header>

    <main>
        <RouterView/>
    </main>
</template>

<style scoped>
.nav-item {
    @apply px-4 py-4 text-gray-700 border-b-8 border-gray-100;
    transition: all 0.2s ease-in-out;
}

.nav-item:hover {
    @apply text-gray-500 border-gray-200;
}

.nav-item.router-link-active {
    @apply text-gray-700 border-b-8 border-primary-100;
}
</style>
