import { api_url } from "$lib";
import { redirect } from "@sveltejs/kit";
import * as setCookieParser from "set-cookie-parser";
import { fail } from "@sveltejs/kit";

import type { Actions } from "./$types";

import type { PageServerLoad } from "./$types";

export const load: PageServerLoad = async ({ request, cookies }) => {
  if (cookies.get("token") && request.method === "GET") {
    redirect(307, "/dashboard");
  }
};

export const actions = {
  login: async ({ request, fetch, cookies, locals }) => {
    const data = await request.formData();
    const name = data.get("name");
    const password = data.get("password");

    try {
      const response = await fetch(api_url + "/auth/login", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ name, password }),
      });

      if (!response.ok) {
        let wrongCredentials = response.status === 400;
        return fail(response.status, {
          wrongCredentials,
        });
      }

      const token = await response.text();
      const {
        name: cookieName,
        value,
        path,
        sameSite,
        ...options
      } = setCookieParser.parseString(token);
      cookies.set(cookieName, value, {
        path: path ? path : "/",
        sameSite: sameSite as boolean | "lax" | "strict" | "none" | undefined,
        ...options,
      });
    } catch (e) {
      console.error("Login failed:", e);
      return fail(500, {
        message: "An unexpected error occurred",
      });
    }
    return redirect(307, "/dashboard"); // this will redirect to the dashboard if the login was successful since errors would have been caught above
  },
  register: async ({ fetch, request }) => {
    const data = await request.formData();
    const name = data.get("name");
    const email = data.get("email");
    if (!email) return fail(400, { emailMissing: true });
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
        return fail(409, { userTaken: true });
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
