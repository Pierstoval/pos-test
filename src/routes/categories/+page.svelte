<script lang="ts">
	import { onMount } from "svelte";
	import { invoke } from "@tauri-apps/api/core";
	import type { Category, CreateCategoryPayload, UpdateCategoryPayload } from "$lib/types";
	import CategoryFormModal from "$lib/components/CategoryFormModal.svelte";
	import { t } from "$lib/i18n";

	let categories = $state<Category[]>([]);
	let editingCategory = $state<Category | null>(null);
	let isFormOpen = $state(false);
	let isLoading = $state(true);
	let error = $state<string | null>(null);

	onMount(async () => {
		await loadCategories();
	});

	async function loadCategories() {
		isLoading = true;
		error = null;
		try {
			categories = await invoke<Category[]>("list_categories");
		} catch (e) {
			error = $t("categories.loadError", { error: String(e) });
		} finally {
			isLoading = false;
		}
	}

	function openCreate() {
		editingCategory = null;
		isFormOpen = true;
	}

	function openEdit(category: Category) {
		editingCategory = category;
		isFormOpen = true;
	}

	function closeForm() {
		isFormOpen = false;
		editingCategory = null;
	}

	async function handleSave(data: { id: string; label: string; color: string }) {
		try {
			if (editingCategory) {
				const payload: UpdateCategoryPayload = {
					id: data.id,
					label: data.label,
					color: data.color,
				};
				await invoke<Category>("update_category", { payload });
			} else {
				const payload: CreateCategoryPayload = {
					id: data.id,
					label: data.label,
					color: data.color,
				};
				await invoke<Category>("create_category", { payload });
			}
			closeForm();
			await loadCategories();
		} catch (e) {
			error = $t("categories.saveError", { error: String(e) });
			closeForm();
		}
	}
</script>

<div class="categories-page">
	<div class="header">
		<h1>{$t("categories.title")}</h1>
		<button class="btn btn-add" onclick={openCreate}>{$t("categories.addButton")}</button>
	</div>

	{#if isLoading}
		<div class="status-msg">{$t("categories.loading")}</div>
	{:else if error}
		<div class="status-msg error">{error}</div>
	{:else if categories.length === 0}
		<div class="status-msg">{$t("categories.empty")}</div>
	{:else}
		<div class="table-wrapper">
			<table>
				<thead>
					<tr>
						<th>{$t("categories.colId")}</th>
						<th>{$t("categories.colLabel")}</th>
						<th>{$t("categories.colColor")}</th>
						<th>{$t("categories.colActions")}</th>
					</tr>
				</thead>
				<tbody>
					{#each categories as cat (cat.id)}
						<tr>
							<td class="id-cell"><code>{cat.id}</code></td>
							<td>{cat.label}</td>
							<td>
								<span
									class="color-swatch"
									style="background: {cat.color};"
								></span>
								<code>{cat.color}</code>
							</td>
							<td>
								<button class="btn btn-edit" onclick={() => openEdit(cat)}>
									{$t("categories.edit")}
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
	<CategoryFormModal category={editingCategory} onSave={handleSave} onCancel={closeForm} />
{/if}

<style>
	.categories-page {
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

	.id-cell code {
		background: #f3f4f6;
		padding: 2px 8px;
		border-radius: 4px;
		font-size: 0.85rem;
	}

	.color-swatch {
		display: inline-block;
		width: 20px;
		height: 20px;
		border-radius: 4px;
		vertical-align: middle;
		margin-right: 8px;
		border: 1px solid rgba(0, 0, 0, 0.1);
	}

	td code {
		font-size: 0.85rem;
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

		.id-cell code {
			background: #374151;
			color: #d1d5db;
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
