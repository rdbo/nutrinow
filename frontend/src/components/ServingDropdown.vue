<script setup>
import { Menu, MenuButton, MenuItem, MenuItems } from '@headlessui/vue';
import { ChevronDownIcon, PlusCircleIcon, XCircleIcon, PencilSquareIcon } from '@heroicons/vue/20/solid';

const props = defineProps(["servings", "curServingIndex"]);
const emit = defineEmits(["update-cur-serving"]);
</script>

<template>
    <Menu as="div" class="relative inline-block text-left w-full md:w-auto">
        <div>
            <MenuButton class="inline-flex w-full justify-center gap-x-1.5 rounded-md bg-white px-3 py-2 text-lg font-semibold text-gray-900 shadow-sm ring-2 ring-inset ring-gray-300 hover:bg-gray-50 w-28">
                <p v-if="servings.length > 0">{{ servings[curServingIndex].unit }}</p>
                <div class="flex items-center">
                    <ChevronDownIcon class="-mr-1 h-5 w-5 text-gray-400" aria-hidden="true" />
                </div>
            </MenuButton>
        </div>

        <transition enter-active-class="transition ease-out duration-100" enter-from-class="transform opacity-0 scale-95" enter-to-class="transform opacity-100 scale-100" leave-active-class="transition ease-in duration-75" leave-from-class="transform opacity-100 scale-100" leave-to-class="transform opacity-0 scale-95">
            <MenuItems class="absolute left-0 z-10 w-full w-28 max-h-96 overflow-y-auto mt-2 origin-top-right rounded-md bg-white shadow-lg ring-2 ring-black ring-opacity-10 focus:outline-none">
                <div class="py-1">
                    <MenuItem v-for="(serving, index) in servings" v-slot="{ active }">
                        <div @click="$emit('update-cur-serving', index)" class="cursor-pointer" :class="[active ? 'bg-gray-100 text-gray-900' : 'text-gray-700', 'block px-4 py-2 text-lg']">{{ serving.unit }}</div>
                    </MenuItem>
                </div>
            </MenuItems>
        </transition>
    </Menu>
</template>
