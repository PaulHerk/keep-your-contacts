import type { Handle } from "@sveltejs/kit";
import { error, redirect } from "@sveltejs/kit";

export const handle: Handle = async ({ event, resolve }) => {
  event.locals.user = null;
  if (!event.url.pathname.startsWith("/dashboard")) {
    const response = await event.fetch("http://localhost:8000/auth/me"); // if user is still logged in but not on auth-required site
    if (response.ok) {
      let user: User = await response.json();
      event.locals.user = user;
    } // !TODO: remove code duplication
    return await resolve(event);
  }

  try {
    const response = await event.fetch("http://localhost:8000/auth/me");
    if (response.ok) {
      let user: User = await response.json();
      event.locals.user = user;
    } else if (response.status === 401) {
      console.error("Unauthorized bb");
      error(404);
    } else if (response.status === 500) {
      console.error("500:", response.text.toString());
      error(500);
    }
  } catch {
    console.error("Api not responding correctly.");
  }
  return await resolve(event);
};
