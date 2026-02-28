<script lang="ts">
	import type { CartItem } from "$lib/types";
	import { formatPrice } from "$lib/utils/format";

	interface Props {
		items: CartItem[];
		onIncrease: (productId: string) => void;
		onDecrease: (productId: string) => void;
		onCheckout: () => void;
		onClear: () => void;
	}

	let { items, onIncrease, onDecrease, onCheckout, onClear }: Props = $props();

	let total = $derived(items.reduce((sum, i) => sum + i.product.price * i.quantity, 0));
	let isEmpty = $derived(items.length === 0);
</script>

<aside class="order-panel">
	<h2>Current Order</h2>

	{#if isEmpty}
		<p class="empty-msg">Tap a product to add it.</p>
	{:else}
		<ul class="item-list">
			{#each items as item (item.product.id)}
				<li class="item-row">
					<div class="item-info">
						<span class="item-name">{item.product.name}</span>
						<span class="item-unit-price">{formatPrice(item.product.price)}</span>
					</div>
					<div class="item-controls">
						<button class="qty-btn" onclick={() => onDecrease(item.product.id)}>-</button>
						<span class="qty">{item.quantity}</span>
						<button class="qty-btn" onclick={() => onIncrease(item.product.id)}>+</button>
					</div>
					<span class="line-total">{formatPrice(item.product.price * item.quantity)}</span>
				</li>
			{/each}
		</ul>
	{/if}

	<div class="panel-footer">
		<div class="total-row">
			<span>Total</span>
			<span class="total-amount">{formatPrice(total)}</span>
		</div>
		<div class="action-buttons">
			<button class="btn btn-clear" disabled={isEmpty} onclick={onClear}>Clear</button>
			<button class="btn btn-checkout" disabled={isEmpty} onclick={onCheckout}>Checkout</button>
		</div>
	</div>
</aside>

<style>
	.order-panel {
		display: flex;
		flex-direction: column;
		height: 100%;
		background: #fff;
		border-left: 1px solid #e0e0e0;
	}

	h2 {
		margin: 0;
		padding: 16px;
		font-size: 1.2rem;
		border-bottom: 1px solid #e0e0e0;
	}

	.empty-msg {
		padding: 24px 16px;
		color: #888;
		text-align: center;
	}

	.item-list {
		list-style: none;
		margin: 0;
		padding: 0;
		flex: 1;
		overflow-y: auto;
	}

	.item-row {
		display: grid;
		grid-template-columns: 1fr auto auto;
		align-items: center;
		gap: 8px;
		padding: 10px 16px;
		border-bottom: 1px solid #f0f0f0;
	}

	.item-info {
		display: flex;
		flex-direction: column;
	}

	.item-name {
		font-weight: 600;
		font-size: 0.95rem;
	}

	.item-unit-price {
		font-size: 0.8rem;
		color: #888;
	}

	.item-controls {
		display: flex;
		align-items: center;
		gap: 6px;
	}

	.qty-btn {
		width: 32px;
		height: 32px;
		border: 1px solid #ccc;
		border-radius: 6px;
		background: #f5f5f5;
		font-size: 1.1rem;
		font-weight: 700;
		cursor: pointer;
		display: flex;
		align-items: center;
		justify-content: center;
		padding: 0;
	}

	.qty-btn:active {
		background: #e0e0e0;
	}

	.qty {
		min-width: 24px;
		text-align: center;
		font-weight: 600;
	}

	.line-total {
		font-weight: 600;
		text-align: right;
		min-width: 60px;
	}

	.panel-footer {
		border-top: 2px solid #e0e0e0;
		padding: 16px;
	}

	.total-row {
		display: flex;
		justify-content: space-between;
		font-size: 1.3rem;
		font-weight: 700;
		margin-bottom: 12px;
	}

	.action-buttons {
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

	.btn-clear {
		background: #e0e0e0;
		color: #333;
	}

	.btn-checkout {
		background: #16a34a;
		color: #fff;
	}

	.btn-checkout:not(:disabled):hover {
		background: #15803d;
	}

	@media (prefers-color-scheme: dark) {
		.order-panel {
			background: #1e1e1e;
			border-left-color: #444;
		}

		h2 {
			border-bottom-color: #444;
		}

		.item-row {
			border-bottom-color: #333;
		}

		.qty-btn {
			background: #333;
			border-color: #555;
			color: #fff;
		}

		.qty-btn:active {
			background: #444;
		}

		.panel-footer {
			border-top-color: #444;
		}

		.btn-clear {
			background: #333;
			color: #ddd;
		}
	}
</style>
