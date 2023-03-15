<script setup>
import TrashIcon from "./TrashIcon.vue";
import PencilSquareIcon from "./PencilSquareIcon.vue";
</script>

<template>
    <div class="mx-8 my-2 text-gray-800 max-w-4xl mx-8 md:mx-auto">
        <h1 class="text-2xl">Dashboard</h1>
        <div>
            <div><p>Diet 1</p></div>
            <div>
                <div class="border-gray-700 border-2 rounded-lg flex flex-col bg-secondary-100">
                    <h2 class="text-xl text-center border-b-2 border-gray-700 py-2">Breakfest</h2>
                    <div>
                        <div class="flex justify-between border-b-2 border-gray-700 bg-orange-200">
                            <div class="flex">
                                <!--
                                    TODO: Remove or implement up/down on database

                                <div class="flex flex-col bg-gray-200 border-r-2 border-gray-700">
                                    <button class="grow border-b-2 border-gray-700 px-1">Up</button>
                                    <button class="grow px-1">Down</button>
                                </div>
                                -->
                                <button class="bg-gray-200 px-4 py-2 border-r-2 border-gray-700">
                                    <PencilSquareIcon/>
                                </button>
                                <div class="flex items-center justify-center text-center">
                                    <p class="text-lg my-2 mx-2">Chicken Breast</p>
                                </div>
                            </div>
                            <div class="text-gray-500 flex">
                                <div class="mx-2 my-2 flex">
                                    <div class="mx-4 text-right">
                                        <p>Calories:</p>
                                        <p>Carbohydrates:</p>
                                        <p>Proteins:</p>
                                        <p>Fats:</p>
                                    </div>
                                    <div>
                                        <p>400 kcal</p>
                                        <p>5g</p>
                                        <p>32g</p>
                                        <p>15g</p>
                                    </div>
                                </div>
                                <button class="bg-red-300 text-red-500 px-4 py-2 flex items-center justify-center border-l-2 border-gray-700 py-4 px-4">
                                    <TrashIcon/>
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
