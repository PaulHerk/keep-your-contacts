// See https://svelte.dev/docs/kit/types#app.d.ts
// for information about these interfaces
declare global {
	namespace App {
		// interface Error {}
		// interface Locals {}
		// interface PageData {}
		// interface PageState {}
		// interface Platform {}
	}
	interface Person {
		id: number;
		first_name: string;
		last_name: string;
		city: string;
		note: string;
	}
	interface NewPerson {
		first_name: string;
		last_name: string;
		city: string;
		note: string;
	}
}

export { };
