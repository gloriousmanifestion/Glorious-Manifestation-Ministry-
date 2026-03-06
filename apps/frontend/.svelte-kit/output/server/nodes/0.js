import * as universal from '../entries/pages/_layout.ts.js';

export const index = 0;
let component_cache;
export const component = async () => component_cache ??= (await import('../entries/pages/_layout.svelte.js')).default;
export { universal };
export const universal_id = "src/routes/+layout.ts";
export const imports = ["_app/immutable/nodes/0.DHiAgWQy.js","_app/immutable/chunks/DVHiahGC.js","_app/immutable/chunks/BXrGO6V1.js","_app/immutable/chunks/D-_-TNbW.js","_app/immutable/chunks/BY5_AGpK.js","_app/immutable/chunks/D3DMGJbf.js","_app/immutable/chunks/Cq4rzHIR.js","_app/immutable/chunks/ByFXtx9r.js","_app/immutable/chunks/Cb1VmGyY.js","_app/immutable/chunks/CGn7WDZ5.js","_app/immutable/chunks/DPhYa2SK.js","_app/immutable/chunks/CqH3PasK.js","_app/immutable/chunks/ajnhZGIt.js"];
export const stylesheets = ["_app/immutable/assets/app.DxC21Yhy.css"];
export const fonts = ["_app/immutable/assets/outfit-latin-ext-wght-normal.DdQaqQDo.woff2","_app/immutable/assets/outfit-latin-wght-normal.Bc-8i84L.woff2"];
