import { defineStore } from "pinia";
import { ref } from "vue";

export const useErrorStore = defineStore("error", () => {
    const msgs = ref([]);
    return { msgs };
});
