import appAdapter from '@sveltejs/adapter-static';
import siteAdapter from "sveltekit-adapter-deno"

const adapter = { 'build:app': appAdapter({ strict: false }), 'build:site': siteAdapter({ }) }[process.env['npm_lifecycle_event']]

/** @type {import('@sveltejs/kit').Config} */
const config = {
	kit: {
		adapter
	}
};

export default config;
