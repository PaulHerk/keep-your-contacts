import { api_get } from "$lib";
import { redirect } from "@sveltejs/kit";
import { api_url } from "../state.svelte";
import * as setCookieParser from "set-cookie-parser";
import { fail } from "@sveltejs/kit";

import type { Actions } from "./$types";

import type { PageServerLoad } from "./$types";

export const load: PageServerLoad = async ({ locals, request }) => {
  if (locals.user && request.method === "GET") {
    redirect(307, "/dashboard");
  }
};

export const actions = {
  login: async ({ request, fetch, cookies }) => {
    const data = await request.formData();
    const name = data.get("name");
    const password = data.get("password");

    console.log(name, password);

    try {
      const response = await fetch(api_url + "/auth/login", {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
        },
        body: JSON.stringify({ name, password }),
      });
      if (response.ok) {
        const token = await response.text();
        const { name, value, path, sameSite, ...options } =
          setCookieParser.parseString(token);
        cookies.set(name, value, {
          path: path ? path : "/",
          sameSite: sameSite as boolean | "lax" | "strict" | "none" | undefined,
          ...options,
        });
        return { success: true, message: "success" };
      } else if (response.status === 400) {
        return fail(400, { wrongCredentials: true });
      } else {
        return { success: false, message: await response.text() };
      }
    } catch (e) {
      return {
        success: false,
        message:
          "Something went wrong that is not your fault.: " + JSON.stringify(e),
      };
    }
  },
  register: async ({ fetch, request }) => {
    const data = await request.formData();
    const name = data.get("name");
    const email = data.get("email");
    if (!email) return fail(400, { email, missing: true });
    const password = data.get("password");

    console.log(name, password);

    try {
      const response = await fetch(api_url + "/auth/register", {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
        },
        body: JSON.stringify({ name, email, password }),
      });
      if (response.ok) {
        // console.log(response, user)
        // locals.user = user;
        return { success: true, message: "Registered! You can now Log In." };
      } else if (response.status === 409) {
        return fail(409, { name, userTaken: true });
      } else {
        return { success: false, message: await response.text() };
      }
    } catch (e) {
      return {
        success: false,
        message:
          "Something went wrong that is not your fault.: " + JSON.stringify(e),
      };
    }
    // TODO register the user
  },
} satisfies Actions;
