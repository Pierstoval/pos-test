import { writable, derived } from "svelte/store";
import fr from "./locales/fr.json";
import en from "./locales/en.json";

type Messages = Record<string, unknown>;

const locales: Record<string, Messages> = { fr, en };

export const locale = writable<string>("fr");

function resolve(obj: unknown, path: string): string {
	const keys = path.split(".");
	let current: unknown = obj;
	for (const key of keys) {
		if (current === null || current === undefined || typeof current !== "object") {
			return path;
		}
		current = (current as Record<string, unknown>)[key];
	}
	return typeof current === "string" ? current : path;
}

export const t = derived(locale, ($locale) => {
	const messages = locales[$locale] ?? locales.fr;
	const fallback = locales.fr;

	return (key: string, params?: Record<string, string>): string => {
		let value = resolve(messages, key);
		if (value === key && messages !== fallback) {
			value = resolve(fallback, key);
		}
		if (params) {
			for (const [k, v] of Object.entries(params)) {
				value = value.replaceAll(`{${k}}`, v);
			}
		}
		return value;
	};
});
