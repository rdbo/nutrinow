<script setup>
import DietDropdown from "./DietDropdown.vue";
import Meal from "./Meal.vue";
import MealFood from "./MealFood.vue";
</script>

<template>
    <div class="mx-8 mt-2 mb-8 text-gray-800 max-w-4xl lg:mx-auto">
        <h1 class="text-2xl">Dashboard</h1>
        <div>
            <div class="my-4">
                <DietDropdown/>
            </div>
            <div>
                <Meal name="Breakfest" class="my-8">
                    <MealFood name="Milk" amount="200ml" cals="250" carbs="12" prots="32" fats="18"/>
                    <MealFood name="Coffee" amount="30g" cals="100" carbs="5" prots="5" fats="0"/>
                    <MealFood name="Bread" amount="30g" cals="200" carbs="20" prots="2" fats="1"/>
                </Meal>
                <Meal name="Lunch">
                    <MealFood name="Chicken Breast" amount="100g" cals="400" carbs="12" prots="32" fats="18"/>
                    <MealFood name="White Rice" amount="100g" cals="500" carbs="50" prots="12" fats="10"/>
                    <MealFood name="Beans" amount="100g" cals="200" carbs="20" prots="10" fats="8"/>
                </Meal>
            </div>
        </div>
    </div>
</template>

<style scoped>
</style>
