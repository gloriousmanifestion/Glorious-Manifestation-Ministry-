import * as universal from '../entries/pages/_layout.ts.js';

export const index = 0;
let component_cache;
export const component = async () => component_cache ??= (await import('../entries/pages/_layout.svelte.js')).default;
export { universal };
export const universal_id = "src/routes/+layout.ts";
export const imports = ["_app/immutable/nodes/0.CkuT3t7V.js","_app/immutable/chunks/l_W-_EEB.js","_app/immutable/chunks/DegViNYW.js","_app/immutable/chunks/2BPMgg6F.js","_app/immutable/chunks/C7zo8GlA.js"];
export const stylesheets = ["_app/immutable/assets/app.C8tCKdNy.css"];
export const fonts = ["_app/immutable/assets/outfit-latin-ext-wght-normal.DdQaqQDo.woff2","_app/immutable/assets/outfit-latin-wght-normal.Bc-8i84L.woff2"];
