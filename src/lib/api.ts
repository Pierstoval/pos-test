import { invoke } from '@tauri-apps/api/core';

export type AppTarget = 'tauri' | 'web' | 'static';

declare const __APP_TARGET__: string;
export const APP_TARGET = __APP_TARGET__ as AppTarget;

export async function api_call<T = void>(
	command: string,
	params: Record<string, unknown> = {}
): Promise<T> {
	switch (APP_TARGET) {
		case 'tauri':
			return invoke<T>(command, params);
		case 'web':
			throw new Error('Web backend not yet implemented');
		case 'static':
			throw new Error('Static backend not yet implemented');
	}
}
