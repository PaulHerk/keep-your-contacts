// See https://svelte.dev/docs/kit/types#app.d.ts
// for information about these interfaces
declare global {
  namespace App {
    // interface Error {}
    interface Locals {
      user: User | null;
    }
    // interface PageData {}
    // interface PageState {}
    // interface Platform {}
  }
  interface User {
    id: number;
    name: String;
    email: String;
    createdAt: String;
    updatedAt: String;
  }

  interface Person {
    id: number;
    first_name: string;
    last_name: string;
    known_from_source_id: number?;
    job_title: string;
    company: string;
    linkedin: string;
    notes: string;
    createdAt: string;
    record: Record;
  }
  interface Record {
    lat: number;
    lon: number;
    name: string;
    admin1: string;
    admin2: string;
    cc: string;
  }
  interface NewPerson {
    first_name: string;
    last_name: string;
    city: string;
    job: string;
    note: string;
  }
}

export {};
