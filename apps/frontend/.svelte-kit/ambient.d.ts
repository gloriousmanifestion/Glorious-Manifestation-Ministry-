
// this file is generated — do not edit it


/// <reference types="@sveltejs/kit" />

/**
 * Environment variables [loaded by Vite](https://vitejs.dev/guide/env-and-mode.html#env-files) from `.env` files and `process.env`. Like [`$env/dynamic/private`](https://svelte.dev/docs/kit/$env-dynamic-private), this module cannot be imported into client-side code. This module only includes variables that _do not_ begin with [`config.kit.env.publicPrefix`](https://svelte.dev/docs/kit/configuration#env) _and do_ start with [`config.kit.env.privatePrefix`](https://svelte.dev/docs/kit/configuration#env) (if configured).
 * 
 * _Unlike_ [`$env/dynamic/private`](https://svelte.dev/docs/kit/$env-dynamic-private), the values exported from this module are statically injected into your bundle at build time, enabling optimisations like dead code elimination.
 * 
 * ```ts
 * import { API_KEY } from '$env/static/private';
 * ```
 * 
 * Note that all environment variables referenced in your code should be declared (for example in an `.env` file), even if they don't have a value until the app is deployed:
 * 
 * ```
 * MY_FEATURE_FLAG=""
 * ```
 * 
 * You can override `.env` values from the command line like so:
 * 
 * ```sh
 * MY_FEATURE_FLAG="enabled" npm run dev
 * ```
 */
declare module '$env/static/private' {
	export const NVM_INC: string;
	export const _ZO_DOCTOR: string;
	export const LDFLAGS: string;
	export const VSCODE_CLI: string;
	export const TERM_PROGRAM: string;
	export const VSCODE_CRASH_REPORTER_PROCESS_TYPE: string;
	export const ZLE_RPROMPT_INDENT: string;
	export const NODE: string;
	export const ANDROID_HOME: string;
	export const NVM_CD_FLAGS: string;
	export const POSH_SHELL: string;
	export const INIT_CWD: string;
	export const SHELL: string;
	export const TERM: string;
	export const VSCODE_PROCESS_TITLE: string;
	export const CPPFLAGS: string;
	export const HOMEBREW_REPOSITORY: string;
	export const TMPDIR: string;
	export const CURSOR_CLI: string;
	export const CONDA_PROMPT_MODIFIER: string;
	export const GROQ_API_KEY: string;
	export const TERM_PROGRAM_VERSION: string;
	export const npm_config_npm_globalconfig: string;
	export const NODE_OPTIONS: string;
	export const CURSOR_TRACE_ID: string;
	export const MallocNanoZone: string;
	export const APP_NAME: string;
	export const TERM_SESSION_ID: string;
	export const NO_COLOR: string;
	export const npm_config_registry: string;
	export const POWERLINE_COMMAND: string;
	export const NDK_HOME: string;
	export const NVM_DIR: string;
	export const POSH_SESSION_ID: string;
	export const USER: string;
	export const COMMAND_MODE: string;
	export const OPENAI_API_KEY: string;
	export const SUBSCRIPTION: string;
	export const npm_config_globalconfig: string;
	export const PNPM_SCRIPT_SRC_DIR: string;
	export const KUBECONFIG: string;
	export const SSH_AUTH_SOCK: string;
	export const PYENV_VIRTUALENV_DISABLE_PROMPT: string;
	export const __CF_USER_TEXT_ENCODING: string;
	export const RESOURCE_GROUP: string;
	export const npm_execpath: string;
	export const AWS_PROFILE: string;
	export const NOOCASH_AZURE: string;
	export const VIRTUAL_ENV_DISABLE_PROMPT: string;
	export const npm_config_frozen_lockfile: string;
	export const npm_config_verify_deps_before_run: string;
	export const OPHER_SUB_SUBID: string;
	export const PATH: string;
	export const npm_package_json: string;
	export const __CFBundleIdentifier: string;
	export const DOTNET_ROOT: string;
	export const ENV_NAME: string;
	export const PWD: string;
	export const npm_command: string;
	export const ELECTRON_NO_ATTACH_CONSOLE: string;
	export const JAVA_HOME: string;
	export const OPHER_SUBSCRIPTION_ID: string;
	export const VSCODE_HANDLES_UNCAUGHT_ERRORS: string;
	export const VSCODE_ESM_ENTRYPOINT: string;
	export const npm_config__jsr_registry: string;
	export const npm_lifecycle_event: string;
	export const LANG: string;
	export const POSH_SHELL_VERSION: string;
	export const CURSOR_AGENT: string;
	export const npm_package_name: string;
	export const CF_GH_TOKEN: string;
	export const CURSOR_CLI_MODE: string;
	export const NODE_PATH: string;
	export const CONVENTIONAL_GITHUB_RELEASER_TOKEN: string;
	export const XPC_FLAGS: string;
	export const CURSOR_EXTENSION_HOST_ROLE: string;
	export const RELUGG_SUBSCRIPTION_ID: string;
	export const TAURI_PRIVATE_KEY: string;
	export const FORCE_COLOR: string;
	export const JFX_PATH: string;
	export const VSCODE_NODE_OPTIONS: string;
	export const npm_config_node_gyp: string;
	export const XPC_SERVICE_NAME: string;
	export const npm_package_version: string;
	export const pnpm_config_verify_deps_before_run: string;
	export const GPG_TTY: string;
	export const HOME: string;
	export const SHLVL: string;
	export const ANTHROPIC_BASE_URL: string;
	export const OSTYPE: string;
	export const RACK_VPS_IP: string;
	export const VSCODE_NLS_CONFIG: string;
	export const HOMEBREW_PREFIX: string;
	export const CI: string;
	export const ANTHROPIC_AUTH_TOKEN: string;
	export const KERAS_BACKEND: string;
	export const LOGNAME: string;
	export const TAURI_KEY_PASSWORD: string;
	export const MODULAR_HOME: string;
	export const npm_lifecycle_script: string;
	export const ARIMMA_AZURE: string;
	export const LC_CTYPE: string;
	export const VSCODE_CODE_CACHE_PATH: string;
	export const VSCODE_IPC_HOOK: string;
	export const GITHUB_TOKEN: string;
	export const NVM_BIN: string;
	export const PKG_CONFIG_PATH: string;
	export const GITHUB_USERNAME: string;
	export const VSCODE_PID: string;
	export const npm_config_ignore_scripts: string;
	export const npm_config_user_agent: string;
	export const ANDROID_NDK_HOME: string;
	export const HOMEBREW_CELLAR: string;
	export const INFOPATH: string;
	export const JAX_PLATFORM: string;
	export const OSLogRateLimit: string;
	export const LOCATION: string;
	export const VSCODE_CWD: string;
	export const COLORTERM: string;
	export const GH_TOKEN: string;
	export const OLLAMA_HOST: string;
	export const npm_node_execpath: string;
	export const NODE_ENV: string;
}

