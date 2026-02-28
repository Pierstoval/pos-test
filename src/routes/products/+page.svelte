<script lang="ts">
	import { onMount } from "svelte";
	import { invoke } from "@tauri-apps/api/core";
	import type { Product, CreateProductPayload, UpdateProductPayload } from "$lib/types";
	import { formatPrice } from "$lib/utils/format";
	import ProductFormModal from "$lib/components/ProductFormModal.svelte";

	let products = $state<Product[]>([]);
	let editingProduct = $state<Product | null>(null);
	let isFormOpen = $state(false);
	let isLoading = $state(true);
	let error = $state<string | null>(null);

	onMount(async () => {
		await loadProducts();
	});

	async function loadProducts() {
		isLoading = true;
		error = null;
		try {
			products = await invoke<Product[]>("list_products");
		} catch (e) {
			error = `Failed to load products: ${e}`;
		} finally {
			isLoading = false;
		}
	}

	function openCreate() {
		editingProduct = null;
		isFormOpen = true;
	}

	function openEdit(product: Product) {
		editingProduct = product;
		isFormOpen = true;
	}

	function closeForm() {
		isFormOpen = false;
		editingProduct = null;
	}

	async function handleSave(data: {
		name: string;
		price: number;
		category: string;
		available: boolean;
	}) {
		try {
			if (editingProduct) {
				const payload: UpdateProductPayload = {
					id: editingProduct.id,
					name: data.name,
					price: data.price,
					category: data.category,
					available: data.available,
				};
				await invoke<Product>("update_product", { payload });
			} else {
				const payload: CreateProductPayload = {
					name: data.name,
					price: data.price,
					category: data.category,
				};
				await invoke<Product>("create_product", { payload });
			}
			closeForm();
			await loadProducts();
		} catch (e) {
			error = `Failed to save product: ${e}`;
			closeForm();
		}
	}

	async function toggleAvailability(productId: string) {
		try {
			const newAvailable = await invoke<boolean>("toggle_product_availability", { productId });
			products = products.map((p) =>
				p.id === productId ? { ...p, available: newAvailable } : p,
			);
		} catch (e) {
			error = `Failed to toggle availability: ${e}`;
		}
	}

	function categoryLabel(cat: string): string {
		switch (cat) {
			case "snack":
				return "🥪 Snack";
			case "soft_drink":
				return "🥤 Soft drink";
			case "alcohol":
				return "🍺 Alcohol";
			case "sweets":
				return "🍫 Sweets";
			default:
				return cat;
		}
	}
</script>

