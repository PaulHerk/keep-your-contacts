import { writable } from "svelte/store";

export const persons = writable([] as Person[]);
export const knownFromSources = writable([] as KnownFromSource[]);
