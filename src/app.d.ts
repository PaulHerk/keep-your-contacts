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

	}
	class NewPerson {
		name: string;
		age: number;
		email: string;
		phone: string;

		constructor() {
			this.name = "";
			this.age = 0;
			this.email = "";
			this.phone = "";
		}
	}
}

export { };
