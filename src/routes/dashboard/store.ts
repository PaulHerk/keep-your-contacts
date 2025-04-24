import { writable } from 'svelte/store';

export const persons = writable([] as Person[])