import type { Config } from "prettier";

const plugins = [
  "svelte",
  "organize-imports",
  "sort-json",
  "tailwindcss", // must be last
];

export default {
  plugins: plugins.map((p) => `prettier-plugin-${p}`),
} satisfies Config;
