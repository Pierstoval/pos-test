<script lang="ts">
	import { onMount } from "svelte";
	import { invoke } from "@tauri-apps/api/core";
	import { save } from "@tauri-apps/plugin-dialog";
	import { writeFile } from "@tauri-apps/plugin-fs";
	import type { DashboardSummary } from "$lib/types";
	import { formatPrice } from "$lib/utils/format";
	import { t } from "$lib/i18n";

	let summary = $state<DashboardSummary | null>(null);
	let isLoading = $state(true);
	let error = $state<string | null>(null);

	onMount(async () => {
		try {
			summary = await invoke<DashboardSummary>("get_dashboard_summary");
		} catch (e) {
			error = $t("dashboard.loadError", { error: String(e) });
		} finally {
			isLoading = false;
		}
	});

	function centsToEuros(cents: number): string {
		return (cents / 100).toFixed(2);
	}

	function q(value: string): string {
		return `"${value.replace(/"/g, '""')}"`;
	}

	function exportCsv() {
		if (!summary) return;

		const lines: string[] = [];

		// Sales by product table
		lines.push([q($t("dashboard.colProduct")), q($t("dashboard.colUnitPrice")), q($t("dashboard.colQuantity")), q($t("dashboard.colTotal"))].join(";"));
		for (const row of summary.per_product) {
			const unitPrice = centsToEuros(Math.round(row.total_revenue / row.total_quantity));
			lines.push([
				q(row.product_name),
				q(unitPrice),
				q(String(row.total_quantity)),
				q(centsToEuros(row.total_revenue)),
			].join(";"));
		}
		lines.push([q($t("dashboard.productTableTotal")), q(""), q(""), q(centsToEuros(summary.total_revenue))].join(";"));

		// Blank separator line
		lines.push("");

		// Payment method breakdown table
		lines.push([q($t("dashboard.colPaymentMethod")), q($t("dashboard.colRevenue")), q($t("dashboard.colTransactions"))].join(";"));
		for (const row of summary.per_payment_method) {
			lines.push([
				q(row.payment_method),
				q(centsToEuros(row.total_revenue)),
				q(String(row.transaction_count)),
			].join(";"));
		}
		lines.push([q($t("dashboard.colTotal")), q(centsToEuros(summary.total_revenue)), q(String(summary.total_transactions))].join(";"));

		const csv = lines.join("\n");
		const blob = new Blob([csv], { type: "text/csv;charset=utf-8;" });

		save({
			defaultPath: "tableau-de-bord.csv",
			filters: [{ name: "CSV", extensions: ["csv"] }],
		}).then((path: string | null) => {
			if (!path) return;
			return writeFile(path, blob.stream());
		});
	}
</script>

<div class="dashboard-page">
	<div class="header">
		<h1>{$t("dashboard.title")}</h1>
		{#if summary && summary.total_transactions > 0}
			<button class="export-btn" onclick={exportCsv}>
				{$t("dashboard.exportCsv")}
			</button>
		{/if}
	</div>

	{#if isLoading}
		<div class="status-msg">{$t("dashboard.loading")}</div>
	{:else if error}
		<div class="status-msg error">{error}</div>
	{:else if summary && summary.total_transactions === 0}
		<div class="status-msg">{$t("dashboard.noSales")}</div>
	{:else if summary}
		<div class="kpi-row">
			<div class="kpi-card">
				<span class="kpi-label">{$t("dashboard.totalRevenue")}</span>
				<span class="kpi-value">{formatPrice(summary.total_revenue)}</span>
			</div>
			<div class="kpi-card">
				<span class="kpi-label">{$t("dashboard.totalTransactions")}</span>
				<span class="kpi-value">{summary.total_transactions}</span>
			</div>
		</div>

		<section class="section">
			<h2>{$t("dashboard.productTable")}</h2>
			<div class="table-wrapper">
				<table>
					<thead>
						<tr>
							<th>{$t("dashboard.colProduct")}</th>
							<th class="num">{$t("dashboard.colUnitPrice")}</th>
							<th class="num">{$t("dashboard.colQuantity")}</th>
							<th class="num">{$t("dashboard.colTotal")}</th>
						</tr>
					</thead>
					<tbody>
						{#each summary.per_product as row (row.product_id)}
							<tr>
								<td>{row.product_name}</td>
								<td class="num">{formatPrice(Math.round(row.total_revenue / row.total_quantity))}</td>
								<td class="num">{row.total_quantity}</td>
								<td class="num">{formatPrice(row.total_revenue)}</td>
							</tr>
						{/each}
					</tbody>
					<tfoot>
						<tr>
							<td colspan="3">{$t("dashboard.productTableTotal")}</td>
							<td class="num">{formatPrice(summary.total_revenue)}</td>
						</tr>
					</tfoot>
				</table>
			</div>
		</section>

		<section class="section">
			<h2>{$t("dashboard.paymentBreakdown")}</h2>
			<div class="table-wrapper">
				<table>
					<thead>
						<tr>
							<th>{$t("dashboard.colPaymentMethod")}</th>
							<th class="num">{$t("dashboard.colRevenue")}</th>
							<th class="num">{$t("dashboard.colTransactions")}</th>
						</tr>
					</thead>
					<tbody>
						{#each summary.per_payment_method as row (row.payment_method)}
							<tr>
								<td>{$t("dashboard.paymentMethod." + row.payment_method)}</td>
								<td class="num">{formatPrice(row.total_revenue)}</td>
								<td class="num">{row.transaction_count}</td>
							</tr>
						{/each}
					</tbody>
				</table>
			</div>
		</section>
	{/if}
</div>

<style>
	.dashboard-page {
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

	.kpi-row {
		display: flex;
		gap: 16px;
		margin-bottom: 24px;
	}

	.kpi-card {
		flex: 1;
		display: flex;
		flex-direction: column;
		gap: 4px;
		padding: 16px 20px;
		border: 1px solid #e0e0e0;
		border-radius: 12px;
		background: #f9fafb;
	}

	.kpi-label {
		font-size: 0.85rem;
		font-weight: 600;
		text-transform: uppercase;
		color: #888;
	}

	.kpi-value {
		font-size: 1.6rem;
		font-weight: 700;
		font-variant-numeric: tabular-nums;
	}

	.section {
		margin-bottom: 24px;
	}

	h2 {
		margin: 0 0 12px;
		font-size: 1.15rem;
		font-weight: 600;
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
		padding: 10px 12px;
		border-bottom: 1px solid #eee;
		font-size: 0.95rem;
	}

	tfoot td {
		border-bottom: none;
		border-top: 2px solid #e0e0e0;
		font-weight: 700;
		font-size: 1rem;
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

		.kpi-card {
			background: #262626;
			border-color: #333;
		}

		th {
			border-bottom-color: #444;
			color: #aaa;
		}

		td {
			border-bottom-color: #333;
		}

		tfoot td {
			border-top-color: #444;
		}
	}
</style>
