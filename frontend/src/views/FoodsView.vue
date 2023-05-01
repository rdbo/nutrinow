<script setup>
import { ref } from "vue";
import { useRouter, useRoute } from "vue-router";
import axios from "axios";
import { useErrorStore } from "@/stores/error";
import { MagnifyingGlassIcon } from "@heroicons/vue/20/solid";
import SearchFood from "../components/SearchFood.vue";
import ModalFoodViewer from "../components/ModalFoodViewer.vue";

const router = useRouter();
const route = useRoute();
const errorStore = useErrorStore();

const searchStatus = ref("");
const searchQuery = ref(null);
const searchResults = ref([]);
const selectedFood = ref(null);
var controller = null;

function searchFood(e) {
    e.preventDefault();
    searchResults.value = [];
    router.push({ name: 'foods', params: { 'foodName': searchQuery.value } });
    document.activeElement.blur(); // remove focus from search bar
    updateResults();
}

function updateResults() {
    let foodName = searchQuery.value;
    searchStatus.value = "Searching for: " + foodName + "...";

    // Abort previous incomplete request
    if (controller) {
        controller.abort();
        controller = null;
    }

    controller = new AbortController();

    axios.get("/api/food_search/" + foodName, {
        signal: controller.signal
    })
    .then(function (response) {
        // Request finished, unset the abort controller
        controller = null;

        if (response.data.err) {
            searchStatus.value = "";
            errorStore.msgs.push(response.data.err);
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
        // Request was canceled, not an actual error
        if (err.code == "ERR_CANCELED")
            return;
        // Request finished, unset the abort controller
        controller = null;

        errorStore.msgs.push("Failed to connect to the server (/api/food_search/<foodName>)");
        searchStatus.value = "";
    }); 
}

function addFoodToMeal(servingId, amount) {
    // user not logged in
    if (!$cookies.get("session_id")) {
        router.push({ name: "login" });
        return;
    }

    let mealId = sessionStorage.getItem("meal_id");
    if (!mealId) {
        // TODO: Implement a way to add food directly on home page
        router.push({ name: "home" });
        return;
    }

    let addMealServingData = new FormData();
    addMealServingData.append("meal_id", mealId);
    addMealServingData.append("serving_id", servingId);
    addMealServingData.append("amount", amount);
    axios.post("/api/add_meal_serving", addMealServingData)
    .then(function (response) {
        // TODO: Show user if succeeded or not
        if (response.data.err) {
            errorStore.msgs.push(response.data.err);
            return;
        }

        selectedFood.value = null;
    })
    .catch (function (err) {
        errorStore.msgs.push("Failed to connect to the server (/api/add_meal_serving)");
    });
}

// search for food in case the URL parameter 'foodName' is set
if (route.params.foodName) {
    searchQuery.value = route.params.foodName;
    updateResults();
}
</script>

<template>
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
                        <ModalFoodViewer v-if="selectedFood" :food="selectedFood" @close="selectedFood = null" @add-food="addFoodToMeal"/>
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
