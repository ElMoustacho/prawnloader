<script lang="ts">
	import { beforeNavigate, goto } from '$app/navigation';
	import type { Config } from '$models/Config';
	import { confirm } from '@tauri-apps/api/dialog';
	import type { UnionToTuple } from 'src/union-to-tuple';
	import { onDestroy, onMount } from 'svelte';
	import { writable } from 'svelte/store';

	export let data;

	const { config } = data;
	const youtubeFormats: UnionToTuple<Config['youtubeFormat']> = ['MP3', 'WEBM', 'WAV', 'OGG'];
	const tempConfig = writable(structuredClone($config));

	$: unsavedChanges = JSON.stringify($tempConfig) !== JSON.stringify($config);
	let forceNavigation = false;

	beforeNavigate(navigation => {
		if (!unsavedChanges || forceNavigation) return;

		navigation.cancel();
		confirm('You have unsaved changes. Do you want to continue?', {
			title: 'Prawnloader',
			type: 'warning',
		}).then(doContinue => {
			if (doContinue) {
				forceNavigation = true;
				if (navigation.to !== null) goto(navigation.to.url.href);
			}
		});
	});

	function escapeListener(event: KeyboardEvent) {
		if (event.key === 'Escape') cancelChanges();
	}

	onMount(() => {
		document.addEventListener('keydown', escapeListener);
	});

	onDestroy(() => {
		document.removeEventListener('keydown', escapeListener);
	});

	function validateChanges() {
		config.set(structuredClone($tempConfig));
	}

	function cancelChanges() {
		tempConfig.set(structuredClone($config));
	}
</script>

<h2 class="subtitle">Youtube</h2>

<div class="field">
	<!-- svelte-ignore a11y-label-has-associated-control -->
	<label class="label">Music format</label>
	<div class="select is-primary">
		<select bind:value={$tempConfig.youtubeFormat}>
			{#each youtubeFormats as youtubeFormat}
				<option value={youtubeFormat}>{youtubeFormat}</option>
			{/each}
		</select>
	</div>
	<p class="help">This will be the extension of the downloaded file.</p>
</div>

<div class="buttons">
	<button class="button is-primary" disabled={!unsavedChanges} on:click={validateChanges}>
		Confirm changes
	</button>
	<button class="button is-danger" disabled={!unsavedChanges} on:click={cancelChanges}>
		Cancel changes
	</button>
</div>
