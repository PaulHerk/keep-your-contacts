import { error, redirect } from "@sveltejs/kit";
import type { LayoutServerLoad } from "./$types";
import { api_request } from "$lib";

export const load: LayoutServerLoad = async ({ locals, fetch }) => {
  if (!locals.user) {
    redirect(307, "/login");
  }

  const response = await api_request(fetch, "/persons/count");
  if (!response.ok) error(500, await response.text());
  const personCount = parseInt(await response.text()); // api will return number.

  return {
    loggedInUser: locals.user,
    personCount,
  };
};
