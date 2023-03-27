<script setup>
import { ref } from 'vue';
import { Dialog, DialogPanel, DialogTitle, TransitionChild, TransitionRoot } from '@headlessui/vue';
import { ExclamationCircleIcon } from '@heroicons/vue/24/outline';
import ServingDropdown from "./ServingDropdown.vue";

const props = defineProps(["food"]);
const emit = defineEmits(["close", "add-food"]);
const curServingIndex = ref(0);
const servingAmount = ref(props.food.servings[curServingIndex.value].amount);
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
                            <div class="h-128 overflow-y-auto">
                                <div class="bg-white pb-4">
                                    <div class="text-center">
                                        <div class="sticky top-0 bg-gray-50 pt-5 px-4 pb-4">
                                            <div class="mx-auto flex h-12 w-12 flex-shrink-0 items-center justify-center rounded-full bg-green-100">
                                                <ExclamationCircleIcon class="h-6 w-6 text-green-600" aria-hidden="true" />
                                            </div>
                                            <DialogTitle as="h3" class="text-lg font-semibold leading-6 text-gray-900 my-2">{{ food.name }}</DialogTitle>
                                            <div class="inline-flex justify-center items-center">
                                                <input v-model="servingAmount" class="text-center h-full w-28 px-2 py-2 mx-2 border-gray-300 border-2 rounded-md" placeholder="Amount" type="number" min="0"/>
                                                <ServingDropdown :servings="food.servings" :curServingIndex="curServingIndex"/>
                                            </div>
                                        </div>
                                        <div class="mt-3 px-4">
                                            <div class="mt-2 text-lg">
                                                <p class="text-gray-500">Name: <span class="text-gray-700 font-bold"></span></p>
                                                <div>
                                                    <div v-for="nutrient in food.servings[curServingIndex].nutrients">
                                                        <p>{{ nutrient.name }}</p>
                                                    </div>
                                                </div>
                                            </div>
                                        </div>
                                    </div>
                                </div>
                                <div class="bg-gray-50 px-4 py-3 sticky w-full bottom-0">
                                    <button type="button" class="inline-flex w-full justify-center rounded-md px-3 py-2 text-lg font-semibold text-white shadow-sm bg-green-400 hover:bg-green-600" @click="$emit('add-food')">Add to Meal</button>
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
