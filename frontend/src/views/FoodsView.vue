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
const selectedFood = ref(null);

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

const food = {"id":838,"name":"Chicken breast, baked, coated, skin / coating not eaten","servings":[{"id":3017,"amount":100,"unit":"g","nutrients":[{"name":"Protein","amount":29.6,"unit":"g"},{"name":"Fats","amount":5.45,"unit":"g"},{"name":"Carbohydrates","amount":0,"unit":"g"},{"name":"Water","amount":62.8,"unit":"ml"},{"name":"Fiber","amount":0,"unit":"g"},{"name":"Calcium","amount":7,"unit":"mg"},{"name":"Iron","amount":0.44,"unit":"mg"},{"name":"Magnesium","amount":28,"unit":"mg"},{"name":"Phosphorus","amount":224,"unit":"mg"},{"name":"Potassium","amount":352,"unit":"mg"},{"name":"Sodium","amount":353,"unit":"mg"},{"name":"Zinc","amount":0.9,"unit":"mg"},{"name":"Copper","amount":0.046,"unit":"mg"},{"name":"Selenium","amount":30,"unit":"mg"},{"name":"Vitamin A","amount":9,"unit":"μg"},{"name":"Vitamin E","amount":1.06,"unit":"mg"},{"name":"Vitamin D","amount":0,"unit":"μg"},{"name":"Vitamin C","amount":0,"unit":"mg"},{"name":"Vitamin B1","amount":0.086,"unit":"μg"},{"name":"Vitamin B2","amount":0.21,"unit":"μg"},{"name":"Vitamin B3","amount":10.1,"unit":"μg"},{"name":"Vitamin B6","amount":0.855,"unit":"mg"},{"name":"Vitamin B9","amount":7,"unit":"μg"},{"name":"Vitamin B12","amount":0.18,"unit":"μg"},{"name":"Vitamin K","amount":2.2,"unit":"μg"},{"name":"Saturated Fat","amount":1.01,"unit":"g"},{"name":"Unsaturated Fat","amount":3.0700000000000003,"unit":"g"}],"relative":null},{"id":3018,"amount":120,"unit":"1 breast, NS as to size","nutrients":[],"relative":3017},{"id":3019,"amount":135,"unit":"1 large breast","nutrients":[],"relative":3017},{"id":3020,"amount":30,"unit":"1 small or thin slice","nutrients":[],"relative":3017},{"id":3021,"amount":105,"unit":"1 small breast","nutrients":[],"relative":3017},{"id":3022,"amount":135,"unit":"1 cup, cooked, diced","nutrients":[],"relative":3017},{"id":3023,"amount":85,"unit":"1 large or thick slice","nutrients":[],"relative":3017},{"id":3024,"amount":120,"unit":"1 medium breast","nutrients":[],"relative":3017},{"id":3025,"amount":155,"unit":"1 breast quarter (yield after cooking, bone removed)","nutrients":[],"relative":3017},{"id":3026,"amount":28.35,"unit":"1 oz, cooked","nutrients":[],"relative":3017},{"id":3027,"amount":60,"unit":"1 medium slice","nutrients":[],"relative":3017}]};
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
                        <SearchFood v-for="food in searchResults" :food="food" @select-food="selectedFood = food"/>
                        <ModalFoodViewer v-if="selectedFood" :food="selectedFood" @close="selectedFood = null"/>
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
