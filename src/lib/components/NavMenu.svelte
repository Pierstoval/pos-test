<script lang="ts">
	import { page } from "$app/state";
	import { api_call } from "$lib/api";
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
	let menuOpen = $state(false);

	function isActive(href: string): boolean {
		if (href === "/") return page.url.pathname === "/";
		return page.url.pathname.startsWith(href);
	}

	function toggleMenu() {
		menuOpen = !menuOpen;
	}

	function closeMenu() {
		menuOpen = false;
	}

	async function resetDatabase() {
		if (!confirm($t("nav.resetConfirm"))) return;
		isResetting = true;
		try {
			await api_call("reset_database");
			window.location.reload();
		} catch (e) {
			alert($t("nav.resetFailed", { error: String(e) }));
			isResetting = false;
		}
	}
</script>

<nav class="nav-menu">
	<button class="hamburger" onclick={toggleMenu} aria-label="Menu">
		<span class="hamburger-line"></span>
		<span class="hamburger-line"></span>
		<span class="hamburger-line"></span>
	</button>

	<div class="menu-links" class:open={menuOpen}>
		{#each links as link}
			<a href={link.href} class:active={isActive(link.href)} onclick={closeMenu}>
				{$t(link.labelKey)}
			</a>
		{/each}
		{#if isDev}
			<button class="reset-btn" onclick={resetDatabase} disabled={isResetting}>
				{isResetting ? $t("nav.resetting") : $t("nav.resetDb")}
			</button>
		{/if}
	</div>
</nav>

{#if menuOpen}
	<!-- svelte-ignore a11y_no_static_element_interactions -->
	<div class="backdrop" onclick={closeMenu}></div>
{/if}

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

	/* Hamburger button — hidden on desktop */
	.hamburger {
		display: none;
		flex-direction: column;
		justify-content: center;
		gap: 4px;
		width: 58px;
		height: 46px;
		padding: 6px 20px;
		border: none;
		border-radius: 6px;
		background: transparent;
		cursor: pointer;
	}

	.hamburger:hover {
		background: #f0f0f0;
	}

	.hamburger-line {
		display: block;
		width: 100%;
		height: 2px;
		background: #555;
		border-radius: 1px;
	}

	/* Backdrop — hidden on desktop */
	.backdrop {
		display: none;
	}

	/* Desktop: menu-links is inline flex */
	.menu-links {
		display: flex;
		align-items: center;
		gap: 4px;
		flex: 1;
	}

	/* Mobile layout */
	@media (max-width: 699px) {
		.hamburger {
			display: flex;
		}

		.menu-links {
			position: fixed;
			top: 0;
			left: 0;
			bottom: 0;
			width: 260px;
			z-index: 1100;
			flex-direction: column;
			align-items: stretch;
			gap: 2px;
			padding: 60px 12px 12px;
			background: #ffffff;
			border-right: 1px solid #e0e0e0;
			transform: translateX(-100%);
			transition: transform 0.25s ease;
			overflow-y: auto;
		}

		.menu-links.open {
			transform: translateX(0);
		}

		.menu-links a {
			height: 44px;
			padding: 0 16px;
			font-size: 1rem;
		}

		.menu-links .reset-btn {
			margin-left: 0;
			margin-top: auto;
			height: 44px;
			font-size: 0.85rem;
		}

		.backdrop {
			display: block;
			position: fixed;
			inset: 0;
			z-index: 950;
			background: rgba(0, 0, 0, 0.4);
		}
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

		.hamburger:hover {
			background: #2a2a2a;
		}

		.hamburger-line {
			background: #ddd;
		}
	}

	@media (prefers-color-scheme: dark) and (max-width: 699px) {
		.menu-links {
			background: #1e1e1e;
			border-right-color: #333;
		}

		.backdrop {
			background: rgba(0, 0, 0, 0.6);
		}
	}
</style>
