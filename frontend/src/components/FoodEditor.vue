<script setup>
import { ref } from "vue";
const props = defineProps({
    foodInfo: {}
});

const food = ref({});
</script>

<template>
    <div>
        <div v-if="foodInfo">
            <h1>Edit Food</h1>
            <h2>Name: {{ foodInfo.name }}</h2>
        </div>
        <div v-else>
            <h1>Add Food</h1>
            <h2>Name: {{ food.name }}</h2>
            <input v-model="food.name">
        </div>
    </div>
</template>

<style scoped>
</style>
