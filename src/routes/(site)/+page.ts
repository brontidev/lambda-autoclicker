import { redirect } from '@sveltejs/kit';

export const prerender = false;

export async function load({  }) {
    redirect(302, '/site')
}