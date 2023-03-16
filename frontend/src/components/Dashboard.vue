<script setup>
import DietDropdown from "./DietDropdown.vue";
import MealFood from "./MealFood.vue";
</script>

<template>
    <div class="mx-8 my-2 text-gray-800 max-w-4xl mx-8 lg:mx-auto">
        <h1 class="text-2xl">Dashboard</h1>
        <div>
            <div class="my-4">
                <DietDropdown/>
            </div>
            <div>
                <div class="border-gray-700 border-2 rounded-lg flex flex-col bg-secondary-100">
                    <h2 class="text-xl text-center border-b-2 border-gray-700 py-2">Breakfest</h2>
                    <div>
                        <MealFood food="Chicken Breast" amount="100g" cals="400" carbs="12" prots="32" fats="18"/>
                    </div>
                    <div class="flex justify-center items-center">
                        <button class="text-center bg-red-300 px-8 py-4 my-2 border-gray-700 border-2 rounded-md text-xl">Add Food</button>
                    </div>
                </div>
            </div>
        </div>
    </div>
</template>

<style scoped>
</style>
