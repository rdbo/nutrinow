import { defineStore } from "pinia";
import { ref } from "vue";

export const useErrorStore = defineStore("error", () => {
    const msg = ref("");
    return { msg };
});
