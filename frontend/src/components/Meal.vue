<script setup>
import { computed } from "vue";
import { useRouter } from "vue-router";
import { getNutrientsFromMealFood, getDisplayNutrients } from "../composables/foods.js";
import MealFood from "./MealFood.vue";
import { XCircleIcon, InformationCircleIcon, TrashIcon } from "@heroicons/vue/20/solid";

const router = useRouter();
const emit = defineEmits(["delete-meal", "delete-meal-food", "edit-meal-food", "view-meal-food"]);
const props = defineProps(["meal"]);

function addFood() {
    sessionStorage.setItem("meal_id", props.meal.id);
    router.push({ name: "foods" });
}

const totalDisplayNutrients = computed(() => {
    let sumDisplayNutrients = { };
    let foods = props.meal.foods;
    for (let i = 0; i < foods.length; ++i) {
        let nutrients = getNutrientsFromMealFood(foods[i]);
        let displayNutrients = getDisplayNutrients(nutrients);
        for (let j = 0; j < displayNutrients.length; ++j) {
            let nutrient = displayNutrients[j];

            if (!sumDisplayNutrients.hasOwnProperty(nutrient.name)) {
                sumDisplayNutrients[nutrient.name] = { amount: nutrient.amount, unit: nutrient.unit };
            } else {
                sumDisplayNutrients[nutrient.name].amount = Number(sumDisplayNutrients[nutrient.name].amount) + Number(nutrient.amount);
            }
        }
    }

    let listDisplayNutrients = [];
    for (const key in sumDisplayNutrients) {
        let nutrient = sumDisplayNutrients[key];
        listDisplayNutrients.push({
            name: key,
            amount: Number(nutrient.amount).toFixed(1),
            unit: nutrient.unit
        });
    }

    return listDisplayNutrients;
});
</script>

<template>
<div class="border-gray-700 border-2 rounded-lg flex flex-col bg-secondary-100">
    <div class="relative">
        <h2 class="text-xl text-center border-b-2 border-gray-700 py-2 font-bold">{{ meal.name }}</h2>
        <button @click="$emit('delete-meal', meal.id)" class="absolute right-0 top-0 py-2 px-4 text-xl">
            <XCircleIcon class="w-6"/>
        </button>
    </div>
    <div>
        <MealFood v-for="food in meal.foods" :food="food" @delete-meal-food="$emit('delete-meal-food', food)" @edit-meal-food="$emit('edit-meal-food', food)" @view-meal-food="$emit('view-meal-food', food)"/>
        <div class="flex border-b-2 border-gray-700 bg-orange-200 select-none">
            <div class="flex">
                <button class="bg-gray-200 px-4 py-2 border-r-2 border-gray-700 cursor-default">
                    <InformationCircleIcon class="w-6"/>
                </button>
            </div>
            <div class="flex flex-col sm:flex-row sm:justify-between grow">
                <div class="flex items-center justify-center text-center text-gray-700">
                    <p class="text-lg my-2 mx-2 font-bold">Total</p>
                </div>
                <div class="mx-2 my-2 flex text-gray-500 text-base justify-center">
                    <div class="mx-4 text-right">
                        <div v-for="nutrient in totalDisplayNutrients">
                            <p>{{ nutrient.name }}</p>
                        </div>
                    </div>
                    <div>
                        <div v-for="nutrient in totalDisplayNutrients">
                            <p>{{ nutrient.amount }}{{ nutrient.unit }}</p>
                        </div>
                    </div>
                </div>
            </div>
            <div class="flex">
                <button class="bg-gray-200 px-4 py-2 border-l-2 border-gray-700 cursor-default">
                    <InformationCircleIcon class="w-6"/>
                </button>
            </div>
        </div>

    </div>
    <div class="flex justify-center items-center">
        <button @click="addFood" id="btn_add_food" class="text-center bg-red-300 px-8 py-4 my-2 border-gray-700 border-2 rounded-md text-xl">Add Food</button>
    </div>
</div>
</template>

<style scoped>
#btn_add_food {
    transition: all 0.2s ease-in-out;
}

#btn_add_food:hover {
    @apply bg-red-400 text-gray-900;
}
</style>
