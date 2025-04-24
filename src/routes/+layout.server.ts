import { redirect } from "@sveltejs/kit"
import type { LayoutServerLoad } from "./$types"

export const load: LayoutServerLoad = async ({ locals, url }) => {
    if (!url.pathname.startsWith("/login") && !locals.user) {
        redirect(307, "/login")
    } else if (url.pathname.startsWith("/login") && locals.user) {
        redirect(307, "/dashboard")
    }
    return {
        loggedInUser: locals.user,
    }
}