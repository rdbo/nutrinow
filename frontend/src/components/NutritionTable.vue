<script setup>
import { computed } from "vue";

const props = defineProps(["meals", "diet", "userInfo", "nutrients"]);
const nutritionTable = computed(() => {
    // TODO: Consider ALL desired nutrients, not only the ones given by the foods (maybe the food has not registered a certain nutrient)
    let nutrient_map = { };
    props.nutrients.forEach((nutrient) => {
        nutrient_map[nutrient.name] = {
            amount: 0,
            desired: 0,
            unit: nutrient.unit
        };
    });

    props.diet.desired_nutrition.forEach((nutrient) => {
        let desired_amount = nutrient.amount;
        if (nutrient.relative) {
            desired_amount *= props.userInfo.weight;
        }
        nutrient_map[nutrient.name].desired = desired_amount;
    });

    props.meals.forEach((meal) => {
        meal.foods.forEach((food) => {
            food.base_nutrients.forEach((nutrient) => {
                let nutrient_count = nutrient_map[nutrient.name].amount;
                // proportionally calculate nutrient intake from base serving amount
                nutrient_map[nutrient.name].amount = Number(Number(nutrient_count) + (nutrient.amount / food.serving_base) * food.serving_amount).toFixed(1);
            });
        });
    });

    let nutrient_arr = [];
    for (const nutrient in nutrient_map) {
        nutrient_arr.push({
            name: nutrient,
            amount: nutrient_map[nutrient].amount,
            unit: nutrient_map[nutrient].unit,
            desired: nutrient_map[nutrient].desired,
            relative: nutrient_map[nutrient].relative
        });
    }

    return nutrient_arr;
});
</script>

<template>
<div class="border-2 border-gray-700 rounded-lg text-xl text-gray-800 bg-green-300">
    <h1 class="text-center font-bold border-b-2 border-gray-700 py-2">Nutrition Table</h1>
    <div>
        <table class="w-full text-center">
            <thead class="border-b-2 border-gray-700">
                <tr>
                    <th class="border-r-2 border-gray-700">Name</th>
                    <th class="border-r-2 border-gray-700">Intake</th>
                    <th>Desired</th>
                </tr>
            </thead>
            <tbody class="bg-green-200">
                <tr v-for="nutrient in nutritionTable" class="border-gray-700 border-b-2" :class="{ 'bg-red-300': nutrient.amount < nutrient.desired }">
                    <td class="border-r-2 border-gray-700 font-bold">{{ nutrient.name }}</td>
                    <td class="border-r-2 border-gray-700">{{ nutrient.amount }}{{ nutrient.unit }}</td>
                    <td>{{ nutrient.desired }}{{ nutrient.unit }}</td>
                </tr>
            </tbody>
        </table>
        <div class="flex justify-center items-center">
            <button class="px-8 py-4 my-2 text-xl text-center border-2 border-gray-700 rounded-md bg-green-400">Edit Intake</button>
        </div>
    </div>
</div>
</template>

<style scoped>
button {
    transition: all 0.2s ease-in-out;
}

button:hover {
    @apply bg-green-500;
}
</style>
