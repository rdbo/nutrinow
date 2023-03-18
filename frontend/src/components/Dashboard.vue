<script setup>
import { ref } from "vue";
import axios from "axios";
import DietDropdown from "./DietDropdown.vue";
import Meal from "./Meal.vue";
import MealFood from "./MealFood.vue";
import NutritionTable from "./NutritionTable.vue";
import ModalDeleteDiet from "./ModalDeleteDiet.vue";

const props = defineProps(["session_id"]);

const curDiet = ref({ id: null, name: "" });
const diets = ref([]);
const showDeleteDiet = ref(false);

function updateCurDiet(diet) {
    curDiet.value = diet;
    $cookies.set("curDiet", curDiet.value);
}

function deleteCurDiet() {
    console.log("delete cur diet");
    showDeleteDiet.value = false;
}

axios.get("/api/diets/" + props.session_id)
.then(function (response) {
    if (response.data.err) {
        // TODO: Handle error
        return;
    }

    diets.value = response.data.diets;

    let curDietCookie = $cookies.get("curDiet");
    if (curDietCookie && diets.value.some((el) => { return JSON.stringify(curDietCookie) === JSON.stringify(el); })) {
        curDiet.value = curDietCookie;
    } else {
        curDiet.value = diets.value[0];
        $cookies.set("curDiet", curDiet.value);
    }
});

const breakfest = [
    {
        "name": "Milk",
        "amount": "200ml",
        "cals": "250",
        "carbs": "12",
        "prots": "32",
        "fats": "18"
    },
    {
        "name": "Coffee",
        "amount": "30g",
        "cals": "100",
        "carbs": "5",
        "prots": "5",
        "fats": "0"
    },
    {
        "name": "Bread",
        "amount": "30g",
        "cals": "200",
        "carbs": "20",
        "prots": "2",
        "fats": "1"
    }
];

const lunch = [
    {
        "name": "Chicken Breast",
        "amount": "100g",
        "cals": "400",
        "carbs": "12",
        "prots": "32",
        "fats": "18"
    },
    {
        "name": "White Rice",
        "amount": "100g",
        "cals": "500",
        "carbs": "50",
        "prots": "12",
        "fats": "8"
    },
    {
        "name": "Beans",
        "amount": "100g",
        "cals": "200",
        "carbs": "20",
        "prots": "10",
        "fats": "8"
    }
];

const nutrients = [
    {
        "name": "Vitamin A",
        "intake": "100",
        "desired": "120",
        "unit": "μg"
    },
    {
        "name": "Vitamin D",
        "intake": "120",
        "desired": "120",
        "unit": "μg"
    }
]
</script>

<template>
    <div class="mx-8 mt-2 mb-8 text-gray-800 max-w-4xl lg:mx-auto">
        <h1 class="text-2xl">Dashboard</h1>
        <div>
            <div class="my-4">
                <DietDropdown @update-cur-diet="updateCurDiet" @delete-cur-diet="showDeleteDiet = true" :curDiet="curDiet" :diets="diets"/>
                <ModalDeleteDiet @cancel-delete="showDeleteDiet = false" @delete-diet="deleteCurDiet" v-if="showDeleteDiet" :diet="curDiet"/>
            </div>
            <div>
                <Meal name="Breakfest" :foods="breakfest" class="my-8"/>
                <Meal name="Lunch" :foods="lunch"/>
                <button id="btn_add_meal" class="text-xl bg-orange-300 px-8 py-4 border-2 border-gray-700 rounded-md my-4 w-full md:w-auto">Add Meal</button>
            </div>
            <NutritionTable :nutrients="nutrients"/>
        </div>
    </div>
</template>

<style scoped>
#btn_add_meal {
    transition: all 0.2s ease-in-out;
}

#btn_add_meal:hover {
    @apply bg-orange-400 text-gray-900;
}
</style>
