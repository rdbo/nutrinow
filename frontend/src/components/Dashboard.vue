<script setup>
</script>

<template>
    <div class="mx-8 my-2">
        <h1 class="text-2xl text-gray-700">Dashboard</h1>
        <div>
            <div><p>Diet 1</p></div>
            <div>
                <div class="border-gray-700 border-2 rounded-lg flex flex-col bg-secondary-100">
                    <h2 class="text-xl text-center border-b-2 border-gray-700 py-2">Breakfest</h2>
                    <div class="grow">
                        <div class="flex items-center justify-between border-b-2 border-gray-700 bg-orange-200">
                            <p class="text-lg my-2 mx-2">Chicken Breast</p>
                            <div class="text-gray-500 flex">
                                <div class="mx-2 my-2">
                                    <p>Calories: 400 kcal</p>
                                    <p>Carbohydrates: 5g</p>
                                    <p>Protein: 32g</p>
                                    <p>Fat: 15g</p>
                                </div>
                                <button class="bg-red-300 text-red-500 flex items-center justify-center border-l-2 border-gray-700 py-4 px-4">
                                    <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-6 h-6">
                                        <path stroke-linecap="round" stroke-linejoin="round" d="M14.74 9l-.346 9m-4.788 0L9.26 9m9.968-3.21c.342.052.682.107 1.022.166m-1.022-.165L18.16 19.673a2.25 2.25 0 01-2.244 2.077H8.084a2.25 2.25 0 01-2.244-2.077L4.772 5.79m14.456 0a48.108 48.108 0 00-3.478-.397m-12 .562c.34-.059.68-.114 1.022-.165m0 0a48.11 48.11 0 013.478-.397m7.5 0v-.916c0-1.18-.91-2.164-2.09-2.201a51.964 51.964 0 00-3.32 0c-1.18.037-2.09 1.022-2.09 2.201v.916m7.5 0a48.667 48.667 0 00-7.5 0" />
                                    </svg>
                                </button>
                            </div>
                        </div>
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
