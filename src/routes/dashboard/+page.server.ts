import { api_request } from "$lib";
import type { Actions } from "@sveltejs/kit";
import { fail } from "@sveltejs/kit";
import { persons } from "./store";

export const actions = {
  addPerson: async ({ request, fetch }) => {
    const data = await request.formData();

    const formData: NewPerson = {
      name: data.get("name") as string,
      knownFromSourceId: null, // Placeholder
      coordinateOrOsmSearch: data.get("coordinateOrOsmSearch") as string | null,
      jobTitle: data.get("jobTitle") as string,
      company: data.get("company") as string,
      linkedin: data.get("linkedin") as string,
      notes: data.get("notes") as string,
    };

    try {
      console.log("Form data:", formData);
      const response = await api_request(fetch, "/persons", {
        method: "POST",
        body: JSON.stringify(formData),
      });

      const newPersonId: Person = await response.json();
      return { success: true, newPersonId };
    } catch (error) {
      console.error("Failed to add person:", error);
      return fail(500, {
        success: false,
        message: "Failed to add person",
      });
    }
  },
} satisfies Actions;
