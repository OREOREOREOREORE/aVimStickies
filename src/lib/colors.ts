import type { Note } from "./types";

export const COLORS: Record<string, string> = {
  yellow: "#fdf6d8",
  blue: "#d8e6fd",
  green: "#d8f5d8",
  pink: "#fdd8e6",
  purple: "#e8d8fd",
  gray: "#e6e6e6",
};

export const COLOR_ORDER = ["yellow", "blue", "green", "pink", "purple", "gray"];

export const DEFAULT_COLOR = "yellow";

export function colorOf(note: Pick<Note, "color"> | null | undefined): string {
  return COLORS[note?.color ?? DEFAULT_COLOR] ?? COLORS[DEFAULT_COLOR];
}

export function nextColor(current: string): string {
  const idx = COLOR_ORDER.indexOf(current);
  return COLOR_ORDER[(idx + 1) % COLOR_ORDER.length];
}
