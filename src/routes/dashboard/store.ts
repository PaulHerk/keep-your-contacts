import { writable } from "svelte/store";

export const persons = writable<Person[]>([]);
export const knownFromSources = writable<KnownFromSource[]>([]);
export const prevQueryParams = writable<string>("");
export const settings = writable<Settings>();

export const authToken = writable<string | null>(null);
