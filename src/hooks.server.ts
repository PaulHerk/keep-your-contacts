import type { Handle } from "@sveltejs/kit";
import { error, redirect } from "@sveltejs/kit";
import * as set_cookie_parser from "set-cookie-parser";

import type { HandleFetch } from "@sveltejs/kit";
import { api_url } from "./routes/state.svelte";
// export const handleFetch: HandleFetch = async ({ event, request, fetch }) => {
//   if (request.url.startsWith(api_url)) {
//     let idk = set_cookie_parser.splitCookiesString(
//       request.headers.getSetCookie(),
//     );
//     console.log("Idk:", idk);
//     let cookie = event.request.headers.get("cookie");
//     console.log(cookie);
//     request.headers.set("cookie", cookie ? cookie : "");
//   }

//   return fetch(request);
// };

export const handle: Handle = async ({ event, resolve }) => {
  event.locals.user = null;
  console.log(event.url.pathname);
  // if (event.locals.user) return await resolve(event)
  if (!event.url.pathname.startsWith("/dashboard")) {
    const response = await event.fetch("http://localhost:8000/auth/me"); // if user is still logged in but not on auth-required site
    if (response.ok) {
      let user: User = await response.json();
      event.locals.user = user;
      redirect(307, "/dashboard");
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
  } catch (e) {
    console.error("Api not responding correctly.", e);
  }
  return await resolve(event);
};
