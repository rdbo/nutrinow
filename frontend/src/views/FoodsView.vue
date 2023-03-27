<script setup>
import { ref } from "vue";
import { useRouter, useRoute } from "vue-router";
import axios from "axios";
import { MagnifyingGlassIcon } from "@heroicons/vue/20/solid";
import SearchFood from "../components/SearchFood.vue";
import ModalFoodViewer from "../components/ModalFoodViewer.vue";

const router = useRouter();
const route = useRoute();

const searchStatus = ref("");
const searchQuery = ref(null);
const searchResults = ref([]);

function searchFood(e) {
    e.preventDefault();
    searchResults.value = [];
    router.push({ name: 'foods', params: { 'foodName': searchQuery.value } });
    document.activeElement.blur(); // remove focus from search bar
    updateResults(searchQuery.value);
}

function updateResults(foodName) {
    searchStatus.value = "Searching for: " + foodName + "...";
    axios.get("/api/food_search/" + foodName)
    .then(function (response) {
        if (response.data.err) {
            // TODO: Handle error
            return;
        }

        if (!response.data.matches.length) {
            searchStatus.value = "No Matches Found";
            return;
        }

        searchStatus.value = "";
        searchResults.value = response.data.matches;
    })
    .catch(function (err) {
        // TODO: Handle error
    });
}

// search for food in case the URL parameter 'foodName' is set
if (route.params.foodName) {
    updateResults(route.params.foodName);
}

const food = {"id":1,"name":"Hummus, commercial","serving_id":1,"serving_base":100,"serving_amount":100,"serving_unit":"g","nutrients":[{"name":"Vitamin C","amount":0,"unit":"mg"},{"name":"Vitamin B1","amount":0.15,"unit":"μg"},{"name":"Vitamin B2","amount":0.115,"unit":"μg"},{"name":"Vitamin B9","amount":36,"unit":"μg"},{"name":"Vitamin K","amount":17.2,"unit":"μg"},{"name":"Saturated Fat","amount":2.22,"unit":"g"},{"name":"Protein","amount":7.35,"unit":"g"},{"name":"Fiber","amount":5.4,"unit":"g"},{"name":"Iron","amount":2.41,"unit":"mg"},{"name":"Magnesium","amount":71.1,"unit":"mg"},{"name":"Phosphorus","amount":166,"unit":"mg"},{"name":"Sodium","amount":438,"unit":"mg"},{"name":"Copper","amount":0.348,"unit":"mg"},{"name":"Manganese","amount":1.06,"unit":"mg"},{"name":"Vitamin A","amount":1,"unit":"μg"},{"name":"Unsaturated Fat","amount":13.850000000000001,"unit":"g"},{"name":"Vitamin B3","amount":0.948,"unit":"μg"},{"name":"Vitamin B5","amount":0.318,"unit":"μg"},{"name":"Vitamin B6","amount":0.143,"unit":"mg"},{"name":"Fats","amount":17.1,"unit":"g"},{"name":"Carbohydrates","amount":14.9,"unit":"g"},{"name":"Water","amount":58.7,"unit":"ml"},{"name":"Calcium","amount":41,"unit":"mg"},{"name":"Potassium","amount":289,"unit":"mg"},{"name":"Zinc","amount":1.38,"unit":"mg"},{"name":"Selenium","amount":16.2,"unit":"mg"},{"name":"Vitamin E","amount":1.74,"unit":"mg"},{"name":"Sugars","amount":0.34,"unit":"g"}]};
</script>

<template>
    <ModalFoodViewer :food="food"/>
    <div class="text-2xl text-gray-700 flex flex-col items-center px-2 mb-16" :class="{ 'pt-48': searchResults.length <= 0 }">
        <div class="w-full max-w-2xl">
            <h1 class="text-6xl my-2 text-center">Foods</h1>
            <div class="w-full flex justify-center">
                <form @submit="searchFood" class="text-2xl flex rounded-md overflow-hidden border-gray-400 border-2 grow">
                    <input v-model="searchQuery" class="bg-red-50 px-2 w-full" placeholder="Search for a food"/>
                    <button class="bg-orange-300 px-4 py-2 border-gray-400 border-l-2"><MagnifyingGlassIcon class="text-yellow-700 w-9"/></button>
                </form>
            </div>
            <div v-if="$route.params.foodName">
                <div v-if="searchResults.length > 0" class="my-2">
                    <h2 class="text-gray-500 text-center my-2">Search Results</h2>
                    <div class="border-gray-700 border-2 border-b-0 rounded-t-md overflow-hidden">
                        <SearchFood v-for="food in searchResults" :food="food"/>
                    </div>
                </div>
                <div v-else-if="searchStatus">
                    <h2 class="text-gray-700 text-center text-4xl my-4">{{ searchStatus }}</h2>
                </div>
            </div>
        </div>
    </div>
</template>

<style scoped>
</style>