<div class="products-page">
	<div class="header">
		<h1>Products</h1>
		<button class="btn btn-add" onclick={openCreate}>+ Add Product</button>
	</div>

	{#if isLoading}
		<div class="status-msg">Loading products...</div>
	{:else if error}
		<div class="status-msg error">{error}</div>
	{:else if products.length === 0}
		<div class="status-msg">No products yet. Add your first product!</div>
	{:else}
		<div class="table-wrapper">
			<table>
				<thead>
					<tr>
						<th>Name</th>
						<th>Price</th>
						<th>Category</th>
						<th>Available</th>
						<th>Actions</th>
					</tr>
				</thead>
				<tbody>
					{#each products as product (product.id)}
						<tr class:unavailable={!product.available}>
							<td>{product.name}</td>
							<td class="price">{formatPrice(product.price)}</td>
							<td>
								<span class="badge badge-{product.category}"
									>{categoryLabel(product.category)}</span
								>
							</td>
							<td>
								<button
									class="toggle-btn"
									class:toggle-on={product.available}
									class:toggle-off={!product.available}
									onclick={() => toggleAvailability(product.id)}
								>
									{product.available ? "Yes" : "No"}
								</button>
							</td>
							<td>
								<button class="btn btn-edit" onclick={() => openEdit(product)}>
									Edit
								</button>
							</td>
						</tr>
					{/each}
				</tbody>
			</table>
		</div>
	{/if}
</div>

{#if isFormOpen}
	<ProductFormModal product={editingProduct} onSave={handleSave} onCancel={closeForm} />
{/if}

<style>
	.products-page {
		padding: 0 24px 24px;
		max-width: 900px;
		margin: 0 auto;
	}

	.header {
		display: flex;
		align-items: center;
		justify-content: space-between;
		padding: 24px 0 16px;
	}

	h1 {
		margin: 0;
		font-size: 1.5rem;
		font-weight: 600;
	}

	.btn-add {
		padding: 10px 20px;
		background: #3b82f6;
		color: #fff;
		border: none;
		border-radius: 8px;
		font-size: 0.95rem;
		font-weight: 600;
		cursor: pointer;
		min-height: 44px;
	}

	.btn-add:hover {
		background: #2563eb;
	}

	.table-wrapper {
		overflow-x: auto;
	}

	table {
		width: 100%;
		border-collapse: collapse;
	}

	th {
		text-align: left;
		padding: 10px 12px;
		border-bottom: 2px solid #e0e0e0;
		font-size: 0.85rem;
		text-transform: uppercase;
		color: #888;
		font-weight: 600;
	}

	td {
		padding: 12px;
		border-bottom: 1px solid #eee;
		font-size: 0.95rem;
	}

	tr.unavailable td {
		opacity: 0.5;
	}

	.price {
		font-variant-numeric: tabular-nums;
		white-space: nowrap;
	}

	.badge {
		display: inline-block;
		padding: 3px 10px;
		border-radius: 12px;
		font-size: 0.8rem;
		font-weight: 600;
	}

	.badge-snack {
		background: #fef3c7;
		color: #92400e;
	}

	.badge-soft_drink {
		background: #dbeafe;
		color: #1e40af;
	}

	.badge-alcohol {
		background: #ede9fe;
		color: #5b21b6;
	}

	.badge-sweets {
		background: #fce7f3;
		color: #9d174d;
	}

	.toggle-btn {
		padding: 4px 14px;
		border: 1px solid #ccc;
		border-radius: 6px;
		font-size: 0.85rem;
		font-weight: 600;
		cursor: pointer;
		min-height: 32px;
		background: #f5f5f5;
	}

	.toggle-on {
		background: #dcfce7;
		color: #166534;
		border-color: #86efac;
	}

	.toggle-off {
		background: #fee2e2;
		color: #991b1b;
		border-color: #fca5a5;
	}

	.btn-edit {
		padding: 4px 14px;
		background: #f5f5f5;
		border: 1px solid #ccc;
		border-radius: 6px;
		font-size: 0.85rem;
		font-weight: 600;
		cursor: pointer;
		min-height: 32px;
	}

	.btn-edit:hover {
		background: #e5e5e5;
	}

	.status-msg {
		display: flex;
		align-items: center;
		justify-content: center;
		height: 200px;
		font-size: 1.1rem;
		color: #888;
	}

	.status-msg.error {
		color: #dc2626;
	}

	@media (prefers-color-scheme: dark) {
		th {
			border-bottom-color: #444;
			color: #aaa;
		}

		td {
			border-bottom-color: #333;
		}

		.badge-snack {
			background: #78350f;
			color: #fef3c7;
		}

		.badge-soft_drink {
			background: #1e3a5f;
			color: #93c5fd;
		}

		.badge-alcohol {
			background: #3b1f6e;
			color: #c4b5fd;
		}

		.badge-sweets {
			background: #831843;
			color: #fbcfe8;
		}

		.toggle-btn {
			background: #333;
			border-color: #555;
		}

		.toggle-on {
			background: #14532d;
			color: #86efac;
			border-color: #166534;
		}

		.toggle-off {
			background: #7f1d1d;
			color: #fca5a5;
			border-color: #991b1b;
		}

		.btn-edit {
			background: #333;
			border-color: #555;
			color: #ddd;
		}

		.btn-edit:hover {
			background: #444;
		}
	}
</style>
