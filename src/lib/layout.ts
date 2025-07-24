import type { KeyInfo } from "$lib/components/keyboard/Key.svelte";
import { icons as lucide } from "@iconify-json/lucide";
import type { IconifyJSON } from "@iconify/svelte";
import { getIconData } from "@iconify/utils";

type KeyInfoWithEvent = Omit<KeyInfo, "active"> & {
  event: string;
};

function icon(
  iconSet: IconifyJSON,
  iconName: string,
  event: string,
  label?: string,
  width = 1,
): KeyInfoWithEvent {
  return { icon: getIconData(iconSet, iconName)!, event, label, width };
}
const label = (label: string, event: string, width = 1): KeyInfoWithEvent => ({
  label,
  event,
  width,
});
const split = (
  letters: string,
  prefix: string = "Key",
  width = 1,
): KeyInfoWithEvent[] =>
  letters
    .split("")
    .map((c) => label(c.toUpperCase(), `${prefix}${c.toUpperCase()}`, width));

const layout: KeyInfoWithEvent[][] = [
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
