<script setup>
const props = defineProps(["name"]);
</script>

<template>
<div class="border-gray-700 border-2 rounded-lg flex flex-col bg-secondary-100">
    <h2 class="text-xl text-center border-b-2 border-gray-700 py-2">{{ name }}</h2>
    <div>
        <slot></slot>
    </div>
    <div class="flex justify-center items-center">
        <button id="btn_add_food" class="text-center bg-red-300 px-8 py-4 my-2 border-gray-700 border-2 rounded-md text-xl">Add Food</button>
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