const { defineConfig } = require('eslint/config');
const js = require('@eslint/js');
const tseslint = require('typescript-eslint');
const svelte = require('eslint-plugin-svelte');
const svelteParser = require('svelte-eslint-parser');
const prettierConfig = require('eslint-config-prettier');
const globals = require('globals');

module.exports = defineConfig([
	{
		ignores: ['.svelte-kit/', 'build/', 'node_modules/', 'playwright-report/', 'test-results/']
	},
	js.configs.recommended,
	...tseslint.configs.recommended,
	...svelte.configs['flat/recommended'],
	{
		languageOptions: {
			parser: tseslint.parser,
			parserOptions: {
				ecmaVersion: 2020,
				sourceType: 'module',
				extraFileExtensions: ['.svelte']
			},
			globals: {
				...globals.browser,
				...globals.node
			}
		}
	},
	{
		files: ['**/*.svelte'],
		languageOptions: {
			parser: svelteParser,
			parserOptions: {
				parser: tseslint.parser
			}
		}
	},
	prettierConfig,
	{
		files: ['eslint.config.cjs'],
		rules: { '@typescript-eslint/no-require-imports': 'off' }
	}
]);
