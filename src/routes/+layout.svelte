<script lang="ts">
	import NavMenu from '$lib/components/NavMenu.svelte';
	import ConfirmModal from '$lib/components/ConfirmModal.svelte';
	import type { Snippet } from 'svelte';
	import {APP_TARGET} from "$lib/api";

	let { children }: { children: Snippet } = $props();

	let topMargin = APP_TARGET === 'tauri';
</script>

<NavMenu />
<div class="page-content" class:topMargin={topMargin}>
	{@render children()}
</div>
<ConfirmModal />

<style>
	:global(body) {
		margin: 0;
		font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
		font-size: 16px;
		line-height: 1.5;
		color: #0f0f0f;
		background: #f6f6f6;
	}
	:global(body.topMargin) {
		padding-top: 15px;
	}

	@media (prefers-color-scheme: dark) {
		:global(body) {
			color: #f6f6f6;
			background: #1a1a1a;
		}
	}

	.page-content {
		padding-top: 48px;
	}
	.page-content.topMargin {
		margin-top: 20px;
	}
</style>
