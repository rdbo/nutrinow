<script setup>
import { ref, defineProps } from "vue";
const props = defineProps({
    foodInfo: {}
});
</script>

<template>
    <div>
        <div v-if="foodInfo">
            <h1>Edit Food</h1>
            <h2>Name: {{ foodInfo.name }}</h2>
        </div>
        <div v-else>
            <h1>Add Food</h1>
        </div>
    </div>
</template>

<style scoped>
</style>
