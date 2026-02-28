<script lang="ts">
	import type { Product, Category } from "$lib/types";
	import { t } from "$lib/i18n";

	interface Props {
		product: Product | null;
		categories: Category[];
		onSave: (data: { name: string; price: number; category_id: string; available: boolean }) => void;
		onCancel: () => void;
	}

	let { product, categories, onSave, onCancel }: Props = $props();

	let name = $state(product?.name ?? "");
	let priceInput = $state(product ? (product.price / 100).toFixed(2).replace(".", ",") : "");
	let category_id = $state(product?.category_id ?? categories[0]?.id ?? "");
	let available = $state(product?.available ?? true);
	let isSubmitting = $state(false);

	let priceCents = $derived.by(() => {
		const val = parseFloat(priceInput.replace(",", "."));
		return isNaN(val) ? 0 : Math.round(val * 100);
	});

	let canSave = $derived(
		!isSubmitting && name.trim().length > 0 && priceCents > 0 && category_id.length > 0,
	);

	function handleSubmit() {
		if (!canSave) return;
		isSubmitting = true;
		onSave({ name: name.trim(), price: priceCents, category_id, available });
	}
</script>

<!-- svelte-ignore a11y_no_static_element_interactions -->
<div class="modal-backdrop" onclick={onCancel} onkeydown={(e) => e.key === "Escape" && onCancel()}>
	<!-- svelte-ignore a11y_no_static_element_interactions -->
	<div class="modal" onclick={(e) => e.stopPropagation()}>
		<h2>{product ? $t("productForm.editTitle") : $t("productForm.newTitle")}</h2>

		<div class="form-field">
			<label for="product-name">{$t("productForm.name")}</label>
			<input id="product-name" type="text" bind:value={name} placeholder={$t("productForm.namePlaceholder")} />
		</div>

		<div class="form-field">
			<label for="product-price">{$t("productForm.priceLabel")}</label>
			<input
				id="product-price"
				type="text"
				inputmode="decimal"
				bind:value={priceInput}
				placeholder="0,00"
			/>
		</div>

		<fieldset class="form-field radio-group">
			<legend>{$t("productForm.category")}</legend>
			{#each categories as cat (cat.id)}
				<label>
					<input type="radio" name="category" value={cat.id} bind:group={category_id} />
					{cat.label}
				</label>
			{/each}
		</fieldset>

		{#if product}
			<div class="form-field checkbox-field">
				<label>
					<input type="checkbox" bind:checked={available} />
					{$t("productForm.available")}
				</label>
			</div>
		{/if}

		<div class="modal-actions">
			<button class="btn btn-cancel" onclick={onCancel} disabled={isSubmitting}>{$t("productForm.cancel")}</button>
			<button class="btn btn-confirm" onclick={handleSubmit} disabled={!canSave}>
				{isSubmitting ? $t("productForm.saving") : $t("productForm.save")}
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

	.form-field {
		margin-bottom: 14px;
	}

	.form-field label {
		display: block;
		font-weight: 600;
		font-size: 0.95rem;
		margin-bottom: 4px;
	}

	.form-field input[type="text"] {
		width: 100%;
		padding: 10px 12px;
		border: 1px solid #ccc;
		border-radius: 8px;
		font-size: 1rem;
		box-sizing: border-box;
	}

	.radio-group {
		border: none;
		padding: 0;
		display: flex;
		flex-wrap: wrap;
		gap: 8px;
	}

	.radio-group legend {
		width: 100%;
		font-weight: 600;
		font-size: 0.95rem;
		margin-bottom: 6px;
	}

	.radio-group label {
		display: inline-block;
		padding: 8px 16px;
		border: 2px solid #e0e0e0;
		border-radius: 8px;
		background: #f5f5f5;
		font-weight: 600;
		font-size: 0.9rem;
		cursor: pointer;
		transition: all 0.15s ease;
	}

	.radio-group label:hover {
		border-color: #93c5fd;
		background: #f0f7ff;
	}

	.radio-group label:has(input:checked) {
		border-color: #3b82f6;
		background: #eff6ff;
		color: #1d4ed8;
	}

	.radio-group input[type="radio"] {
		position: absolute;
		opacity: 0;
		pointer-events: none;
	}

	.checkbox-field label {
		display: flex;
		align-items: center;
		gap: 8px;
		cursor: pointer;
	}

	.checkbox-field input[type="checkbox"] {
		width: 18px;
		height: 18px;
	}

	.modal-actions {
		display: flex;
		gap: 8px;
		margin-top: 16px;
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
		background: #3b82f6;
		color: #fff;
	}

	.btn-confirm:not(:disabled):hover {
		background: #2563eb;
	}

	@media (prefers-color-scheme: dark) {
		.modal {
			background: #1e1e1e;
			color: #f6f6f6;
		}

		.form-field input[type="text"] {
			background: #333;
			border-color: #555;
			color: #f6f6f6;
		}

		.radio-group label {
			background: #333;
			border-color: #555;
			color: #ddd;
		}

		.radio-group label:hover {
			border-color: #60a5fa;
			background: #1e3a5f;
		}

		.radio-group label:has(input:checked) {
			border-color: #3b82f6;
			background: #1e3a5f;
			color: #93c5fd;
		}

		.btn-cancel {
			background: #333;
			color: #ddd;
		}
	}
</style>
