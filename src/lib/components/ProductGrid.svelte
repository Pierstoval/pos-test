<script lang="ts">
	import type { Product, Category } from "$lib/types";
	import { formatPrice } from "$lib/utils/format";

	interface Props {
		products: Product[];
		categories: Category[];
		onProductClick: (product: Product) => void;
	}

	let { products, categories, onProductClick }: Props = $props();

	let categoryMap = $derived(
		Object.fromEntries(categories.map((c) => [c.id, c])),
	);

	let sortedProducts = $derived(
		[...products].sort((a, b) => {
			if (a.available !== b.available) return a.available ? -1 : 1;
			const catA = categoryMap[a.category_id]?.label ?? a.category_id;
			const catB = categoryMap[b.category_id]?.label ?? b.category_id;
			const catCmp = catA.localeCompare(catB);
			if (catCmp !== 0) return catCmp;
			return a.name.localeCompare(b.name);
		}),
	);

	let categoryColors = $derived(
		Object.fromEntries(categories.map((c) => [c.id, c.color])),
	);
</script>

<div class="product-grid">
	{#each sortedProducts as product (product.id)}
		<button
			class="product-btn"
			class:unavailable={!product.available}
			style="--cat-color: {categoryColors[product.category_id] ?? '#888'}"
			onclick={() => onProductClick(product)}
			disabled={!product.available}
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
		position: relative;
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
		overflow: hidden;
		transition: filter 0.15s, transform 0.1s;
		user-select: none;
		-webkit-tap-highlight-color: transparent;
	}

	.product-btn:hover:not(:disabled) {
		filter: brightness(1.1);
	}

	.product-btn:active:not(:disabled) {
		transform: scale(0.96);
	}

	.product-btn.unavailable {
		background: #999;
		border: 3px solid var(--cat-color);
		opacity: 0.7;
		cursor: not-allowed;
	}

	.product-btn.unavailable::after {
		content: "";
		position: absolute;
		inset: 0;
		background: linear-gradient(
			to bottom right,
			transparent calc(50% - 2px),
			#dc2626 calc(50% - 2px),
			#dc2626 calc(50% + 2px),
			transparent calc(50% + 2px)
		);
		pointer-events: none;
	}

	.product-price {
		font-size: 0.85rem;
		font-weight: 400;
		opacity: 0.9;
	}
</style>
