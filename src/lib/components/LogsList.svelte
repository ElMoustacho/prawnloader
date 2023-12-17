<script lang="ts">
	import { logs, type Log } from '$lib/log';
	import { onMount } from 'svelte';
	import LogComponent from './Log.svelte';

	let logsList: HTMLDivElement;
	$: if (logsList !== undefined) {
		// Re-run this block when logs is updated
		$logs;

		let isScrolledTop =
			Math.abs(logsList.scrollTop) >= logsList.scrollHeight - logsList.clientHeight;

		if (isScrolledTop) {
			scrollTop();
		}
	}

	onMount(scrollTop);

	function scrollTop() {
		requestAnimationFrame(() => {
			logsList.scrollTo({ top: -logsList.scrollHeight });
		});
	}
</script>

<div class="logs-wrapper max-overflow" bind:this={logsList}>
	{#each $logs as log}
		<LogComponent {log} />
	{/each}
</div>

<style lang="scss">
	.logs-wrapper {
		display: flex;
		flex-direction: column-reverse;
		overflow: hidden auto;
	}
</style>
