<script lang="ts">
	import NavMenu from "$lib/components/NavMenu.svelte";
	import type { Snippet } from "svelte";

	let { children }: { children: Snippet } = $props();
</script>

<NavMenu />
<div class="page-content">
	{@render children()}
</div>

<style>
	:global(body) {
		margin: 0;
		font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
		font-size: 16px;
		line-height: 1.5;
		color: #0f0f0f;
		background: #f6f6f6;
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
</style>
