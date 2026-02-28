<script lang="ts">
	import type { Category } from "$lib/types";
	import { t } from "$lib/i18n";

	interface Props {
		category: Category | null;
		onSave: (data: { id: string; label: string; color: string }) => void;
		onCancel: () => void;
	}

	let { category, onSave, onCancel }: Props = $props();

	let id = $state(category?.id ?? "");
	let label = $state(category?.label ?? "");
	let color = $state(category?.color ?? "#6b7280");
	let isSubmitting = $state(false);

	let isEditing = $derived(category !== null);

	function sanitizeId(value: string): string {
		return value
			.toLowerCase()
			.replace(/[^a-z0-9._-]/g, "")
			.replace(/[._-]{2,}/g, (m) => m[0]);
	}

	function handleIdInput(e: Event) {
		const input = e.target as HTMLInputElement;
		id = sanitizeId(input.value);
		input.value = id;
	}

	let canSave = $derived(
		!isSubmitting && id.trim().length > 0 && label.trim().length > 0 && color.length > 0,
	);

	function handleSubmit() {
		if (!canSave) return;
		isSubmitting = true;
		onSave({ id: id.trim(), label: label.trim(), color });
	}
</script>

<!-- svelte-ignore a11y_no_static_element_interactions -->
<div class="modal-backdrop" onclick={onCancel} onkeydown={(e) => e.key === "Escape" && onCancel()}>
	<!-- svelte-ignore a11y_no_static_element_interactions -->
	<div class="modal" onclick={(e) => e.stopPropagation()}>
		<h2>{isEditing ? $t("categoryForm.editTitle") : $t("categoryForm.newTitle")}</h2>

		<div class="form-field">
			<label for="category-id">{$t("categoryForm.idLabel")}</label>
			<input
				id="category-id"
				type="text"
				value={id}
				oninput={handleIdInput}
				placeholder={$t("categoryForm.idPlaceholder")}
				disabled={isEditing}
			/>
			{#if isEditing}
				<span class="hint">{$t("categoryForm.idHint")}</span>
			{/if}
		</div>

		<div class="form-field">
			<label for="category-label">{$t("categoryForm.label")}</label>
			<input id="category-label" type="text" bind:value={label} placeholder={$t("categoryForm.labelPlaceholder")} />
		</div>

		<div class="form-field">
			<label for="category-color">{$t("categoryForm.color")}</label>
			<div class="color-row">
				<input id="category-color" type="color" bind:value={color} />
				<span class="color-preview" style="background: {color};">{color}</span>
			</div>
		</div>

		<div class="modal-actions">
			<button class="btn btn-cancel" onclick={onCancel} disabled={isSubmitting}>{$t("categoryForm.cancel")}</button>
			<button class="btn btn-confirm" onclick={handleSubmit} disabled={!canSave}>
				{isSubmitting ? $t("categoryForm.saving") : $t("categoryForm.save")}
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

	.form-field input[type="text"]:disabled {
		opacity: 0.6;
		cursor: not-allowed;
	}

	.hint {
		display: block;
		font-size: 0.8rem;
		color: #888;
		margin-top: 4px;
	}

	.color-row {
		display: flex;
		align-items: center;
		gap: 12px;
	}

	.color-row input[type="color"] {
		width: 48px;
		height: 40px;
		border: 1px solid #ccc;
		border-radius: 8px;
		padding: 2px;
		cursor: pointer;
	}

	.color-preview {
		display: inline-block;
		padding: 4px 12px;
		border-radius: 8px;
		color: #fff;
		font-size: 0.85rem;
		font-weight: 600;
		font-family: monospace;
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

		.color-row input[type="color"] {
			border-color: #555;
		}

		.btn-cancel {
			background: #333;
			color: #ddd;
		}
	}
</style>
