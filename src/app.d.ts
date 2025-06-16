// See https://svelte.dev/docs/kit/types#app.d.ts
// for information about these interfaces
declare global {
	namespace App {
		interface Error {
			message: string;
		}

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
		settings: Settings;
		createdAt: String;
		updatedAt: String;
	}

	interface Settings {
		perPage: number;
	}

	interface Person {
		id: number;
		firstName: string;
		lastName: string;
		knownFromSourceId: number?;
		jobTitle: string;
		company: string;
		website: string;
		notes: string;
		birthday: Date;
		createdAt: string;
		record: PlaceRecord?;
	}
	interface PlaceRecord {
		search: string;
		lat: number;
		lon: number;
		name: string;
		admin1: string;
		admin2: string;
		cc: string;
	}

	interface Coordinate {
		lat: number;
		lon: number;
	}

	interface CoordinateSearch {
		search: string;
		lat: number;
		lon: number;
	}

	interface KnownFromSource {
		sourceId: number;
		sourceName: string;
		description: string?;
		locationSearch: string?;
	}

	interface NewPerson {
		name: string;
		knownFromSourceId: number?;
		coordinateWithSearch: CoordinateSearch?;
		jobTitle: string;
		company: string;
		website: string;
		birthday: string?;
		notes: string;
	}
}

export { };
