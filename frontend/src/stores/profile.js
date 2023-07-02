import { defineStore } from "pinia";
import { ref } from "vue";

export const useProfileStore = defineStore("profile", () => {
    const name = ref("User");
    const birthdate = ref(null);
    const gender = ref(null);
    const weight = ref(null);
    return { name, birthdate, gender, weight };
});
