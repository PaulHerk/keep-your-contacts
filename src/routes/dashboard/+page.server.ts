import { api_request, getCoordsFromPlace } from "$lib";
import type { Actions } from "@sveltejs/kit";
import { fail } from "@sveltejs/kit";

const createNewPersonObj = async (formData: FormData) => {
  const place = formData.get("coordinateOrSearch");
  let coordinate;
  try {
    coordinate = JSON.parse(place as string) as Coordinate;
  } catch {
    const coordinateOrFail = place
      ? await getCoordsFromPlace(place as string)
      : null;
    if (coordinateOrFail && !coordinateOrFail.success) {
      return coordinateOrFail;
    }
    coordinate = coordinateOrFail?.coordinate as Coordinate | null;
  }
  let knownFromSourceId = formData.get("knownFromSourceId") as number | null;
  knownFromSourceId = knownFromSourceId ? Number(knownFromSourceId) : null;

  return {
    success: true,
    person: {
      name: formData.get("name") as string,
      knownFromSourceId: knownFromSourceId, // Placeholder
      coordinate,
      jobTitle: formData.get("jobTitle") as string,
      company: formData.get("company") as string,
      linkedin: formData.get("linkedin") as string,
      notes: formData.get("notes") as string,
    } as NewPerson,
  };
};

export const actions = {
  addPerson: async ({ request, fetch }) => {
    const data = await request.formData();

    const parsedFormData = await createNewPersonObj(data);
    if (!parsedFormData.success) return fail(500, parsedFormData);
    const personToAdd = (parsedFormData as any).person; // this works since if personToAdd only exists if it succeeded

    const response = await api_request(fetch, "/persons", {
      method: "POST",
      body: JSON.stringify(personToAdd),
    });

    if (!response.ok)
      return fail(response.status, {
        success: false,
        message: await response.text(),
      });

    const newPerson: Person = await response.json();
    return { success: true, newPerson };
  },

  updatePerson: async ({ request, fetch, route, url }) => {
    const data = await request.formData();

    const parsedFormData = await createNewPersonObj(data);
    if (!parsedFormData.success) return fail(500, parsedFormData);
    const personToUpdate = (parsedFormData as any).person; // this works since if personToAdd only exists if it succeeded

    const id = data.get("personId");
    if (!id)
      return fail(500, {
        success: false,
        message: "Weird form input Ig.",
      });

    const response = await api_request(
      fetch,
      `/persons/${id as any as number}`,
      {
        method: "PUT",
        body: JSON.stringify(personToUpdate),
      }
    );

    if (!response.ok)
      return fail(response.status, { message: await response.text() });

    const person = await response.json();

    return { success: true, newPerson: person };
  },
} satisfies Actions;