/**
 * Similar to [`$env/static/private`](https://svelte.dev/docs/kit/$env-static-private), except that it only includes environment variables that begin with [`config.kit.env.publicPrefix`](https://svelte.dev/docs/kit/configuration#env) (which defaults to `PUBLIC_`), and can therefore safely be exposed to client-side code.
 * 
 * Values are replaced statically at build time.
 * 
 * ```ts
 * import { PUBLIC_BASE_URL } from '$env/static/public';
 * ```
 */
declare module '$env/static/public' {
	
}

/**
 * This module provides access to runtime environment variables, as defined by the platform you're running on. For example if you're using [`adapter-node`](https://github.com/sveltejs/kit/tree/main/packages/adapter-node) (or running [`vite preview`](https://svelte.dev/docs/kit/cli)), this is equivalent to `process.env`. This module only includes variables that _do not_ begin with [`config.kit.env.publicPrefix`](https://svelte.dev/docs/kit/configuration#env) _and do_ start with [`config.kit.env.privatePrefix`](https://svelte.dev/docs/kit/configuration#env) (if configured).
 * 
 * This module cannot be imported into client-side code.
 * 
 * ```ts
 * import { env } from '$env/dynamic/private';
 * console.log(env.DEPLOYMENT_SPECIFIC_VARIABLE);
 * ```
 * 
 * > [!NOTE] In `dev`, `$env/dynamic` always includes environment variables from `.env`. In `prod`, this behavior will depend on your adapter.
 */
