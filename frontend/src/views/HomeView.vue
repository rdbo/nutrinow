<script setup>
import Welcome from "../components/Welcome.vue";
import Dashboard from "../components/Dashboard.vue";
import { useSessionStore } from "@/stores/session";

const sessionStore = useSessionStore();
</script>

<template>
    <div v-if="sessionStore.id">
        <Dashboard/>
    </div>
    <div v-else>
        <Welcome/>
    </div>
</template>

<style scoped>
</style>
