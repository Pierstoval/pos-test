import { invoke } from "@tauri-apps/api/core";

type ApiBackend = "tauri" | "http";

const BACKEND: ApiBackend = "tauri";

export async function api_call<T = void>(
	command: string,
	params: Record<string, unknown> = {},
): Promise<T> {
	switch (BACKEND) {
		case "tauri":
			return invoke<T>(command, params);
		case "http":
			// Future: map command names to HTTP endpoints and call fetch().
			throw new Error("HTTP backend not yet implemented");
	}
}
