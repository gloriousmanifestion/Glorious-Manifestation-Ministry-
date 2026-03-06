export const manifest = (() => {
function __memo(fn) {
	let value;
	return () => value ??= (value = fn());
}

return {
	appDir: "_app",
	appPath: "_app",
	assets: new Set(["cookies.png","favicon.svg","oneauth-logo.svg"]),
	mimeTypes: {".png":"image/png",".svg":"image/svg+xml"},
	_: {
		client: {start:"_app/immutable/entry/start.C2cBu1K1.js",app:"_app/immutable/entry/app.C1IngWFR.js",imports:["_app/immutable/entry/start.C2cBu1K1.js","_app/immutable/chunks/troYN5uf.js","_app/immutable/chunks/DegViNYW.js","_app/immutable/chunks/BUFiasJp.js","_app/immutable/entry/app.C1IngWFR.js","_app/immutable/chunks/DegViNYW.js","_app/immutable/chunks/E3gk6-eO.js","_app/immutable/chunks/l_W-_EEB.js","_app/immutable/chunks/BUFiasJp.js","_app/immutable/chunks/BxtSLVu0.js","_app/immutable/chunks/DO9qiojC.js"],stylesheets:[],fonts:[],uses_env_dynamic_public:true},
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
		prerendered_routes: new Set(["/","/careers","/contact","/cookies","/privacy","/terms"]),
		matchers: async () => {
			
			return {  };
		},
		server_assets: {}
	}
}
})();

export const prerendered = new Set(["/","/careers","/contact","/cookies","/privacy","/terms"]);

export const base = "";