declare module '$env/dynamic/private' {
	export const env: {
		NVM_INC: string;
		_ZO_DOCTOR: string;
		LDFLAGS: string;
		VSCODE_CLI: string;
		TERM_PROGRAM: string;
		VSCODE_CRASH_REPORTER_PROCESS_TYPE: string;
		ZLE_RPROMPT_INDENT: string;
		NODE: string;
		ANDROID_HOME: string;
		NVM_CD_FLAGS: string;
		POSH_SHELL: string;
		INIT_CWD: string;
		SHELL: string;
		TERM: string;
		VSCODE_PROCESS_TITLE: string;
		CPPFLAGS: string;
		HOMEBREW_REPOSITORY: string;
		TMPDIR: string;
		CURSOR_CLI: string;
		CONDA_PROMPT_MODIFIER: string;
		GROQ_API_KEY: string;
		TERM_PROGRAM_VERSION: string;
		npm_config_npm_globalconfig: string;
		NODE_OPTIONS: string;
		CURSOR_TRACE_ID: string;
		MallocNanoZone: string;
		APP_NAME: string;
		TERM_SESSION_ID: string;
		NO_COLOR: string;
		npm_config_registry: string;
		POWERLINE_COMMAND: string;
		NDK_HOME: string;
		NVM_DIR: string;
		POSH_SESSION_ID: string;
		USER: string;
		COMMAND_MODE: string;
		OPENAI_API_KEY: string;
		SUBSCRIPTION: string;
		npm_config_globalconfig: string;
		PNPM_SCRIPT_SRC_DIR: string;
		KUBECONFIG: string;
		SSH_AUTH_SOCK: string;
		PYENV_VIRTUALENV_DISABLE_PROMPT: string;
		__CF_USER_TEXT_ENCODING: string;
		RESOURCE_GROUP: string;
		npm_execpath: string;
		AWS_PROFILE: string;
		NOOCASH_AZURE: string;
		VIRTUAL_ENV_DISABLE_PROMPT: string;
		npm_config_frozen_lockfile: string;
		npm_config_verify_deps_before_run: string;
		OPHER_SUB_SUBID: string;
		PATH: string;
		npm_package_json: string;
		__CFBundleIdentifier: string;
		DOTNET_ROOT: string;
		ENV_NAME: string;
		PWD: string;
		npm_command: string;
		ELECTRON_NO_ATTACH_CONSOLE: string;
		JAVA_HOME: string;
		OPHER_SUBSCRIPTION_ID: string;
		VSCODE_HANDLES_UNCAUGHT_ERRORS: string;
		VSCODE_ESM_ENTRYPOINT: string;
		npm_config__jsr_registry: string;
		npm_lifecycle_event: string;
		LANG: string;
		POSH_SHELL_VERSION: string;
		CURSOR_AGENT: string;
		npm_package_name: string;
		CF_GH_TOKEN: string;
		CURSOR_CLI_MODE: string;
		NODE_PATH: string;
		CONVENTIONAL_GITHUB_RELEASER_TOKEN: string;
		XPC_FLAGS: string;
		CURSOR_EXTENSION_HOST_ROLE: string;
		RELUGG_SUBSCRIPTION_ID: string;
		TAURI_PRIVATE_KEY: string;
		FORCE_COLOR: string;
		JFX_PATH: string;
		VSCODE_NODE_OPTIONS: string;
		npm_config_node_gyp: string;
		XPC_SERVICE_NAME: string;
		npm_package_version: string;
		pnpm_config_verify_deps_before_run: string;
		GPG_TTY: string;
		HOME: string;
		SHLVL: string;
		ANTHROPIC_BASE_URL: string;
		OSTYPE: string;
		RACK_VPS_IP: string;
		VSCODE_NLS_CONFIG: string;
		HOMEBREW_PREFIX: string;
		CI: string;
		ANTHROPIC_AUTH_TOKEN: string;
		KERAS_BACKEND: string;
		LOGNAME: string;
		TAURI_KEY_PASSWORD: string;
		MODULAR_HOME: string;
		npm_lifecycle_script: string;
		ARIMMA_AZURE: string;
		LC_CTYPE: string;
		VSCODE_CODE_CACHE_PATH: string;
		VSCODE_IPC_HOOK: string;
		GITHUB_TOKEN: string;
		NVM_BIN: string;
		PKG_CONFIG_PATH: string;
		GITHUB_USERNAME: string;
		VSCODE_PID: string;
		npm_config_ignore_scripts: string;
		npm_config_user_agent: string;
		ANDROID_NDK_HOME: string;
		HOMEBREW_CELLAR: string;
		INFOPATH: string;
		JAX_PLATFORM: string;
		OSLogRateLimit: string;
		LOCATION: string;
		VSCODE_CWD: string;
		COLORTERM: string;
		GH_TOKEN: string;
		OLLAMA_HOST: string;
		npm_node_execpath: string;
		NODE_ENV: string;
		[key: `PUBLIC_${string}`]: undefined;
		[key: `${string}`]: string | undefined;
	}
}

/**
 * Similar to [`$env/dynamic/private`](https://svelte.dev/docs/kit/$env-dynamic-private), but only includes variables that begin with [`config.kit.env.publicPrefix`](https://svelte.dev/docs/kit/configuration#env) (which defaults to `PUBLIC_`), and can therefore safely be exposed to client-side code.
 * 
 * Note that public dynamic environment variables must all be sent from the server to the client, causing larger network requests — when possible, use `$env/static/public` instead.
 * 
 * ```ts
 * import { env } from '$env/dynamic/public';
 * console.log(env.PUBLIC_DEPLOYMENT_SPECIFIC_VARIABLE);
 * ```
 */
declare module '$env/dynamic/public' {
	export const env: {
		[key: `PUBLIC_${string}`]: string | undefined;
	}
}
