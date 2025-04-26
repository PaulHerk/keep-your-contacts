import { api_request, getCoordsFromPlace } from "$lib";
import type { Actions } from "@sveltejs/kit";
import { fail } from "@sveltejs/kit";

export const actions = {
  addPerson: async ({ request, fetch }) => {
    const data = await request.formData();

    const place = data.get("coordinate");
    const coordinateOrFail = place
      ? await getCoordsFromPlace(place as string)
      : null;
    if (coordinateOrFail && !coordinateOrFail.success) {
      return fail(500, { ...coordinateOrFail });
    }
    const coordinate = coordinateOrFail?.coordinate as Coordinate | null;

    console.log(coordinate);
    const formData: NewPerson = {
      name: data.get("name") as string,
      knownFromSourceId: null, // Placeholder
      coordinate: coordinate,
      jobTitle: data.get("jobTitle") as string,
      company: data.get("company") as string,
      linkedin: data.get("linkedin") as string,
      notes: data.get("notes") as string,
    };

    console.log("Form data:", formData);
    const response = await api_request(fetch, "/persons", {
      method: "POST",
      body: JSON.stringify(formData),
    });

    if (!response.ok)
      return fail(response.status, { message: await response.text() });

    const newPerson: Person = await response.json();
    return { success: true, newPerson };
  },
} satisfies Actions;
