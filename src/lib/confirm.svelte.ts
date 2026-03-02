type ConfirmState = {
	isOpen: boolean;
	message: string;
	resolve: ((value: boolean) => void) | null;
};

export const confirmState: ConfirmState = $state({
	isOpen: false,
	message: '',
	resolve: null
});

export function confirm(message: string): Promise<boolean> {
	return new Promise((resolve) => {
		confirmState.message = message;
		confirmState.resolve = resolve;
		confirmState.isOpen = true;
	});
}

export function _respond(value: boolean) {
	const resolve = confirmState.resolve;
	confirmState.isOpen = false;
	confirmState.message = '';
	confirmState.resolve = null;
	resolve?.(value);
}
