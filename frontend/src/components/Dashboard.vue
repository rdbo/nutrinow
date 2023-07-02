<script setup>
import { ref } from "vue";
import DietDropdown from "./DietDropdown.vue";
import Meal from "./Meal.vue";
import NutritionTable from "./NutritionTable.vue";
import ModalDeleteDiet from "./ModalDeleteDiet.vue";
import ModalNewDiet from "./ModalNewDiet.vue";
import ModalEditDiet from "./ModalEditDiet.vue";
import ModalAddMeal from "./ModalAddMeal.vue";
import ModalDeleteMeal from "./ModalDeleteMeal.vue";
import ModalDuplicateDiet from "./ModalDuplicateDiet.vue";
import ModalDeleteMealFood from "./ModalDeleteMealFood.vue";
import ModalFoodViewer from "./ModalFoodViewer.vue";
import ModalSpinner from "./ModalSpinner.vue";
import { useErrorStore } from "@/stores/error";
import { useProfileStore } from "@/stores/profile";
import { api_get, api_post } from "../composables/api_request.js";

const errorStore = useErrorStore();
const profileStore = useProfileStore();

const curDietIndex = ref(0);
const diets = ref([]);
const showDeleteDiet = ref(false);
const showNewDiet = ref(false);
const showEditDiet = ref(false);
const showAddMeal = ref(false);
const showDuplicateDiet = ref(false);
const deleteMealFood = ref(null);
const deleteMealId = ref(null); // will prompt for meal deletion if not null
const editMealFoodRef = ref(null); // will prompt for edit meal serving if not null
const editFoodViewerRef = ref(null); // will show the food viewer
const meals = ref([]);
const userInfo = ref(null);
const nutrients = ref([]);

function updateCurDiet(index) {
    curDietIndex.value = index;
    localStorage.setItem("curDiet", diets.value[curDietIndex.value].id);

    // load diet meals
    let diet = diets.value[curDietIndex.value];

    api_get("meals/" + diet.id, null,
        (data) => { meals.value = data.meals; },
        () => { meals.value = []; }
    );

    api_get("diet_nutrition/" + diet.id, null,
        (data) => { diet.desired_nutrition = data.nutrition; },
        () => { diet.desired_nutrition = []; }
    );
}

function createNewDiet(name) {
    showNewDiet.value = false;
    let newDietData = new URLSearchParams();
    newDietData.append("diet_name", name);

    api_post("new_diet", newDietData,
        (_data) => { updateDiets(true); }
    );
}

function editCurDiet(name) {
    showEditDiet.value = false;
    let editDietData = new URLSearchParams();
    editDietData.append("diet_id", diets.value[curDietIndex.value].id);
    editDietData.append("diet_name", name);

    let diet = diets.value[curDietIndex.value];
    api_post("edit_diet", editDietData,
        (data) => {
            diet.name = name;
            updateCurDiet(curDietIndex.value); // refresh cookie
        }
    );
}

function deleteCurDiet() {
    showDeleteDiet.value = false;
    let deleteDietData = new URLSearchParams();
    deleteDietData.append("diet_id", diets.value[curDietIndex.value].id);

    api_post("delete_diet", deleteDietData,
        (_data) => {
            let oldIndex = curDietIndex.value;
            curDietIndex.value = 0;
            diets.value.splice(oldIndex, 1);
            if (diets.value.length > 0)
                updateCurDiet(curDietIndex.value);
        }
    );
}

function duplicateCurDiet(name) {
    showDuplicateDiet.value = false;
    let duplicateDietData = new URLSearchParams();
    duplicateDietData.append("diet_id", diets.value[curDietIndex.value].id);
    duplicateDietData.append("diet_name", name);

    api_post("duplicate_diet", duplicateDietData,
        (_data) => { updateDiets(true); }
    );
}

function addMeal(mealName) {
    showAddMeal.value = false;
    let addMealData = new URLSearchParams();
    addMealData.append("diet_id", diets.value[curDietIndex.value].id);
    addMealData.append("meal_name", mealName);

    api_post("add_meal", addMealData,
        (data) => { meals.value.push(data.meal); }
    );
}

function deleteMeal() {
    let mealId = deleteMealId.value;
    deleteMealId.value = null;

    let deleteMealData = new URLSearchParams();
    deleteMealData.append("meal_id", mealId);

    api_post("delete_meal", deleteMealData,
        (data) => {
            for (let i = 0; i < meals.value.length; ++i) {
                if (meals.value[i].id == mealId) {
                    meals.value.splice(i, 1);
                    break;
                }
            }
        }
    );
}

function updateDiets(useLast = false) {
    api_get("diets", null,
        (data) => {
            diets.value = data.diets;
            if (!diets.value.length) {
                curDietIndex.value = 0;
                return;
            }

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
        }
    );
}

function updateUserInfo() {
    api_get("user", null,
        (data) => {
            // TODO: Remove 'userInfo' and use only 'profileStore'
            userInfo.value = data;
            profileStore.name = data.name;
            profileStore.birthdate = data.birthdate;
            profileStore.weight = data.weight;
            profileStore.gender = data.gender;
        }
    );
}

function updateNutrients() {
    api_get("nutrients", null, 
        (data) => { nutrients.value = data.nutrients; }
    );
}

