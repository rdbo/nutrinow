<script setup>
import { PlusIcon } from "@heroicons/vue/20/solid";

const props = defineProps(["food"]);
const displayNutrients = ["Protein", "Carbohydrates", "Fats", "Calories"];
</script>

<template>
<div class="flex justify-between border-b-2 border-gray-700 bg-gray-200">
    <div class="flex">
        <!--
            TODO: Remove or implement up/down on database

        <div class="flex flex-col bg-gray-200 border-r-2 border-gray-700">
            <button class="grow border-b-2 border-gray-700 px-1">Up</button>
            <button class="grow px-1">Down</button>
        </div>
        -->
        <div class="flex items-center justify-center text-center">
            <p class="text-lg my-2 mx-2">{{ food.name }} ({{ food.servings[0].amount }}{{ food.servings[0].unit }})</p>
        </div>
    </div>
    <div class="text-gray-500 flex text-base">
        <div class="mx-2 my-2 flex">
            <div class="mx-4 text-right">
                <div v-for="nutrient in food.servings[0].nutrients">
                    <p v-if="displayNutrients.includes(nutrient.name)">{{ nutrient.name }}</p>
                </div>
            </div>
            <div>
                <div v-for="nutrient in food.servings[0].nutrients">
                    <!-- proportionally calculate nutrient amount from base serving values -->
                    <p v-if="displayNutrients.includes(nutrient.name)">{{ nutrient.amount }}{{ nutrient.unit }}</p>
                </div>
            </div>
        </div>
        <button class="bg-gray-500 text-gray-200 px-4 py-2 flex items-center justify-center border-l-2 border-gray-700 py-4 px-4 hover:bg-gray-400 hover:text-gray-700">
            <PlusIcon class="w-6"/>
        </button>
    </div>
</div>
</template>

<style scoped>
button {
    transition: all 0.2s ease-in-out;
}
</style>
