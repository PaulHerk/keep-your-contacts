import type { Handle } from "@sveltejs/kit";
import { redirect } from "@sveltejs/kit";
import { api_url } from "$lib";

export const handle: Handle = async ({ event, resolve }) => {
  // Only verify token if accessing protected routes
  if (!event.url.pathname.startsWith("/dashboard")) {
    return await resolve(event);
  }

  const token = event.cookies.get("token");
  if (!token) {
    return redirect(307, "/login");
  }

  try {
    const response = await event.fetch(api_url + "/auth/me", {
      headers: {
        Cookie: `token=${token}`,
      },
    });

    if (response.ok) {
      event.locals.user = await response.json();
    } else {
      event.cookies.delete("token", { path: "/" });
      event.locals.user = null;
      return redirect(307, "/login");
    }
  } catch (e) {
    console.error("Auth verification failed:", e);
    return redirect(307, "/login");
  }

  return await resolve(event);
};
