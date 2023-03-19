<script setup>
import { ref } from "vue";
import axios from "axios";
import DietDropdown from "./DietDropdown.vue";
import Meal from "./Meal.vue";
import MealFood from "./MealFood.vue";
import NutritionTable from "./NutritionTable.vue";
import ModalDeleteDiet from "./ModalDeleteDiet.vue";
import ModalNewDiet from "./ModalNewDiet.vue";
import ModalEditDiet from "./ModalEditDiet.vue";

const curDietIndex = ref(0);
const diets = ref([]);
const showDeleteDiet = ref(false);
const showNewDiet = ref(false);
const showEditDiet = ref(false);
const meals = ref([]);

function updateCurDiet(index) {
    curDietIndex.value = index;
    localStorage.setItem("curDiet", diets.value[curDietIndex.value].id);

    // load diet meals
    axios.get("/api/meals/" + diets.value[curDietIndex.value].id)
    .then(function (response) {
        if (response.data.err) {
            // TODO: Handle error
            meals.value = [];
            return;
        }

        meals.value = response.data.meals;
        console.log(meals.value);
    })
    .catch(function (err) {
        // TODO: Handle error
        meals.value = [];
    });
}

function createNewDiet(name) {
    showNewDiet.value = false;
    let newDietData = new FormData();
    newDietData.append("diet_name", name);
    axios.post("/api/new_diet", newDietData)
    .then(function (response) {
        if (response.data.err) {
            // TODO: Handle error
            return;
        }
        updateDiets(true);
    })
    .catch(function (err) {
        // TODO: Handle error
    });
}

function editCurDiet(name) {
    showEditDiet.value = false;
    let editDietData = new FormData();
    editDietData.append("diet_id", diets.value[curDietIndex.value].id);
    editDietData.append("diet_name", name);
    axios.post("/api/edit_diet", editDietData)
    .then(function (response) {
        if (response.data.err) {
            // TODO: Handle error
            return;
        }

        diets.value[curDietIndex.value].name = name;
        updateCurDiet(curDietIndex.value); // refresh cookie
    })
    .catch(function (err) {
        // TODO: Handle error
    });
}

function deleteCurDiet() {
    showDeleteDiet.value = false;
    let deleteDietData = new FormData();
    deleteDietData.append("diet_id", diets.value[curDietIndex.value].id);
    axios.post("/api/delete_diet", deleteDietData)
    .then(function (response) {
        if (response.data.err) {
            // TODO: Handle error
            return;
        }

        let oldIndex = curDietIndex.value;
        curDietIndex.value = 0;
        diets.value.splice(oldIndex, 1);
    })
    .catch(function (err) {
        // TODO: Handle error
    });
}

function updateDiets(useLast = false) {
    axios.get("/api/diets")
    .then(function (response) {
        if (response.data.err) {
            // TODO: Handle error
            return;
        }

        diets.value = response.data.diets;

        let curDietCookie = localStorage.getItem("curDiet");
        let newCurDietIndex = 0;
        if (useLast) {
            newCurDietIndex = diets.value.length - 1;
        } else if (curDietCookie) {
            for (let i = 0; i < diets.value.length; ++i) {
                if (curDietCookie == diets.value[i].id)
                    newCurDietIndex = i;
            }
        }

        updateCurDiet(newCurDietIndex);
    })
    .catch(function (err) {
        // TODO: Handle error
    });
}

updateDiets();

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
                <DietDropdown @update-cur-diet="updateCurDiet" @new-diet="showNewDiet = true" @edit-cur-diet="showEditDiet = true" @delete-cur-diet="showDeleteDiet = true" :curDietIndex="curDietIndex" :diets="diets"/>
                <ModalNewDiet @cancel-new="showNewDiet = false" @new-diet="createNewDiet" v-if="showNewDiet"/>
                <ModalEditDiet @cancel-edit="showEditDiet = false" @edit-diet="editCurDiet" v-if="showEditDiet" :diet="diets[curDietIndex]"/>
                <ModalDeleteDiet @cancel-delete="showDeleteDiet = false" @delete-diet="deleteCurDiet" v-if="showDeleteDiet" :diet="diets[curDietIndex]"/>
            </div>
            <div>
                <Meal v-for="meal in meals" :name="meal.name" :foods="meal.foods" class="mt-8"/>
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
