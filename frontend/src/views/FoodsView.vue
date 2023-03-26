<script setup>
import { MagnifyingGlassIcon } from '@heroicons/vue/20/solid'
</script>

<template>
    <div class="text-2xl text-gray-700 flex flex-col items-center">
        <h1 class="text-6xl my-2 text-center">Foods</h1>
        <div class="w-full px-2 flex justify-center">
            <div class="text-2xl flex rounded-md overflow-hidden border-gray-400 border-2 max-w-xl grow">
                <input class="bg-red-50 px-2 w-full" placeholder="Search for a food"/>
                <button class="bg-orange-300 px-4 py-2 border-gray-400 border-l-2"><MagnifyingGlassIcon class="text-yellow-700 w-9"/></button>
            </div>
        </div>
        <div class="my-2">
            <h2 class="text-gray-500 text-center">Search Results</h2>
            <div>
            </div>
        </div>
    </div>
</template>

<style scoped>
</style>
