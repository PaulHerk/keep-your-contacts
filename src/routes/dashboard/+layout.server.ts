import { redirect } from "@sveltejs/kit";
import type { LayoutServerLoad } from "./$types";

export const load: LayoutServerLoad = async ({ locals, cookies }) => {
  console.log(locals.user);
  const token = cookies.get("token");
  if (!locals.user || !token) {
    redirect(307, "/login");
  }

  return {
    loggedInUser: locals.user,
    token,
  };
};