function getMealById(meal_id) {
    for (let i = 0; i < meals.value.length; ++i) {
        if (meals.value[i].id === meal_id)
            return meals.value[i];
    }
}

function findMealServingIndices(meal_serving_id) {
    for (let i = 0; i < meals.value.length; ++i) {
        for (let j = 0; j < meals.value[i].foods.length; ++j) {
            if (meals.value[i].foods[j].meal_serving_id == meal_serving_id)
                return [i, j];
        }
   }

   return null;
}

function deleteMealServing() {
    let food = deleteMealFood.value;
    let food_indices = findMealServingIndices(food.meal_serving_id);
    deleteMealFood.value = null;
    if (!food_indices) {
        errorStore.msgs.push("Failed to find serving in meals");
        return;
    }

    let deleteMealServingForm = new URLSearchParams();
    deleteMealServingForm.append("meal_serving_id", food.meal_serving_id);
    api_post("delete_meal_serving", deleteMealServingForm,
        (data) => { meals.value[food_indices[0]].foods.splice(food_indices[1], 1); } // TODO: Refactor
    );
}

function editFoodViewer() {
    let food = editMealFoodRef.value;
    api_get("food/" + food.id, null,
        (data) => { editFoodViewerRef.value = data.food; },
        () => { editMealFoodRef.value = null; }
    );
}

function editMealFood(servingId, amount) {
    let mealServingId = editMealFoodRef.value.meal_serving_id;
    let editMealServingForm = new URLSearchParams();
    editMealServingForm.append("meal_serving_id", mealServingId);
    editMealServingForm.append("serving_id", servingId);
    editMealServingForm.append("amount", amount);

    api_post("edit_meal_serving", editMealServingForm,
        (data) => {
            // TODO: Avoid reloading the whole diet
            updateCurDiet(curDietIndex.value);

            editMealFoodRef.value = null;
            editFoodViewerRef.value = null;
        },
        
        () => {
            editMealFoodRef.value = null;
            editFoodViewerRef.value = null;
        }
    );
}

function viewMealFood(food) {
    console.log(food);
}

updateUserInfo();
updateDiets();
updateNutrients();
// unset cached meal ID (for adding foods)
sessionStorage.removeItem("meal_id");
</script>

<template>
    <div class="mx-1 sm:mx-8 mt-2 mb-8 text-gray-800 max-w-4xl lg:mx-auto" v-if="userInfo">
        <h1 class="text-2xl max-md:text-center">Dashboard</h1>
        <div>
            <div class="my-4">
                <DietDropdown @update-cur-diet="updateCurDiet" @new-diet="showNewDiet = true" @edit-cur-diet="showEditDiet = true" @delete-cur-diet="showDeleteDiet = true" @duplicate-diet="showDuplicateDiet = true" :curDietIndex="curDietIndex" :diets="diets"/>
                <ModalNewDiet @cancel-new="showNewDiet = false" @new-diet="createNewDiet" v-if="showNewDiet"/>
                <ModalEditDiet @cancel-edit="showEditDiet = false" @edit-diet="editCurDiet" v-if="showEditDiet" :diet="diets[curDietIndex]"/>
                <ModalDeleteDiet @cancel-delete="showDeleteDiet = false" @delete-diet="deleteCurDiet" v-if="showDeleteDiet" :diet="diets[curDietIndex]"/>
                <ModalDuplicateDiet @cancel-duplicate="showDuplicateDiet = false" @duplicate-diet="duplicateCurDiet" v-if="showDuplicateDiet" :diet="diets[curDietIndex]"/>
            </div>
            <div v-if="diets.length">
                <Meal v-for="meal in meals" @delete-meal="(id) => deleteMealId = id" @delete-meal-food="(food) => deleteMealFood = food" @edit-meal-food="(food) => { editMealFoodRef = food; editFoodViewer() }" @view-meal-food="viewMealFood" :meal="meal" class="mt-8"/>
                <button id="btn_add_meal" @click="showAddMeal = true" class="text-xl bg-orange-300 px-8 py-4 border-2 border-gray-700 rounded-md my-4 w-full md:w-auto">Add Meal</button>
                <ModalAddMeal @cancel-add="showAddMeal = false" @add-meal="addMeal" v-if="showAddMeal"/>
                <ModalDeleteMeal @cancel-delete="deleteMealId = null" @delete-meal="deleteMeal" :meal="getMealById(deleteMealId)" v-if="deleteMealId"/>
                <ModalDeleteMealFood v-if="deleteMealFood" :food="deleteMealFood" @cancel-delete="deleteMealFood = null" @delete-meal-food="deleteMealServing"/>
                <ModalSpinner v-if="editMealFoodRef && !editFoodViewerRef"/>
                <ModalFoodViewer v-if="editFoodViewerRef" :food="editFoodViewerRef" @close="editMealFoodRef = null; editFoodViewerRef = null" @add-food="editMealFood"/>
            </div>
            <div v-else class="flex flex-col justify-center items-center text-2xl text-gray-500 text-center">
                <h1>No Diets Found</h1>
                <h2>Try creating a new diet using the Dashboard Menu</h2>
            </div>
            <NutritionTable v-if="diets.length" :meals="meals" :userInfo="userInfo" :nutrients="nutrients" :diet="diets[curDietIndex]"/>
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
