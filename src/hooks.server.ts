import type { Handle } from '@sveltejs/kit';
import { redirect } from '@sveltejs/kit';


export const handle: Handle = async ({ event, resolve }) => {
    event.locals.user = null;
    if (!event.url.pathname.startsWith("/dashboard")) {
        return await resolve(event);
    }

    try {
        const response = await event.fetch("http://localhost:8000/auth/me");
        if (response.ok) {
            let user: User = await response.json();
            event.locals.user = user;
        } else if (response.status === 401) {
            console.error("Unauthorized bb");
        } else if (response.status === 500) {
            console.error("500:", response.text.toString())
        }

    } catch {
        console.error("Api not responding correctly.")
    }
    const response = await resolve(event)
    return response;
}