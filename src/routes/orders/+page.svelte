<script lang="ts">
	import { onMount } from "svelte";
	import { invoke } from "@tauri-apps/api/core";
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
			orders = await invoke<OrderWithItems[]>("list_orders");
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
		});
	}
</script>

<div class="orders-page">
	<div class="header">
		<h1>{$t("orders.title")}</h1>
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
