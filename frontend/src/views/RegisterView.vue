<script setup>
import { useRouter } from "vue-router";
import { ref } from "vue";
import { useErrorStore } from "@/stores/error";
import axios from "axios";

const router = useRouter();
const errorStore = useErrorStore();

const creatingAccount = ref(false);
const confirmPassword = ref(null)
const errors = ref([]);
const nameForm = ref(null);
const birthdateForm = ref(null);
const emailForm = ref(null);
const passwordForm = ref(null);
const genderForm = ref(null);
const weightForm = ref(null);

function checkForm() {
    errors.value = [];

    if (passwordForm.value != confirmPassword.value) {
        errors.value.push("Passwords don't match");
    }

    return errors.value.length == 0;
}

function registerHandler(e) {
    e.preventDefault();
    creatingAccount.value = true;
    if (!checkForm()) {
        return;
    }

    let registerData = new FormData();
    registerData.append("name", nameForm.value);
    registerData.append("birthdate", birthdateForm.value);
    registerData.append("email", emailForm.value);
    registerData.append("password", passwordForm.value);
    registerData.append("gender", genderForm.value);
    registerData.append("weight", weightForm.value);

    axios.post("/api/register", registerData)
    .then(function (response) {
        if (response.data.err) {
            errorStore.msgs.push(response.data.err);
            creatingAccount.value = false;
        } else {
            router.push({ name: "login" });
        }
    })
    .catch(function (err) {
        creatingAccount.value = false;
        errorStore.msgs.push("Failed to connect to the server (/api/register)");
    });
}
</script>

<template>
    <div class="max-w-lg mx-auto my-4 flex flex-col justify-center items-center bg-secondary-100 border-2 border-gray-700 px-4 py-4 rounded-md text-gray-700">
        <h1 class="text-4xl">Register</h1>
        <form @submit="registerHandler" action="/api/register" method="POST" class="flex flex-col">
            <div>
                <label>Name:</label>
                <input v-model="nameForm" name="name" type="text" required/>
            </div>
            <div>
                <label>Birthdate:</label>
                <input v-model="birthdateForm" name="birthdate" type="date" min="1900-01-01" :max="(new Date()).toISOString().split('T')[0]" required/>
            </div>
            <div>
                <label>E-Mail:</label>
                <input v-model="emailForm" name="email" type="email" required/>
            </div>
            <div>
                <label>Password:</label>
                <input v-model="passwordForm" name="password" type="password" required/>
            </div>
            <div>
                <label>Confirm Password:</label>
                <input type="password" v-model="confirmPassword" required/>
            </div>
            <div>
                <label>Gender:</label>
                <div class="gender">
                    <input v-model="genderForm" name="gender" type="radio" value="M" required>
                    <label>Male</label>
                </div>
                <div class="gender">
                    <input v-model="genderForm" name="gender" type="radio" value="F" required>
                    <label>Female</label>
                </div>
            </div>
            <div>
                <label>Weight</label>
                <div class="flex">
                    <input v-model="weightForm" name="weight" type="number" class="w-20" min="1" step="1" required/>
                    <label class="mx-2">kg</label>
                </div>
            </div>
            <div v-if="errors.length">
                <label class="text-red-500">Errors:</label>
                <ul>
                    <li v-for="error in errors" class="text-red-500 text-xl">{{ error }}</li>
                </ul>
            </div>
            <button class="text-2xl py-2 px-2 my-2 border-2 border-gray-700 rounded-md bg-amber-500" :class="{ 'btn-creating': creatingAccount }" :disabled="creatingAccount">
                <span v-if="!creatingAccount">Create Account</span>
                <span v-else>Creating Account...</span>
            </button>
            <a @click="router.push({ name: 'login' })" class="text-xl text-blue-500 cursor-pointer border-secondary-100 border-b-2">Already have an account? Log-in now!</a>
        </form>
    </div>
</template>

<style scoped>
label {
    @apply text-2xl;
}

input {
    @apply text-xl border-2 border-gray-700 px-1 rounded-md;
}

form > div {
    @apply my-2 flex flex-col;
}

button {
    transition: all 0.2s ease-in-out;
}

button:hover {
    @apply bg-amber-400 text-gray-600;
}

a {
    transition: all 0.2s ease-in-out;
}

a:hover {
    @apply border-blue-500;
}

.gender {
    @apply flex text-xl;
}

.gender label {
    @apply text-xl;
}

.gender input {
    @apply mx-2;
}

ul {
    list-style: square inside;
}

.btn-creating {
    @apply bg-gray-200;
}

.btn-creating:hover {
    @apply bg-gray-400;
}
</style>
