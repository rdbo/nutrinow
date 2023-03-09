<script setup>
import { useRouter } from 'vue-router';
import { ref } from 'vue';

const router = useRouter();

const password = ref(null);
const confirmPassword = ref(null)
const errors = ref([]);

function checkForm(e) {
    errors.value = [];

    if (password.value != confirmPassword.value) {
        errors.value.push("Passwords don't match");
    }

    if (errors.value.length > 0) {
        e.preventDefault();
    }
}

</script>

<template>
    <div class="max-w-lg mx-auto my-4 flex flex-col justify-center items-center bg-secondary-100 border-2 border-gray-700 px-4 py-4 rounded-md text-gray-700">
        <h1 class="text-4xl">Register</h1>
        <form @submit="checkForm" method="POST" class="flex flex-col">
            <div>
                <label>Name:</label>
                <input name="name" type="text" required/>
            </div>
            <div>
                <label>Birthdate:</label>
                <input name="birthdate" type="date" min="1900-01-01" :max="(new Date()).toISOString().split('T')[0]" required/>
            </div>
            <div>
                <label>E-Mail:</label>
                <input name="email" type="email" required/>
            </div>
            <div>
                <label>Password:</label>
                <input name="password" type="password" v-model="password" required/>
            </div>
            <div>
                <label>Confirm Password:</label>
                <input type="password" v-model="confirmPassword" required/>
            </div>
            <div>
                <label>Gender:</label>
                <div class="gender">
                    <input name="gender" type="radio" value="M" required>
                    <label>Male</label>
                </div>
                <div class="gender">
                    <input name="gender" type="radio" value="F" required>
                    <label>Female</label>
                </div>
            </div>
            <div v-if="errors.length">
                <label class="text-red-500">Errors:</label>
                <ul>
                    <li v-for="error in errors" class="text-red-500 text-xl">{{ error }}</li>
                </ul>
            </div>
            <button class="text-2xl py-2 px-2 my-2 border-2 border-gray-700 rounded-md bg-amber-500">Create Account</button>
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
</style>
