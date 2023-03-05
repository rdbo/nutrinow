<script setup>
import FoodEditor from "../components/FoodEditor.vue";
</script>

<template>
    <h1>Welcome to NutriNow</h1>
    <h2>Make the diet of your dreams!</h2>
    <FoodEditor :foodInfo="{ name: 'Apple' }"/>
    <FoodEditor :foodInfo="null"/>
</template>

<style scoped>
</style>
