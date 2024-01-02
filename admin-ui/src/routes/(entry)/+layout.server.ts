import type { LayoutServerLoad } from './$types';

export const load: LayoutServerLoad = async () => {
	const res = await fetch('http://0.0.0.0:3006/api/login_schema');
    const schema = await res.json();
    return { schema };
};