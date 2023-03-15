<script setup>
import { defineProps } from "vue";
import Welcome from "../components/Welcome.vue";
import FoodEditor from "../components/FoodEditor.vue";
import Dashboard from "../components/Dashboard.vue";

const props = defineProps(["session_id"]);
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
