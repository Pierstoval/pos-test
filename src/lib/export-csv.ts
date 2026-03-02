import { APP_TARGET } from '$lib/api';

export async function downloadCsv(filename: string, csv: string): Promise<void> {
	if (APP_TARGET === 'tauri') {
		const { save } = await import('@tauri-apps/plugin-dialog');
		const { writeFile } = await import('@tauri-apps/plugin-fs');

		const path = await save({
			defaultPath: filename,
			filters: [{ name: 'CSV', extensions: ['csv'] }]
		});

		if (!path) {
			return;
		}

		const blob = new Blob([csv], { type: 'text/csv;charset=utf-8;' });
		await writeFile(path, blob.stream());
	} else {
		const blob = new Blob([csv], { type: 'text/csv;charset=utf-8;' });
		const url = URL.createObjectURL(blob);
		const a = document.createElement('a');
		a.href = url;
		a.download = filename;
		a.click();
		URL.revokeObjectURL(url);
	}
}
