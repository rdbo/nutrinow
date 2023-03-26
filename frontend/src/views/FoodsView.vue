<script setup>
import { ref } from "vue";
import axios from "axios";
import { MagnifyingGlassIcon } from '@heroicons/vue/20/solid';
import SearchFood from "../components/SearchFood.vue";

const searchQuery = ref(null);
const searchResults = ref([]);

function updateResults() {
    axios.get("/api/food_search/" + searchQuery.value)
    .then(function (response) {
        if (response.data.err) {
            // TODO: Handle error
            return;
        }

        searchResults.value = response.data.matches;
        console.log(searchResults.value);
    })
    .catch(function (err) {
        // TODO: Handle error
    });
}
</script>

<template>
    <div class="text-2xl text-gray-700 flex flex-col items-center px-2 mb-16" :class="{ 'pt-48': searchResults.length <= 0 }">
        <div class="w-full max-w-2xl">
            <h1 class="text-6xl my-2 text-center">Foods</h1>
            <div class="w-full flex justify-center">
                <div class="text-2xl flex rounded-md overflow-hidden border-gray-400 border-2 grow">
                    <input v-model="searchQuery" class="bg-red-50 px-2 w-full" placeholder="Search for a food"/>
                    <button @click="updateResults" class="bg-orange-300 px-4 py-2 border-gray-400 border-l-2"><MagnifyingGlassIcon class="text-yellow-700 w-9"/></button>
                </div>
            </div>
            <div v-if="searchResults.length > 0" class="my-2">
                <h2 class="text-gray-500 text-center my-2">Search Results</h2>
                <div class="border-gray-700 border-2 border-b-0 rounded-t-md overflow-hidden">
                    <SearchFood v-for="food in searchResults" :food="food"/>
                </div>
            </div>
        </div>
    </div>
</template>

<style scoped>
</style>
