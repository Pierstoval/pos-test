<script lang="ts">
	import { page } from "$app/state";
	import { invoke } from "@tauri-apps/api/core";
	import { t } from "$lib/i18n";

	const links = [
		{ href: "/", labelKey: "nav.sales" },
		{ href: "/products", labelKey: "nav.products" },
		{ href: "/categories", labelKey: "nav.categories" },
		{ href: "/orders", labelKey: "nav.orders" },
		{ href: "/dashboard", labelKey: "nav.dashboard" },
	];

	const isDev = import.meta.env.DEV;

	let isResetting = $state(false);

	function isActive(href: string): boolean {
		if (href === "/") return page.url.pathname === "/";
		return page.url.pathname.startsWith(href);
	}

	async function resetDatabase() {
		if (!confirm($t("nav.resetConfirm"))) return;
		isResetting = true;
		try {
			await invoke("reset_database");
			window.location.reload();
		} catch (e) {
			alert($t("nav.resetFailed", { error: String(e) }));
			isResetting = false;
		}
	}
</script>

<nav class="nav-menu">
	{#each links as link}
		<a href={link.href} class:active={isActive(link.href)}>
			{$t(link.labelKey)}
		</a>
	{/each}
	{#if isDev}
		<button class="reset-btn" onclick={resetDatabase} disabled={isResetting}>
			{isResetting ? $t("nav.resetting") : $t("nav.resetDb")}
		</button>
	{/if}
</nav>

<style>
	.nav-menu {
		position: fixed;
		top: 0;
		left: 0;
		right: 0;
		z-index: 1000;
		height: 48px;
		display: flex;
		align-items: center;
		gap: 4px;
		padding: 0 12px;
		background: #ffffff;
		border-bottom: 1px solid #e0e0e0;
	}

	a {
		display: flex;
		align-items: center;
		height: 32px;
		padding: 0 14px;
		border-radius: 6px;
		text-decoration: none;
		font-size: 0.9rem;
		font-weight: 500;
		color: #555;
		transition:
			background 0.15s,
			color 0.15s;
	}

	a:hover {
		background: #f0f0f0;
		color: #111;
	}

	a.active {
		background: #e8e8e8;
		color: #111;
		font-weight: 600;
	}

	.reset-btn {
		margin-left: auto;
		padding: 0 12px;
		height: 32px;
		border: 1px solid #dc2626;
		border-radius: 6px;
		background: transparent;
		color: #dc2626;
		font-size: 0.8rem;
		font-weight: 600;
		cursor: pointer;
	}

	.reset-btn:hover:not(:disabled) {
		background: #dc2626;
		color: #fff;
	}

	.reset-btn:disabled {
		opacity: 0.5;
		cursor: not-allowed;
	}

	@media (prefers-color-scheme: dark) {
		.nav-menu {
			background: #1e1e1e;
			border-bottom-color: #333;
		}

		a {
			color: #aaa;
		}

		a:hover {
			background: #2a2a2a;
			color: #eee;
		}

		a.active {
			background: #333;
			color: #fff;
		}
	}
</style>
