import { writable } from "svelte/store";

export const persons = writable<Person[]>([]);
export const knownFromSources = writable<KnownFromSource[]>([]);
export const prevQueryParams = writable<string>("");
