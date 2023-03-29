<script setup>
import { getNutrientsFromMealFood, getDisplayNutrients } from "../composables/foods.js";
import { TrashIcon } from "@heroicons/vue/20/solid";
import { PencilSquareIcon } from "@heroicons/vue/20/solid";
import { PlusIcon } from "@heroicons/vue/20/solid";

const emit = defineEmits(["edit-meal-food", "delete-meal-food", "view-meal-food"]);
const props = defineProps(["food"]);
const nutrients = getNutrientsFromMealFood(props.food);
const displayNutrients = getDisplayNutrients(nutrients);
</script>

<template>
<div class="flex border-b-2 border-gray-700 bg-orange-200 cursor-pointer hover:bg-orange-300 select-none">
    <div class="flex">
        <button @click="$emit('edit-meal-food')" class="bg-gray-200 px-4 py-2 border-r-2 border-gray-700 hover:bg-gray-400 hover:text-gray-900">
            <PencilSquareIcon class="w-6"/>
        </button>
    </div>
    <div @click="$emit('view-meal-food')" class="flex justify-between grow">
        <div class="flex items-center justify-center text-center text-gray-700">
            <p class="text-lg my-2 mx-2">{{ food.name }} (<span class="font-bold">{{ food.serving_amount }}</span> <span class="italic">{{ food.serving_unit }}</span>)</p>
        </div>
        <div class="mx-2 my-2 flex text-gray-500 text-base">
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
    </div>
    <div class="flex">
        <button @click="$emit('delete-meal-food')" class="bg-red-300 text-red-500 px-4 py-2 flex items-center justify-center border-l-2 border-gray-700 py-4 px-4 hover:bg-red-400 hover:text-gray-700">
            <TrashIcon class="w-6"/>
        </button>
    </div>
</div>
</template>

<style scoped>
button, div {
    transition: all 0.2s ease-in-out;
}
</style>
