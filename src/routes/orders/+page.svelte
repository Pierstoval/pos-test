<script lang="ts">
	import { onMount } from "svelte";
	import { api_call } from "$lib/api";
	import { save } from '@tauri-apps/plugin-dialog';
	import { writeTextFile, writeFile } from '@tauri-apps/plugin-fs';
	import type { OrderWithItems } from "$lib/types";
	import { formatPrice } from "$lib/utils/format";
	import { t } from "$lib/i18n";

	let orders = $state<OrderWithItems[]>([]);
	let isLoading = $state(true);
	let error = $state<string | null>(null);

	onMount(async () => {
		await loadOrders();
	});

	async function loadOrders() {
		isLoading = true;
		error = null;
		try {
			orders = await api_call<OrderWithItems[]>("list_orders");
		} catch (e) {
			error = $t("orders.loadError", { error: String(e) });
		} finally {
			isLoading = false;
		}
	}

	function formatDateTime(iso: string): string {
		const d = new Date(iso);
		return d.toLocaleDateString("fr-FR", {
			day: "2-digit",
			month: "2-digit",
			year: "numeric",
		}) + " " + d.toLocaleTimeString("fr-FR", {
			hour: "2-digit",
			minute: "2-digit",
			second: "2-digit",
		});
	}

	function centsToEuros(cents: number): string {
		return (cents / 100).toFixed(2);
	}

	function q(value: string): string {
		return `"${value.replace(/"/g, '""')}"`;
	}

	function exportCsv() {
		// Collect all unique product names from order items, in first-seen order
		const productNames: string[] = [];
		const seen = new Set<string>();
		for (const order of orders) {
			for (const item of order.items) {
				if (!seen.has(item.product_name)) {
					seen.add(item.product_name);
					productNames.push(item.product_name);
				}
			}
		}

		// Build header
		const header = [
			q($t("orders.csvColId")),
			q($t("orders.csvColDate")),
			q($t("orders.csvColTotal")),
			q($t("orders.csvColPaymentMethod")),
		];
		for (const name of productNames) {
			header.push(q(`${name} (${$t("orders.colUnitPrice")})`));
			header.push(q(`${name} (${$t("orders.colQty")})`));
			header.push(q(`${name} (${$t("orders.colSubtotal")})`));
		}

		const lines: string[] = [header.join(";")];

		// Build one line per order
		for (const order of orders) {
			const itemsByProduct = new Map<string, typeof order.items[number]>();
			for (const item of order.items) {
				itemsByProduct.set(item.product_name, item);
			}

			const row = [
				q(order.id),
				q(order.created_at),
				q(centsToEuros(order.total)),
				q(order.payment_method),
			];
			for (const name of productNames) {
				const item = itemsByProduct.get(name);
				if (item) {
					row.push(q(centsToEuros(item.unit_price)));
					row.push(q(String(item.quantity)));
					row.push(q(centsToEuros(item.total)));
				} else {
					row.push(q(""));
					row.push(q(""));
					row.push(q(""));
				}
			}
			lines.push(row.join(";"));
		}

		const csv = lines.join("\n");
		const blob = new Blob([csv], { type: "text/csv;charset=utf-8;" });

		const path = save({
			defaultPath: 'commandes.csv',
			filters: [
				{
					name: 'CSV',
					extensions: ['csv']
				},
			],
		}).then((path: string|null) => {
			if (!path) {
				throw new Error('no path');
			}
			return writeFile(path, blob.stream());
		});

		// const url = URL.createObjectURL(blob);
		// const a = document.createElement("a");
		// a.href = url;
		// a.download = "orders.csv";
		// a.click();
		// URL.revokeObjectURL(url);
		// console.info('CSV', csv);
		// console.info('blob', blob);
		// console.info('URL', url);
		// console.info('anchor', a);
	}
</script>

