<script setup>
import { useRouter } from 'vue-router';

const router = useRouter();
</script>

<template>
    <div class="max-w-lg mx-auto my-4 flex flex-col justify-center items-center bg-secondary-100 border-2 border-gray-700 px-4 py-4 rounded-md text-gray-700">
        <h1 class="text-4xl">Register</h1>
        <form method="POST" class="flex flex-col">
            <div>
                <label>Name:</label>
                <input type="text" required/>
            </div>
            <div>
                <label>Birthdate:</label>
                <input type="date" required/>
            </div>
            <div>
                <label>E-Mail:</label>
                <input type="email" required/>
            </div>
            <div>
                <label>Password:</label>
                <input type="password" required/>
            </div>
            <div>
                <label>Confirm Password:</label>
                <input type="password" required/>
            </div>
            <div>
                <label>Gender:</label>
                <div class="gender">
                    <input id="gender_male" name="gender" type="radio" value="Male" required>
                    <label for="gender_male">Male</label>
                </div>
                <div class="gender">
                    <input id="gender_female" name="gender" type="radio" value="Female" required>
                    <label for="gender_female">Female</label>
                </div>
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
</style>
