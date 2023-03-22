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
import ModalAddMeal from "./ModalAddMeal.vue";
import ModalDeleteMeal from "./ModalDeleteMeal.vue";

const curDietIndex = ref(0);
const diets = ref([]);
const showDeleteDiet = ref(false);
const showNewDiet = ref(false);
const showEditDiet = ref(false);
const showAddMeal = ref(false);
const deleteMealId = ref(null); // will prompt for meal deletion if not null
const meals = ref([]);
const userInfo = ref(null);
const nutrients = ref([]);

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
        updateCurDiet(curDietIndex.value);
    })
    .catch(function (err) {
        // TODO: Handle error
    });
}

function addMeal(mealName) {
    showAddMeal.value = false;
    let addMealData = new FormData();
    addMealData.append("diet_id", diets.value[curDietIndex.value].id);
    addMealData.append("meal_name", mealName);
    axios.post("/api/add_meal", addMealData)
    .then(function (response) {
        if (response.data.err) {
            // TODO: Handle error
            return;
        }

        meals.value.push(response.data.meal);
    })
    .catch(function (err) {
        // TODO: Handle error
    });
}

function deleteMeal() {
    let mealId = deleteMealId.value;
    deleteMealId.value = null;

    let deleteMealData = new FormData();
    deleteMealData.append("meal_id", mealId);
    axios.post("/api/delete_meal", deleteMealData)
    .then(function (response) {
        if (response.data.err) {
            // TODO: Handle error
            return;
        }

        for (let i = 0; i < meals.value.length; ++i) {
            if (meals.value[i].id == mealId) {
                meals.value.splice(i, 1);
                break;
            }
        }
    })
    .catch(function (err) {
        // TODO: Handle error
    })
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

function updateUserInfo() {
    axios.get("/api/user")
    .then(function (response) {
        if (response.data.err) {
            // TODO: Handle error
            return;
        }

        userInfo.value = response.data;
    })
    .catch(function (err) {
        // TODO: Handle error
    });
}

function updateNutrients() {
    axios.get("/api/nutrients")
    .then(function (response) {
        if (response.data.err) {
            // TODO: Handle error
            return;
        }

        nutrients.value = response.data.nutrients;
    })
    .catch(function (err) {
        // TODO: Handle error
    });
}

function getMealById(meal_id) {
    for (let i = 0; i < meals.value.length; ++i) {
        if (meals.value[i].id === meal_id)
            return meals.value[i];
    }
}

updateDiets();
updateUserInfo();
updateNutrients();
</script>

<template>
    <div class="mx-8 mt-2 mb-8 text-gray-800 max-w-4xl lg:mx-auto" v-if="userInfo">
        <h1 class="text-2xl">Dashboard - {{ userInfo.name }}</h1>
        <div>
            <div class="my-4">
                <DietDropdown @update-cur-diet="updateCurDiet" @new-diet="showNewDiet = true" @edit-cur-diet="showEditDiet = true" @delete-cur-diet="showDeleteDiet = true" :curDietIndex="curDietIndex" :diets="diets"/>
                <ModalNewDiet @cancel-new="showNewDiet = false" @new-diet="createNewDiet" v-if="showNewDiet"/>
                <ModalEditDiet @cancel-edit="showEditDiet = false" @edit-diet="editCurDiet" v-if="showEditDiet" :diet="diets[curDietIndex]"/>
                <ModalDeleteDiet @cancel-delete="showDeleteDiet = false" @delete-diet="deleteCurDiet" v-if="showDeleteDiet" :diet="diets[curDietIndex]"/>
            </div>
            <div>
                <Meal @delete-meal="(id) => deleteMealId = id" v-for="meal in meals" :meal="meal" class="mt-8"/>
                <button id="btn_add_meal" @click="showAddMeal = true" class="text-xl bg-orange-300 px-8 py-4 border-2 border-gray-700 rounded-md my-4 w-full md:w-auto">Add Meal</button>
                <ModalAddMeal @cancel-add="showAddMeal = false" @add-meal="addMeal" v-if="showAddMeal"/>
                <ModalDeleteMeal @cancel-delete="deleteMealId = null" @delete-meal="deleteMeal" :meal="getMealById(deleteMealId)" v-if="deleteMealId"/>
            </div>
            <NutritionTable :meals="meals" :userInfo="userInfo" :nutrients="nutrients" :diet="diets[curDietIndex]"/>
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