<div class="orders-page">
	<div class="header">
		<h1>{$t("orders.title")}</h1>
		{#if orders.length > 0}
			<button class="export-btn" onclick={exportCsv}>
				{$t("orders.exportCsv")}
			</button>
		{/if}
	</div>

	{#if isLoading}
		<div class="status-msg">{$t("orders.loading")}</div>
	{:else if error}
		<div class="status-msg error">{error}</div>
	{:else if orders.length === 0}
		<div class="status-msg">{$t("orders.empty")}</div>
	{:else}
		<div class="order-list">
			{#each orders as order (order.id)}
				<div class="order-card">
					<div class="order-header">
						<span class="order-date">{formatDateTime(order.created_at)}</span>
						<span class="order-payment badge-{order.payment_method}">
							{$t("orders.paymentMethod." + order.payment_method)}
						</span>
						<span class="order-total">{formatPrice(order.total)}</span>
					</div>
					<table class="items-table">
						<thead>
							<tr>
								<th>{$t("orders.colProduct")}</th>
								<th class="num">{$t("orders.colQty")}</th>
								<th class="num">{$t("orders.colUnitPrice")}</th>
								<th class="num">{$t("orders.colSubtotal")}</th>
							</tr>
						</thead>
						<tbody>
							{#each order.items as item (item.id)}
								<tr>
									<td>{item.product_name}</td>
									<td class="num">{item.quantity}</td>
									<td class="num">{formatPrice(item.unit_price)}</td>
									<td class="num">{formatPrice(item.total)}</td>
								</tr>
							{/each}
						</tbody>
					</table>
				</div>
			{/each}
		</div>
	{/if}
</div>

<style>
	.orders-page {
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

	.export-btn {
		padding: 0 14px;
		height: 34px;
		border: 1px solid #3b82f6;
		border-radius: 6px;
		background: transparent;
		color: #3b82f6;
		font-size: 0.85rem;
		font-weight: 600;
		cursor: pointer;
	}

	.export-btn:hover {
		background: #3b82f6;
		color: #fff;
	}

	.order-list {
		display: flex;
		flex-direction: column;
		gap: 16px;
	}

	.order-card {
		border: 1px solid #e0e0e0;
		border-radius: 12px;
		overflow: hidden;
	}

	.order-header {
		display: flex;
		align-items: center;
		gap: 12px;
		padding: 12px 16px;
		background: #f9fafb;
		border-bottom: 1px solid #e0e0e0;
	}

	.order-date {
		font-size: 0.9rem;
		color: #555;
	}

	.order-payment {
		display: inline-block;
		padding: 2px 10px;
		border-radius: 12px;
		font-size: 0.8rem;
		font-weight: 600;
	}

	.badge-cash {
		background: #dcfce7;
		color: #166534;
	}

	.badge-card {
		background: #dbeafe;
		color: #1e40af;
	}

	.order-total {
		margin-left: auto;
		font-size: 1.1rem;
		font-weight: 700;
		font-variant-numeric: tabular-nums;
	}

	.items-table {
		width: 100%;
		border-collapse: collapse;
	}

	.items-table th {
		text-align: left;
		padding: 8px 16px;
		font-size: 0.8rem;
		text-transform: uppercase;
		color: #888;
		font-weight: 600;
		border-bottom: 1px solid #eee;
	}

	.items-table td {
		padding: 8px 16px;
		font-size: 0.9rem;
		border-bottom: 1px solid #f3f4f6;
	}

	.items-table tr:last-child td {
		border-bottom: none;
	}

	.num {
		text-align: right;
		font-variant-numeric: tabular-nums;
		white-space: nowrap;
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
		.export-btn {
			border-color: #60a5fa;
			color: #60a5fa;
		}

		.export-btn:hover {
			background: #60a5fa;
			color: #111;
		}

		.order-card {
			border-color: #333;
		}

		.order-header {
			background: #262626;
			border-bottom-color: #333;
		}

		.order-date {
			color: #aaa;
		}

		.badge-cash {
			background: #14532d;
			color: #86efac;
		}

		.badge-card {
			background: #1e3a5f;
			color: #93c5fd;
		}

		.items-table th {
			color: #aaa;
			border-bottom-color: #333;
		}

		.items-table td {
			border-bottom-color: #2a2a2a;
		}
	}
</style>
