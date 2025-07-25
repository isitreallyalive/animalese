import { icons as lucide } from "@iconify-json/lucide";
import type { IconifyJSON } from "@iconify/svelte";
import { getIconData } from "@iconify/utils";

import type { Props } from "$lib/components/keyboard/Key.svelte";

type KeyInfo = Omit<Props, "active">;

function icon(
	iconSet: IconifyJSON,
	iconName: string,
	key: string,
	label?: string,
	width = 1,
): KeyInfo {
	return {
		icon: getIconData(iconSet, iconName)!,
		key,
		label,
		width,
	};
}

function label(label: string, key: string, width = 1): KeyInfo {
	return { key, label, width };
}

function split(letters: string, prefix: string = "Key", width = 1): KeyInfo[] {
	return letters
		.split("")
		.map(c => label(c.toUpperCase(), `${prefix}${c.toUpperCase()}`, width));
}

const layout: KeyInfo[][] = [
	[
		label("`", "Unknown(223)"),
		...split("1234567890", "Num"),
		label("-", "Minus"),
		label("=", "Equal"),
		icon(lucide, "arrow-left", "Backspace", "Backspace", 3.25),
	],
	[
		icon(lucide, "arrow-right-to-line", "Tab", "Tab", 1.75),
		...split("qwertyuiop"),
		label("[", "LeftBracket"),
		label("]", "RightBracket"),
		label("\\", "BackSlash", 1.75),
	],
	[
		label("Caps", "CapsLock", 1.75),
		...split("asdfghjkl"),
		label(";", "SemiColon"),
		label("'", "BackQuote"),
		icon(lucide, "corner-down-left", "Return", "Enter", 2.25),
	],
	[
		icon(lucide, "arrow-big-up-dash", "ShiftLeft", "Shift", 2.25),
		...split("zxcvbnm"),
		label(",", "Comma"),
		label(".", "Dot"),
		label("/", "Slash"),
		icon(lucide, "arrow-big-up-dash", "ShiftRight", "Shift", 2.25),
	],
	[
		icon(lucide, "command", "ControlLeft", "Ctrl", 1.75),
		label("Alt", "Alt", 1.25),
		label(" ", "Space", 9.5),
		label("Alt", "AltGr", 1.25),
		icon(lucide, "command", "ControlRight", "Ctrl", 1.75),
	],
];

export default layout;
