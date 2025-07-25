import js from "@eslint/js";
import simpleImportSort from "eslint-plugin-simple-import-sort";
import svelte from "eslint-plugin-svelte";
import unusedImports from "eslint-plugin-unused-imports";
import globals from "globals";
import ts from "typescript-eslint";

import svelteConfig from "./svelte.config";

export default ts.config(
	// js/ts
	js.configs.recommended,
	...ts.configs.recommended,
	// svelte
	svelte.configs.recommended,
	{
		languageOptions: {
			globals: {
				...globals.browser,
			},
		},
	},
	{
		files: ["**/*.svelte", "**/*.svelte.js"],
		languageOptions: {
			parserOptions: {
				extraFileExtensions: [".svelte"],
				parser: ts.parser,
				projectService: true,
				svelteConfig,
			},
		},
	},
	// imports
	{
		plugins: {
			"simple-import-sort": simpleImportSort,
			"sort-keys-fix": await import("eslint-plugin-sort-keys"),
			"unused-imports": unusedImports,
		},
		rules: {
			"simple-import-sort/exports": "error",
			"simple-import-sort/imports": "error",
			"sort-keys-fix/sort-keys-fix": "error",
			"unused-imports/no-unused-imports": "error",
		},
	},
);
