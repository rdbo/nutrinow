<script setup>
import { Menu, MenuButton, MenuItem, MenuItems } from '@headlessui/vue';
import { ChevronDownIcon, PlusCircleIcon, XCircleIcon, PencilSquareIcon, DocumentDuplicateIcon } from '@heroicons/vue/20/solid';

const props = defineProps(["curDietIndex", "diets"]);
const emit = defineEmits(["update-cur-diet", "new-diet", "delete-cur-diet", "edit-cur-diet", "duplicate-diet"]);
</script>

<template>
    <Menu as="div" class="relative inline-block text-left w-full md:w-auto">
        <div>
            <MenuButton class="inline-flex w-full justify-center gap-x-1.5 rounded-md bg-white px-3 py-2 text-lg font-semibold text-gray-900 shadow-sm ring-2 ring-inset ring-gray-300 hover:bg-gray-50 md:w-56">
                <p v-if="diets.length > 0">{{ diets[curDietIndex].name }}</p>
                <p v-else class="text-gray-300">Dashboard Menu</p>
                <div class="flex items-center">
                    <ChevronDownIcon class="-mr-1 h-5 w-5 text-gray-400" aria-hidden="true" />
                </div>
            </MenuButton>
        </div>

        <transition enter-active-class="transition ease-out duration-100" enter-from-class="transform opacity-0 scale-95" enter-to-class="transform opacity-100 scale-100" leave-active-class="transition ease-in duration-75" leave-from-class="transform opacity-100 scale-100" leave-to-class="transform opacity-0 scale-95">
            <MenuItems class="absolute left-0 z-10 w-full md:w-56 mt-2 origin-top-right rounded-md bg-white shadow-lg ring-2 ring-black ring-opacity-10 focus:outline-none">
                <div class="py-1">
                    <MenuItem v-for="(diet, index) in diets" v-slot="{ active }">
                        <div @click="$emit('update-cur-diet', index)" class="cursor-pointer" :class="[active ? 'bg-gray-100 text-gray-900' : 'text-gray-700', 'block px-4 py-2 text-lg']">{{ diet.name }}</div>
                    </MenuItem>
                    <MenuItem v-slot="{ active }">
                       <div @click="$emit('new-diet')" class="cursor-pointer flex items-center border-t-2 border-gray-300" :class="[active ? 'bg-gray-100 text-gray-900' : 'text-gray-700', 'block px-4 py-2 text-lg']">
                           <PlusCircleIcon class="w-6 mr-1"/>
                           <p>New Diet</p>
                       </div>
                    </MenuItem>
                    <MenuItem v-slot="{ active }">
                       <div @click="$emit('duplicate-diet')" class="cursor-pointer flex items-center" :class="[active ? 'bg-gray-100 text-gray-900' : 'text-gray-700', 'block px-4 py-2 text-lg']">
                           <DocumentDuplicateIcon class="w-6 mr-1"/>
                           <p>Duplicate Diet</p>
                       </div>
                    </MenuItem>
                    <MenuItem v-if="diets.length > 0" v-slot="{ active }">
                        <div @click="$emit('edit-cur-diet')" class="cursor-pointer flex items-center" :class="[active ? 'bg-gray-100 text-gray-900' : 'text-gray-700', 'block px-4 py-2 text-lg']">
                           <PencilSquareIcon class="w-6 mr-1"/>
                           <p>Edit Diet</p>
                       </div>
                    </MenuItem>
                    <MenuItem v-if="diets.length > 0" v-slot="{ active }">
                        <div @click="$emit('delete-cur-diet')" class="cursor-pointer flex items-center" :class="[active ? 'bg-gray-100 text-gray-900' : 'text-gray-700', 'block px-4 py-2 text-lg']">
                           <XCircleIcon class="w-6 mr-1"/>
                           <p>Delete Diet</p>
                       </div>
                    </MenuItem>
                </div>
            </MenuItems>
        </transition>
    </Menu>
</template>
