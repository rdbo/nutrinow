<script setup>
import { computed } from "vue";

const props = defineProps(["meals"]);
const nutrients = computed(() => {
    // TODO: Consider ALL desired nutrients, not only the ones given by the foods (maybe the food has not registered a certain nutrient)
    let nutrient_map = { };
    props.meals.forEach((meal) => {
        meal.foods.forEach((food) => {
            food.base_nutrients.forEach((nutrient) => {
                let nutrient_count = 0;
                if (nutrient_map.hasOwnProperty(nutrient.name)) {
                    nutrient_count = nutrient_map[nutrient.name].amount;
                }

                // proportionally calculate nutrient intake from base serving amount
                nutrient_map[nutrient.name] = {
                    amount: Number(Number(nutrient_count) + (nutrient.amount / food.serving_base) * food.serving_amount).toFixed(1),
                    unit: nutrient.unit
                };
            });
        });
    });

    let nutrient_arr = [];
    for (const nutrient in nutrient_map) {
        nutrient_arr.push({
            name: nutrient,
            amount: nutrient_map[nutrient].amount,
            unit: nutrient_map[nutrient].unit,
            desired: 0 /* TODO: Fix */
        });
    }

    console.log(nutrient_arr);

    return nutrient_arr;
});
</script>

<template>
<div class="border-2 border-gray-700 rounded-t-lg text-xl text-gray-800 bg-green-300">
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
                <tr v-for="nutrient in nutrients" class="border-gray-700" :class="{ 'border-b-2': nutrient != nutrients[nutrients.length - 1], 'bg-red-300': nutrient.intake < nutrient.desired }">
                    <td class="border-r-2 border-gray-700 font-bold">{{ nutrient.name }}</td>
                    <td class="border-r-2 border-gray-700">{{ nutrient.amount }}{{ nutrient.unit }}</td>
                    <td>{{ nutrient.desired }}{{ nutrient.unit }}</td>
                </tr>
            </tbody>
        </table>
    </div>
</div>
</template>
