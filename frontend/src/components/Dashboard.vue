<script setup>
import { ref } from "vue";
import axios from "axios";
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
import { useErrorStore } from "@/stores/error";

const errorStore = useErrorStore();

const curDietIndex = ref(0);
const diets = ref([]);
const showDeleteDiet = ref(false);
const showNewDiet = ref(false);
const showEditDiet = ref(false);
const showAddMeal = ref(false);
const showDuplicateDiet = ref(false);
const deleteMealFood = ref(null);
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
            errorStore.msgs.push(response.data.err);
            meals.value = [];
            return;
        }

        meals.value = response.data.meals;
    })
    .catch(function (err) {
        errorStore.msgs.push("Failed to connect to the server (/api/meals/<diet_id>)");
        meals.value = [];
    });

    axios.get("/api/diet_nutrition/" + diets.value[curDietIndex.value].id)
    .then(function (response) {
        if (response.data.err) {
            errorStore.msgs.push(response.data.err);
            diets.value[curDietIndex.value].desired_nutrition = [];
            return;
        }

        diets.value[curDietIndex.value].desired_nutrition = response.data.nutrition;
    })
    .catch(function (err) {
        errorStore.msgs.push("Failed to connect to the server (/api/diet_nutrition/<diet_id>)");
        diets.value[curDietIndex.value].desired_nutrition = [];
    });
}

function createNewDiet(name) {
    showNewDiet.value = false;
    let newDietData = new FormData();
    newDietData.append("diet_name", name);
    axios.post("/api/new_diet", newDietData)
    .then(function (response) {
        if (response.data.err) {
            errorStore.msgs.push(response.data.err);
            return;
        }
        updateDiets(true);
    })
    .catch(function (err) {
        errorStore.msgs.push("Failed to connect to the server (/api/new_diet)");
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
            errorStore.msgs.push(response.data.err);
            return;
        }

        diets.value[curDietIndex.value].name = name;
        updateCurDiet(curDietIndex.value); // refresh cookie
    })
    .catch(function (err) {
        errorStore.msgs.push("Failed to connect to the server (/api/edit_diet)");
    });
}

function deleteCurDiet() {
    showDeleteDiet.value = false;
    let deleteDietData = new FormData();
    deleteDietData.append("diet_id", diets.value[curDietIndex.value].id);
    axios.post("/api/delete_diet", deleteDietData)
    .then(function (response) {
        if (response.data.err) {
            errorStore.msgs.push(response.data.err);
            return;
        }

        let oldIndex = curDietIndex.value;
        curDietIndex.value = 0;
        console.log("test");
        diets.value.splice(oldIndex, 1);
        console.log("test2");
        if (diets.value.length > 0)
            updateCurDiet(curDietIndex.value);
    })
    .catch(function (err) {
        errorStore.msgs.push("Failed to connect to the server (/api/delete_diet)");
    });
}

function duplicateCurDiet(name) {
    showDuplicateDiet.value = false;
    let duplicateDietData = new FormData();
    duplicateDietData.append("diet_id", diets.value[curDietIndex.value].id);
    duplicateDietData.append("diet_name", name);
    axios.post("/api/duplicate_diet", duplicateDietData)
    .then(function (response) {
        if (response.data.err) {
            errorStore.msgs.push(response.data.err);
            return;
        }
        updateDiets(true);
    })
    .catch(function (err) {
        errorStore.msgs.push("Failed to connect to the server (/api/edit_diet)");
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
            errorStore.msgs.push(response.data.err);
            return;
        }

        meals.value.push(response.data.meal);
    })
    .catch(function (err) {
        errorStore.msgs.push("Failed to connect to the server (/api/add_meal)");
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
            errorStore.msgs.push(response.data.err);
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
        errorStore.msgs.push("Failed to connect to the server (/api/delete_meal)");
    })
}

function updateDiets(useLast = false) {
    axios.get("/api/diets")
    .then(function (response) {
        if (response.data.err) {
            errorStore.msgs.push(response.data.err);
            return;
        }

        diets.value = response.data.diets;
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
    })
    .catch(function (err) {
        errorStore.msgs.push("Failed to connect to the server (/api/diets)");
    });
}

function updateUserInfo() {
    axios.get("/api/user")
    .then(function (response) {
        if (response.data.err) {
            errorStore.msgs.push(response.data.err);
            return;
        }

        userInfo.value = response.data;
    })
    .catch(function (err) {
        errorStore.msgs.push("Failed to connect to the server (/api/user)");
    });
}

function updateNutrients() {
    axios.get("/api/nutrients")
    .then(function (response) {
        if (response.data.err) {
            errorStore.msgs.push(response.data.err);
            return;
        }

        nutrients.value = response.data.nutrients;
    })
    .catch(function (err) {
        errorStore.msgs.push("Failed to connect to the server (/api/nutrients)");
    });
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

    let deleteMealServingForm = new FormData();
    deleteMealServingForm.append("meal_serving_id", food.meal_serving_id);
    axios.post("/api/delete_meal_serving", deleteMealServingForm)
    .then(function (response) {
        if (response.data.err) {
            errorStore.msgs.push(response.data.err);
            return;
        }

        meals.value[food_indices[0]].foods.splice(food_indices[1], 1);
    })
    .catch(function (err) {
        errorStore.msgs.push("Failed to connect to the server (/api/delete_meal_serving)");
    });
}

function editMealFood(food) {
    console.log(food);
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
        <h1 class="text-2xl max-md:text-center">Dashboard - {{ userInfo.name }}</h1>
        <div>
            <div class="my-4">
                <DietDropdown @update-cur-diet="updateCurDiet" @new-diet="showNewDiet = true" @edit-cur-diet="showEditDiet = true" @delete-cur-diet="showDeleteDiet = true" @duplicate-diet="showDuplicateDiet = true" :curDietIndex="curDietIndex" :diets="diets"/>
                <ModalNewDiet @cancel-new="showNewDiet = false" @new-diet="createNewDiet" v-if="showNewDiet"/>
                <ModalEditDiet @cancel-edit="showEditDiet = false" @edit-diet="editCurDiet" v-if="showEditDiet" :diet="diets[curDietIndex]"/>
                <ModalDeleteDiet @cancel-delete="showDeleteDiet = false" @delete-diet="deleteCurDiet" v-if="showDeleteDiet" :diet="diets[curDietIndex]"/>
                <ModalDuplicateDiet @cancel-duplicate="showDuplicateDiet = false" @duplicate-diet="duplicateCurDiet" v-if="showDuplicateDiet" :diet="diets[curDietIndex]"/>
            </div>
            <div v-if="diets.length">
                <Meal v-for="meal in meals" @delete-meal="(id) => deleteMealId = id" @delete-meal-food="(food) => deleteMealFood = food" @edit-meal-food="editMealFood" @view-meal-food="viewMealFood" :meal="meal" class="mt-8"/>
                <button id="btn_add_meal" @click="showAddMeal = true" class="text-xl bg-orange-300 px-8 py-4 border-2 border-gray-700 rounded-md my-4 w-full md:w-auto">Add Meal</button>
                <ModalAddMeal @cancel-add="showAddMeal = false" @add-meal="addMeal" v-if="showAddMeal"/>
                <ModalDeleteMeal @cancel-delete="deleteMealId = null" @delete-meal="deleteMeal" :meal="getMealById(deleteMealId)" v-if="deleteMealId"/>
                <ModalDeleteMealFood v-if="deleteMealFood" :food="deleteMealFood" @cancel-delete="deleteMealFood = null" @delete-meal-food="deleteMealServing"/>
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
