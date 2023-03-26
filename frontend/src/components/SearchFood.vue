<script setup>
import { PlusIcon } from "@heroicons/vue/20/solid";
import { getDisplayNutrients } from "../composables/foods.js";

const props = defineProps(["food"]);
const displayNutrients = getDisplayNutrients(props.food.servings[0].nutrients);
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
    <div class="text-gray-500 flex text-base cursor-pointer select-none">
        <div class="mx-2 my-2 flex">
            <div class="mx-4 text-right">
                <div v-for="nutrient in displayNutrients">
                    <p>{{ nutrient.name }}</p>
                </div>
            </div>
            <div>
                <div v-for="nutrient in displayNutrients">
                    <!-- proportionally calculate nutrient amount from base serving values -->
                    <p>{{ nutrient.amount }}{{ nutrient.unit }}</p>
                </div>
            </div>
        </div>
    </div>
</div>
</template>

<style scoped>
button {
    transition: all 0.2s ease-in-out;
}
</style>
