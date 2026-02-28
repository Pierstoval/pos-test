<script lang="ts">
	import type { CartItem } from "$lib/types";
	import { formatPrice } from "$lib/utils/format";

	interface Props {
		items: CartItem[];
		total: number;
		onConfirm: (paymentMethod: "cash" | "card") => void;
		onCancel: () => void;
	}

	let { items, total, onConfirm, onCancel }: Props = $props();

	let paymentMethod = $state<"cash" | "card" | null>(null);
	let cashReceived = $state("");
	let isSubmitting = $state(false);

	let cashReceivedCents = $derived.by(() => {
		const val = parseFloat(cashReceived.replace(",", "."));
		return isNaN(val) ? 0 : Math.round(val * 100);
	});

	let change = $derived(cashReceivedCents - total);

	let canConfirm = $derived.by(() => {
		if (isSubmitting) return false;
		if (!paymentMethod) return false;
		if (paymentMethod === "cash" && cashReceivedCents < total) return false;
		return true;
	});

	function selectPayment(method: "cash" | "card") {
		paymentMethod = method;
		if (method === "card") {
			cashReceived = "";
		}
	}

	async function handleConfirm() {
		if (!paymentMethod || !canConfirm) return;
		isSubmitting = true;
		onConfirm(paymentMethod);
	}
</script>

<!-- svelte-ignore a11y_no_static_element_interactions -->
<div class="modal-backdrop" onclick={onCancel} onkeydown={(e) => e.key === "Escape" && onCancel()}>
	<!-- svelte-ignore a11y_no_static_element_interactions -->
	<div class="modal" onclick={(e) => e.stopPropagation()}>
		<h2>Checkout</h2>

		<div class="summary">
			{#each items as item (item.product.id)}
				<div class="summary-row">
					<span>{item.quantity}x {item.product.name}</span>
					<span>{formatPrice(item.product.price * item.quantity)}</span>
				</div>
			{/each}
			<div class="summary-total">
				<span>Total</span>
				<span>{formatPrice(total)}</span>
			</div>
		</div>

		<div class="payment-methods">
			<button
				class="payment-btn"
				class:selected={paymentMethod === "cash"}
				onclick={() => selectPayment("cash")}
			>
				Cash
			</button>
			<button
				class="payment-btn"
				class:selected={paymentMethod === "card"}
				onclick={() => selectPayment("card")}
			>
				Card
			</button>
		</div>

		{#if paymentMethod === "cash"}
			<div class="cash-section">
				<label>
					Amount received
					<input
						type="text"
						inputmode="decimal"
						placeholder="0,00"
						bind:value={cashReceived}
					/>
				</label>
				{#if cashReceivedCents >= total}
					<div class="change">
						Change: <strong>{formatPrice(change)}</strong>
					</div>
				{/if}
			</div>
		{/if}

		<div class="modal-actions">
			<button class="btn btn-cancel" onclick={onCancel} disabled={isSubmitting}>Cancel</button>
			<button class="btn btn-confirm" onclick={handleConfirm} disabled={!canConfirm}>
				{isSubmitting ? "Submitting..." : "Confirm"}
			</button>
		</div>
	</div>
</div>

<style>
	.modal-backdrop {
		position: fixed;
		inset: 0;
		background: rgba(0, 0, 0, 0.5);
		display: flex;
		align-items: center;
		justify-content: center;
		z-index: 100;
	}

	.modal {
		background: #fff;
		border-radius: 16px;
		padding: 24px;
		width: 90%;
		max-width: 420px;
		max-height: 90vh;
		overflow-y: auto;
	}

	h2 {
		margin: 0 0 16px;
		font-size: 1.3rem;
	}

	.summary {
		margin-bottom: 16px;
	}

	.summary-row {
		display: flex;
		justify-content: space-between;
		padding: 4px 0;
		font-size: 0.95rem;
	}

	.summary-total {
		display: flex;
		justify-content: space-between;
		padding: 8px 0 0;
		margin-top: 8px;
		border-top: 2px solid #e0e0e0;
		font-size: 1.2rem;
		font-weight: 700;
	}

	.payment-methods {
		display: flex;
		gap: 8px;
		margin-bottom: 16px;
	}

	.payment-btn {
		flex: 1;
		padding: 14px;
		border: 2px solid #e0e0e0;
		border-radius: 10px;
		background: #f5f5f5;
		font-size: 1rem;
		font-weight: 600;
		cursor: pointer;
		min-height: 48px;
	}

	.payment-btn.selected {
		border-color: #3b82f6;
		background: #eff6ff;
		color: #1d4ed8;
	}

	.cash-section {
		margin-bottom: 16px;
	}

	.cash-section label {
		display: flex;
		flex-direction: column;
		gap: 4px;
		font-weight: 600;
		font-size: 0.95rem;
	}

	.cash-section input {
		padding: 10px 12px;
		border: 1px solid #ccc;
		border-radius: 8px;
		font-size: 1.2rem;
		text-align: right;
	}

	.change {
		margin-top: 8px;
		font-size: 1.1rem;
		color: #16a34a;
	}

	.modal-actions {
		display: flex;
		gap: 8px;
	}

	.btn {
		flex: 1;
		padding: 14px;
		border: none;
		border-radius: 10px;
		font-size: 1rem;
		font-weight: 600;
		cursor: pointer;
		min-height: 48px;
	}

	.btn:disabled {
		opacity: 0.4;
		cursor: not-allowed;
	}

	.btn-cancel {
		background: #e0e0e0;
		color: #333;
	}

	.btn-confirm {
		background: #16a34a;
		color: #fff;
	}

	.btn-confirm:not(:disabled):hover {
		background: #15803d;
	}

	@media (prefers-color-scheme: dark) {
		.modal {
			background: #1e1e1e;
			color: #f6f6f6;
		}

		.summary-total {
			border-top-color: #444;
		}

		.payment-btn {
			background: #333;
			border-color: #555;
			color: #ddd;
		}

		.payment-btn.selected {
			border-color: #3b82f6;
			background: #1e3a5f;
			color: #93c5fd;
		}

		.cash-section input {
			background: #333;
			border-color: #555;
			color: #f6f6f6;
		}

		.btn-cancel {
			background: #333;
			color: #ddd;
		}
	}
</style>
