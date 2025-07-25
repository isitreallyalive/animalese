import type { Config } from "prettier";

const plugins = [
	"svelte",
	"organize-imports",
	"sort-json",
	"tailwindcss", // must be last
];

export default {
	arrowParens: "avoid",
	singleQuote: false,
	bracketSpacing: true,
	useTabs: true,
	plugins: plugins.map(p => `prettier-plugin-${p}`),
} satisfies Config;
