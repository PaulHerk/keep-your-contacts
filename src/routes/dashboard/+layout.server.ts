import { error, redirect } from "@sveltejs/kit";
import type { LayoutServerLoad } from "./$types";
import { api_request } from "$lib";

export const load: LayoutServerLoad = async ({ locals, fetch }) => {
  if (!locals.user) {
    redirect(307, "/login");
  }

  const kfsResponse = await api_request(fetch, "/known-from-sources");
  if (!kfsResponse.ok) error(500, await kfsResponse.text());
  const knownFromSources: KnownFromSource[] = await kfsResponse.json();

  const pcResponse = await api_request(fetch, "/persons/count");
  if (!pcResponse.ok) error(500, await pcResponse.text());
  const personCount = parseInt(await pcResponse.text()); // api will return number.

  return {
    loggedInUser: locals.user,
    personCount,
    knownFromSources,
  };
};
