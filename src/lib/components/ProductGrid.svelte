<script lang="ts">
	import type { Product } from "$lib/types";
	import { formatPrice } from "$lib/utils/format";

	interface Props {
		products: Product[];
		onProductClick: (product: Product) => void;
	}

	let { products, onProductClick }: Props = $props();

	let availableProducts = $derived(products.filter((p) => p.available));

	const categoryColors: Record<string, string> = {
		snack: "#e8a735",
		soft_drink: "#3b82f6",
		alcohol: "#8b5cf6",
		sweets: "#e84393",
	};
</script>

<div class="product-grid">
	{#each availableProducts as product (product.id)}
		<button
			class="product-btn"
			style="--cat-color: {categoryColors[product.category] ?? '#888'}"
			onclick={() => onProductClick(product)}
		>
			<span class="product-name">{product.name}</span>
			<span class="product-price">{formatPrice(product.price)}</span>
		</button>
	{/each}
</div>

<style>
	.product-grid {
		display: grid;
		grid-template-columns: repeat(auto-fill, minmax(140px, 1fr));
		gap: 12px;
		padding: 16px;
	}

	.product-btn {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		gap: 6px;
		min-height: 80px;
		padding: 12px 8px;
		border: none;
		border-radius: 12px;
		background: var(--cat-color);
		color: #fff;
		font-size: 1rem;
		font-weight: 600;
		cursor: pointer;
		transition: filter 0.15s, transform 0.1s;
		user-select: none;
		-webkit-tap-highlight-color: transparent;
	}

	.product-btn:hover {
		filter: brightness(1.1);
	}

	.product-btn:active {
		transform: scale(0.96);
	}

	.product-price {
		font-size: 0.85rem;
		font-weight: 400;
		opacity: 0.9;
	}
</style>
