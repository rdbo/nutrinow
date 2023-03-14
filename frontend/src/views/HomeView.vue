<script setup>
import Welcome from "../components/Welcome.vue";
import FoodEditor from "../components/FoodEditor.vue";
import Dashboard from "../components/Dashboard.vue";

const session_id = null;
</script>

<template>
    <div v-if="session_id">
        <Dashboard/>
    </div>
    <div v-else>
        <Welcome/>
    </div>
</template>

<style scoped>
</style>
