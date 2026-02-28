/** Converts integer cents to a formatted euro string, e.g. 150 → "1,50 €". */
export function formatPrice(cents: number): string {
	const euros = Math.floor(cents / 100);
	const remainder = Math.abs(cents % 100);
	return `${euros},${remainder.toString().padStart(2, "0")} €`;
}
