<script setup>
import { useRouter } from "vue-router";
import { ref } from "vue";
import axios from "axios";

const router = useRouter();

const creatingAccount = ref(false);
const password = ref(null);
const confirmPassword = ref(null)
const errors = ref([]);
const nameForm = ref(null);
const birthdateForm = ref(null);
const emailForm = ref(null);
const passwordForm = ref(null);
const genderForm = ref(null);

function checkForm() {
    errors.value = [];

    if (password.value != confirmPassword.value) {
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
    registerData.append("name", nameForm.value.value);
    registerData.append("birthdate", birthdateForm.value.value);
    registerData.append("email", emailForm.value.value);
    registerData.append("password", passwordForm.value.value);
    registerData.append("gender", genderForm.value.value);

    axios.post("/api/register", registerData)
    .then(function (response) {
        if (response.data.err) {
            // TODO: Show error message to user
            console.log("failed to log in");
            creatingAccount.value = false;
        } else {
            router.push({ name: "login" });
        }
    }).catch(function (err) {
        // TODO: Handle error
        console.log("/api/register request error: " + err);
        waitingLogin.value = false;
    });
}
</script>

<template>
    <div class="max-w-lg mx-auto my-4 flex flex-col justify-center items-center bg-secondary-100 border-2 border-gray-700 px-4 py-4 rounded-md text-gray-700">
        <h1 class="text-4xl">Register</h1>
        <form @submit="registerHandler" action="/api/register" method="POST" class="flex flex-col">
            <div>
                <label>Name:</label>
                <input ref="nameForm" name="name" type="text" required/>
            </div>
            <div>
                <label>Birthdate:</label>
                <input ref="birthdateForm" name="birthdate" type="date" min="1900-01-01" :max="(new Date()).toISOString().split('T')[0]" required/>
            </div>
            <div>
                <label>E-Mail:</label>
                <input ref="emailForm" name="email" type="email" required/>
            </div>
            <div>
                <label>Password:</label>
                <input ref="passwordForm" name="password" type="password" v-model="password" required/>
            </div>
            <div>
                <label>Confirm Password:</label>
                <input type="password" v-model="confirmPassword" required/>
            </div>
            <div>
                <label>Gender:</label>
                <div class="gender">
                    <input ref="genderForm" name="gender" type="radio" value="M" required>
                    <label>Male</label>
                </div>
                <div class="gender">
                    <input ref="genderForm" name="gender" type="radio" value="F" required>
                    <label>Female</label>
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
