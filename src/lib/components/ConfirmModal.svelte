<script lang="ts">
	import { confirmState, _respond } from '$lib/confirm.svelte';
	import { t } from '$lib/i18n';

	function handleKeydown(e: KeyboardEvent) {
		if (e.key === 'Escape') {
			_respond(false);
		}
	}
</script>

<svelte:window onkeydown={confirmState.isOpen ? handleKeydown : undefined} />

{#if confirmState.isOpen}
	<!-- svelte-ignore a11y_no_static_element_interactions -->
	<div class="modal-backdrop" onclick={() => _respond(false)}>
		<!-- svelte-ignore a11y_no_static_element_interactions -->
		<div class="modal" onclick={(e) => e.stopPropagation()}>
			<h2>{$t('confirm.title')}</h2>
			<p class="message">{confirmState.message}</p>
			<div class="modal-actions">
				<button class="btn btn-cancel" onclick={() => _respond(false)}>
					{$t('confirm.cancel')}
				</button>
				<button class="btn btn-confirm" onclick={() => _respond(true)}>
					{$t('confirm.ok')}
				</button>
			</div>
		</div>
	</div>
{/if}

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

	.message {
		margin: 0 0 24px;
		font-size: 0.95rem;
		line-height: 1.5;
	}

	.modal-actions {
		display: flex;
		gap: 8px;
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

	.btn-cancel {
		background: #e0e0e0;
		color: #333;
	}

	.btn-confirm {
		background: #dc2626;
		color: #fff;
	}

	.btn-confirm:hover {
		background: #b91c1c;
	}

	@media (prefers-color-scheme: dark) {
		.modal {
			background: #1e1e1e;
			color: #f6f6f6;
		}

		.btn-cancel {
			background: #333;
			color: #ddd;
		}
	}
</style>
