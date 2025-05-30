import type { RequestHandler } from './$types';
import { LinkedInProfileScraper } from 'linkedin-profile-scraper';

// Plain Javascript
// const { LinkedInProfileScraper } = require('linkedin-profile-scraper')

const scrape = async () => {
	const scraper = new LinkedInProfileScraper({
		sessionCookieValue: 'AQEDAVtMFgsDFbbFAAABlyCz6-oAAAGXRMBv6k0AN1L0AXmVutKEdQBZB8mfXSf3-ihrPhzp-mtvGN4nSPX0e2YwCyPFmwbHQ6YDUk5icxBv4q56FuKoLj2nmM4ENLqNRZWt09xzzTr6DEJYna_4nKQe',
		keepAlive: false
	});

	// Prepare the scraper
	// Loading it in memory
	await scraper.setup()

	const result = await scraper.run('https://www.linkedin.com/in/williamhgates/')

	console.log(result)
	return result
}


export const GET: RequestHandler = async ({ }) => {
	const res = await scrape();

	return new Response(Object(res));
};
