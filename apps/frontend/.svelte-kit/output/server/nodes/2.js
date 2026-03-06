import * as universal from '../entries/pages/dashboard/_layout.ts.js';

export const index = 2;
let component_cache;
export const component = async () => component_cache ??= (await import('../entries/fallbacks/layout.svelte.js')).default;
export { universal };
export const universal_id = "src/routes/dashboard/+layout.ts";
export const imports = ["_app/immutable/nodes/2.BtCd66cE.js","_app/immutable/chunks/DVHiahGC.js","_app/immutable/chunks/BXrGO6V1.js","_app/immutable/chunks/DhcPj9z8.js","_app/immutable/chunks/Cq4rzHIR.js"];
export const stylesheets = [];
export const fonts = [];
