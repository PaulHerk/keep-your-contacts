import type { Handle } from "@sveltejs/kit";
import { redirect } from "@sveltejs/kit";
import { api_url } from "$lib";
import type { HandleFetch } from "@sveltejs/kit";

export const handleFetch: HandleFetch = async ({ event, request, fetch }) => {
  if (request.url.startsWith("https://api.my-domain.com/")) {
    request.headers.set("cookie", event.request.headers.get("cookie") ?? "");
  }

  return fetch(request);
};

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
