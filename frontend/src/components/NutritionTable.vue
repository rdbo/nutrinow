<script setup>
const props = defineProps(["nutrients"]);
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
                    <td class="border-r-2 border-gray-700">{{ nutrient.intake }}{{ nutrient.unit }}</td>
                    <td>{{ nutrient.desired }}{{ nutrient.unit }}</td>
                </tr>
            </tbody>
        </table>
    </div>
</div>
</template>
