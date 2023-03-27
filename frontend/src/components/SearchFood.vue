<script setup>
import { getDisplayNutrients } from "../composables/foods.js";
import { PlusIcon } from "@heroicons/vue/20/solid";

const props = defineProps(["food"]);
const emit = defineEmits(["select-food"]);
const displayNutrients = getDisplayNutrients(props.food.servings[0].nutrients);
</script>

<template>
<div class="flex justify-between border-b-2 border-gray-700 bg-gray-200 cursor-pointer select-none" @click="$emit('select-food')">
    <div class="flex">
        <div class="flex items-center justify-center text-center">
            <p class="text-lg my-2 mx-2">{{ food.name }} ({{ food.servings[0].amount }}{{ food.servings[0].unit }})</p>
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
