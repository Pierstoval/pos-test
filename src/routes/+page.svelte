<script lang="ts">
	import { onMount } from "svelte";
	import { invoke } from "@tauri-apps/api/core";
	import type { Product, Category, CartItem, OrderWithItems, CreateOrderPayload } from "$lib/types";
	import ProductGrid from "$lib/components/ProductGrid.svelte";
	import OrderPanel from "$lib/components/OrderPanel.svelte";
	import CheckoutModal from "$lib/components/CheckoutModal.svelte";
	import { t } from "$lib/i18n";

	let products = $state<Product[]>([]);
	let categories = $state<Category[]>([]);
	let cart = $state<CartItem[]>([]);
	let isCheckoutOpen = $state(false);
	let isLoading = $state(true);
	let error = $state<string | null>(null);

	let cartTotal = $derived(cart.reduce((sum, i) => sum + i.product.price * i.quantity, 0));

	onMount(async () => {
		try {
			[products, categories] = await Promise.all([
				invoke<Product[]>("list_products"),
				invoke<Category[]>("list_categories"),
			]);
		} catch (e) {
			error = $t("sales.loadError", { error: String(e) });
		} finally {
			isLoading = false;
		}
	});

	function addToCart(product: Product) {
		const existing = cart.find((i) => i.product.id === product.id);
		if (existing) {
			cart = cart.map((i) =>
				i.product.id === product.id ? { ...i, quantity: i.quantity + 1 } : i,
			);
		} else {
			cart = [...cart, { product, quantity: 1 }];
		}
	}

	function increaseQuantity(productId: string) {
		cart = cart.map((i) =>
			i.product.id === productId ? { ...i, quantity: i.quantity + 1 } : i,
		);
	}

	function decreaseQuantity(productId: string) {
		cart = cart
			.map((i) => (i.product.id === productId ? { ...i, quantity: i.quantity - 1 } : i))
			.filter((i) => i.quantity > 0);
	}

	function clearCart() {
		cart = [];
	}

	async function submitOrder(paymentMethod: "cash" | "card") {
		const payload: CreateOrderPayload = {
			items: cart.map((i) => ({
				product_id: i.product.id,
				product_name: i.product.name,
				unit_price: i.product.price,
				quantity: i.quantity,
			})),
			payment_method: paymentMethod,
		};

		try {
			await invoke<OrderWithItems>("create_order", { payload });
			cart = [];
			isCheckoutOpen = false;
		} catch (e) {
			error = $t("sales.orderError", { error: String(e) });
			isCheckoutOpen = false;
		}
	}
</script>

<div class="sales-screen">
	{#if isLoading}
		<div class="status-msg">{$t("sales.loading")}</div>
	{:else if error}
		<div class="status-msg error">{error}</div>
	{:else}
		<main class="product-area">
			<ProductGrid {products} {categories} onProductClick={addToCart} />
		</main>
		<div class="sidebar">
			<OrderPanel
				items={cart}
				onIncrease={increaseQuantity}
				onDecrease={decreaseQuantity}
				onCheckout={() => (isCheckoutOpen = true)}
				onClear={clearCart}
			/>
		</div>
	{/if}
</div>

{#if isCheckoutOpen}
	<CheckoutModal
		items={cart}
		total={cartTotal}
		onConfirm={submitOrder}
		onCancel={() => (isCheckoutOpen = false)}
	/>
{/if}

<style>
	.sales-screen {
		display: grid;
		grid-template-columns: 1fr 360px;
		height: calc(100vh - 48px);
	}

	.product-area {
		overflow-y: auto;
	}

	.sidebar {
		height: calc(100vh - 48px);
		overflow: hidden;
	}

	.status-msg {
		grid-column: 1 / -1;
		display: flex;
		align-items: center;
		justify-content: center;
		height: calc(100vh - 48px);
		font-size: 1.2rem;
		color: #888;
	}

	.status-msg.error {
		color: #dc2626;
	}

	@media (max-width: 700px) {
		.sales-screen {
			grid-template-columns: 1fr;
			grid-template-rows: 1fr auto;
			height: calc(100vh - 48px);
		}

		.product-area {
			overflow-y: auto;
		}

		.sidebar {
			height: auto;
			max-height: 40vh;
			overflow-y: auto;
			border-top: 1px solid #e0e0e0;
		}
	}

	@media (prefers-color-scheme: dark) {
		.sidebar {
			border-left-color: #444;
		}
	}
</style>
