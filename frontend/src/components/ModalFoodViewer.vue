<script setup>
import { ref, computed } from 'vue';
import ServingDropdown from "./ServingDropdown.vue";
import { getServingNutrients } from "../composables/foods.js";
import { Dialog, DialogPanel, DialogTitle, TransitionChild, TransitionRoot } from '@headlessui/vue';
import { ExclamationCircleIcon } from '@heroicons/vue/24/outline';

const props = defineProps(["food"]);
const emit = defineEmits(["close", "add-food"]);
const curServingIndex = ref(0);
const servingAmount = ref(props.food.servings[curServingIndex.value].amount);
const formServingAmount = ref(null);
const nutrientList = computed(() => {
    return getServingNutrients(props.food.servings, curServingIndex.value);
});

function updateCurServing(index) {
    curServingIndex.value = index;
    let serving = props.food.servings[curServingIndex.value];
    if (!serving.relative) {
        servingAmount.value = serving.amount;
    } else {
        servingAmount.value = 1;
    }
}
</script>

<template>
    <TransitionRoot as="template" :show="food != null">
        <Dialog as="div" class="relative z-10" @close="$emit('close')">
            <TransitionChild as="template" enter="ease-out duration-300" enter-from="opacity-0" enter-to="opacity-100" leave="ease-in duration-200" leave-from="opacity-100" leave-to="opacity-0">
                <div class="fixed inset-0 bg-gray-500 bg-opacity-75 transition-opacity" />
            </TransitionChild>

            <div class="fixed inset-0 z-10 overflow-y-auto">
                <div class="flex min-h-full items-end justify-center p-4 text-center items-center">
                    <TransitionChild as="template" enter="ease-out duration-300" enter-from="opacity-0 translate-y-4" enter-to="opacity-100 translate-y-0" leave="ease-in duration-200" leave-from="opacity-100 translate-y-0" leave-to="opacity-0 translate-y-4">
                        <DialogPanel class="relative transform overflow-hidden rounded-lg bg-white text-left shadow-xl transition-all">
                            <div class="h-100 md:h-128 overflow-y-auto">
                                <div class="bg-white">
                                    <div class="text-center">
                                        <div class="sticky top-0 bg-gray-50 pt-5 px-4 pb-4">
                                            <div class="mx-auto flex h-12 w-12 flex-shrink-0 items-center justify-center rounded-full bg-green-100">
                                                <ExclamationCircleIcon class="h-6 w-6 text-green-600" aria-hidden="true" />
                                            </div>
                                            <DialogTitle as="h3" class="text-lg font-semibold leading-6 text-gray-900 my-2">{{ food.name }}</DialogTitle>
                                            <div class="inline-flex">
                                                <div class="flex flex-col">
                                                    <input ref="formServingAmount" v-model="servingAmount" @input="servingAmount = formServingAmount.value" class="grow text-center w-28 px-2 py-2 mx-2 border-gray-300 border-2 rounded-md" placeholder="Amount" type="number" min="0"/>
                                                </div>
                                                <ServingDropdown @update-cur-serving="updateCurServing" :servings="food.servings" :curServingIndex="curServingIndex"/>
                                            </div>

                                        </div>
                                        <div>
                                            <div class="text-lg">
                                                <div>
                                                    <table class="w-full text-center bg-orange-100">
                                                        <thead class="border-y-2 border-gray-400 bg-orange-200">
                                                            <tr>
                                                                <th class="border-r-2 border-gray-400">Nutrient</th>
                                                                <th>Amount (per {{ servingAmount }} "{{ food.servings[curServingIndex].unit  }}")</th>
                                                            </tr>
                                                        </thead>
                                                        <tbody>
                                                            <tr v-for="nutrient in nutrientList" class="border-gray-400 border-b-2">
                                                                <td class="border-r-2 border-gray-400">{{ nutrient.name }}</td>
                                                                <td>{{ Number(nutrient.amount * (servingAmount / (food.servings[curServingIndex].relative ? 1 : food.servings[curServingIndex].amount))).toFixed(1) }}{{ nutrient.unit }}</td>
                                                            </tr>
                                                        </tbody>
                                                    </table>
                                                </div>
                                            </div>
                                        </div>
                                    </div>
                                </div>
                                <div class="bg-gray-50 px-4 py-3 sticky w-full bottom-0">
                                    <button type="button" class="inline-flex w-full justify-center rounded-md px-3 py-2 text-lg font-semibold text-white shadow-sm bg-green-400 hover:bg-green-600" @click="$emit('add-food', food.servings[curServingIndex].id, servingAmount)">Add to Meal</button>
                                    <button type="button" class="mt-3 inline-flex w-full justify-center rounded-md bg-white px-3 py-2 text-lg font-semibold text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 hover:bg-gray-50" @click="$emit('close')" ref="cancelButtonRef">Close</button>
                                </div>
                            </div>
                        </DialogPanel>
                    </TransitionChild>
                </div>
            </div>
        </Dialog>
    </TransitionRoot>
</template>
