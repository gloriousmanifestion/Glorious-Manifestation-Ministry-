export const manifest = (() => {
function __memo(fn) {
	let value;
	return () => value ??= (value = fn());
}

return {
	appDir: "_app",
	appPath: "_app",
	assets: new Set([".DS_Store","cookies.png","favicon.png","favicon.svg","logo.svg","og-image.png","oneauth-logo.svg","pastoral-school-flyer.png","prophetess-leader.png","voice/.DS_Store","voice/hold.mp3","voice/welcome.mp3"]),
	mimeTypes: {".png":"image/png",".svg":"image/svg+xml",".mp3":"audio/mpeg"},
	_: {
		client: {start:"_app/immutable/entry/start.BBKFDdfd.js",app:"_app/immutable/entry/app.Cl3b11jr.js",imports:["_app/immutable/entry/start.BBKFDdfd.js","_app/immutable/chunks/DPhYa2SK.js","_app/immutable/chunks/BXrGO6V1.js","_app/immutable/chunks/BY5_AGpK.js","_app/immutable/entry/app.Cl3b11jr.js","_app/immutable/chunks/BXrGO6V1.js","_app/immutable/chunks/B2lhiR6I.js","_app/immutable/chunks/CqH3PasK.js","_app/immutable/chunks/DVHiahGC.js","_app/immutable/chunks/BY5_AGpK.js","_app/immutable/chunks/D3DMGJbf.js","_app/immutable/chunks/Cq4rzHIR.js","_app/immutable/chunks/C9ac3dJW.js","_app/immutable/chunks/Cb1VmGyY.js"],stylesheets:[],fonts:[],uses_env_dynamic_public:true},
		nodes: [
			__memo(() => import('./nodes/0.js')),
			__memo(() => import('./nodes/1.js'))
		],
		remotes: {
			
		},
		routes: [
			{
				id: "/api/v1/[...path]",
				pattern: /^\/api\/v1(?:\/([^]*))?\/?$/,
				params: [{"name":"path","optional":false,"rest":true,"chained":true}],
				page: null,
				endpoint: __memo(() => import('./entries/endpoints/api/v1/_...path_/_server.ts.js'))
			}
		],
		prerendered_routes: new Set(["/","/admissions","/careers","/contact","/cookies","/dashboard","/events","/login","/privacy","/programs","/register","/terms"]),
		matchers: async () => {
			
			return {  };
		},
		server_assets: {}
	}
}
})();
