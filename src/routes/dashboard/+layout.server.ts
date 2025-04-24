import { redirect } from "@sveltejs/kit"
import type { LayoutServerLoad } from "./$types"

export const load: LayoutServerLoad = async ({ locals, fetch }) => {
    if (!locals.user) {
        redirect(307, "/login")
    }
    return {
        loggedInUser: locals.user,
    }
}