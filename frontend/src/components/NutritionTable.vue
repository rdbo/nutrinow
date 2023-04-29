<script setup>
import { computed } from "vue";

const props = defineProps(["meals", "diet", "userInfo", "nutrients"]);
const nutritionTable = computed(() => {
    // TODO: Consider ALL desired nutrients, not only the ones given by the foods (maybe the food has not registered a certain nutrient)
    let nutrient_map = { };
    props.nutrients.forEach((nutrient) => {
        nutrient_map[nutrient.name] = {
            amount: 0,
            min_desired: 0,
            max_desired: 0,
            unit: nutrient.unit
        };
    });

    props.diet.desired_nutrition.forEach((nutrient) => {
        let min_desired = nutrient.min_amount;
        let max_desired = nutrient.max_amount;
        if (nutrient.relative) {
            if (min_desired !== null)
                min_desired *= props.userInfo.weight;
            if (max_desired !== null)
                max_desired *= props.userInfo.weight;
        }
        nutrient_map[nutrient.name].min_desired = min_desired;
        nutrient_map[nutrient.name].max_desired = max_desired;
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
            min_desired: nutrient_map[nutrient].min_desired,
            max_desired: nutrient_map[nutrient].max_desired,
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
                    <th class="border-r-2 border-gray-700">Min</th>
                    <th class="border-r-2 border-gray-700">Max</th>
                    <th>Intake</th>
                </tr>
            </thead>
            <tbody class="bg-green-200">
                <tr v-for="nutrient in nutritionTable" class="border-gray-700 border-b-2" :class="{ 'bg-red-300': (nutrient.min_desired !== null && nutrient.amount < nutrient.min_desired) || (nutrient.max_desired !== null && nutrient.amount > nutrient.max_desired) }">
                    <td class="border-r-2 border-gray-700 font-bold">{{ nutrient.name }}</td>
                    <td class="border-r-2 border-gray-700">
                        <span v-if="nutrient.min_desired !== null">{{ nutrient.min_desired }}{{ nutrient.unit }}</span>
                        <span class="mx-2" v-else>-</span>
                    </td>
                    <td class="border-r-2 border-gray-700">
                        <span v-if="nutrient.max_desired !== null">{{ nutrient.max_desired }}{{ nutrient.unit }}</span>
                        <span class="mx-2" v-else>-</span>
                    </td>
                    <td>{{ nutrient.amount }}{{ nutrient.unit }}</td>
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
