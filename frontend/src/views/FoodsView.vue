<script setup>
import { ref } from "vue";
import { MagnifyingGlassIcon } from '@heroicons/vue/20/solid';
import MealFood from "../components/MealFood.vue";

const searchResults = ref([]);
const food = {
  "id": 1,
  "name": "Hummus, commercial",
  "serving_id": 1,
  "serving_base": 100,
  "serving_amount": 100,
  "serving_unit": "g",
  "base_nutrients": [
    {
      "name": "Vitamin C",
      "amount": 0,
      "unit": "mg"
    },
    {
      "name": "Vitamin B1",
      "amount": 0.15,
      "unit": "μg"
    },
    {
      "name": "Vitamin B2",
      "amount": 0.115,
      "unit": "μg"
    },
    {
      "name": "Vitamin B9",
      "amount": 36,
      "unit": "μg"
    },
    {
      "name": "Vitamin K",
      "amount": 17.2,
      "unit": "μg"
    },
    {
      "name": "Saturated Fat",
      "amount": 2.22,
      "unit": "g"
    },
    {
      "name": "Protein",
      "amount": 7.35,
      "unit": "g"
    },
    {
      "name": "Fiber",
      "amount": 5.4,
      "unit": "g"
    },
    {
      "name": "Iron",
      "amount": 2.41,
      "unit": "mg"
    },
    {
      "name": "Magnesium",
      "amount": 71.1,
      "unit": "mg"
    },
    {
      "name": "Phosphorus",
      "amount": 166,
      "unit": "mg"
    },
    {
      "name": "Sodium",
      "amount": 438,
      "unit": "mg"
    },
    {
      "name": "Copper",
      "amount": 0.348,
      "unit": "mg"
    },
    {
      "name": "Manganese",
      "amount": 1.06,
      "unit": "mg"
    },
    {
      "name": "Vitamin A",
      "amount": 1,
      "unit": "μg"
    },
    {
      "name": "Unsaturated Fat",
      "amount": 13.850000000000001,
      "unit": "g"
    },
    {
      "name": "Vitamin B3",
      "amount": 0.948,
      "unit": "μg"
    },
    {
      "name": "Vitamin B5",
      "amount": 0.318,
      "unit": "μg"
    },
    {
      "name": "Vitamin B6",
      "amount": 0.143,
      "unit": "mg"
    },
    {
      "name": "Fats",
      "amount": 17.1,
      "unit": "g"
    },
    {
      "name": "Carbohydrates",
      "amount": 14.9,
      "unit": "g"
    },
    {
      "name": "Water",
      "amount": 58.7,
      "unit": "ml"
    },
    {
      "name": "Calcium",
      "amount": 41,
      "unit": "mg"
    },
    {
      "name": "Potassium",
      "amount": 289,
      "unit": "mg"
    },
    {
      "name": "Zinc",
      "amount": 1.38,
      "unit": "mg"
    },
    {
      "name": "Selenium",
      "amount": 16.2,
      "unit": "mg"
    },
    {
      "name": "Vitamin E",
      "amount": 1.74,
      "unit": "mg"
    },
    {
      "name": "Sugars",
      "amount": 0.34,
      "unit": "g"
    }
  ]
};
</script>

<template>
    <div class="text-2xl text-gray-700 flex flex-col items-center px-2 mb-16" :class="{ 'pt-48': searchResults.length <= 0 }">
        <div class="w-full max-w-2xl">
            <h1 class="text-6xl my-2 text-center">Foods</h1>
            <div class="w-full flex justify-center">
                <div class="text-2xl flex rounded-md overflow-hidden border-gray-400 border-2 grow">
                    <input class="bg-red-50 px-2 w-full" placeholder="Search for a food"/>
                    <button class="bg-orange-300 px-4 py-2 border-gray-400 border-l-2" @click="searchResults = [123]"><MagnifyingGlassIcon class="text-yellow-700 w-9"/></button>
                </div>
            </div>
            <div v-if="searchResults.length > 0" class="my-2">
                <h2 class="text-gray-500 text-center my-2">Search Results</h2>
                <div class="border-gray-700 border-2 rounded-md overflow-hidden">
                    <MealFood :search="true" :food="food"/>
                    <MealFood :search="true" :food="food"/>
                    <MealFood :search="true" :food="food"/>
                    <MealFood :search="true" :food="food"/>
                    <MealFood :search="true" :food="food"/>
                    <MealFood :search="true" :food="food"/>
                    <MealFood :search="true" :food="food"/>
                    <MealFood :search="true" :food="food"/>
                    <MealFood :search="true" :food="food"/>
                    <MealFood :search="true" :food="food"/>
                </div>
            </div>
        </div>
    </div>
</template>

<style scoped>
</style>
