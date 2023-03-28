<script setup>
import { getNutrientsFromMealFood, getDisplayNutrients } from "../composables/foods.js";
import { TrashIcon } from "@heroicons/vue/20/solid";
import { PencilSquareIcon } from "@heroicons/vue/20/solid";
import { PlusIcon } from "@heroicons/vue/20/solid";

const props = defineProps(["food"]);
const nutrients = getNutrientsFromMealFood(props.food);
const displayNutrients = getDisplayNutrients(nutrients);
</script>

<template>
<div class="flex justify-between border-b-2 border-gray-700 bg-orange-200">
    <div class="flex">
        <!--
            TODO: Remove or implement up/down on database

        <div class="flex flex-col bg-gray-200 border-r-2 border-gray-700">
            <button class="grow border-b-2 border-gray-700 px-1">Up</button>
            <button class="grow px-1">Down</button>
        </div>
        -->
        <button class="bg-gray-200 px-4 py-2 border-r-2 border-gray-700 hover:bg-gray-400 hover:text-gray-900">
            <PencilSquareIcon class="w-6"/>
        </button>
        <div class="flex items-center justify-center text-center">
            <p class="text-lg my-2 mx-2">{{ food.name }} (<span class="font-bold">{{ food.serving_amount }}</span> <span class="italic">{{ food.serving_unit }}</span>)</p>
        </div>
    </div>
    <div class="text-gray-500 flex text-base">
        <div class="mx-2 my-2 flex">
            <div class="mx-4 text-right">
                <div v-for="nutrient in displayNutrients">
                    <p>{{ nutrient.name }}</p>
                </div>
            </div>
            <div>
                <div v-for="nutrient in displayNutrients">
                    <p>{{ nutrient.amount }}{{ nutrient.unit }}</p>
                </div>
            </div>
        </div>
        <button class="bg-red-300 text-red-500 px-4 py-2 flex items-center justify-center border-l-2 border-gray-700 py-4 px-4 hover:bg-red-400 hover:text-gray-700">
            <TrashIcon class="w-6"/>
        </button>
    </div>
</div>
</template>

<style scoped>
button {
    transition: all 0.2s ease-in-out;
}
</style>
