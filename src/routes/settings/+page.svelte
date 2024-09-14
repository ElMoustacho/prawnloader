<script lang="ts">
	import { beforeNavigate, goto } from '$app/navigation';
	import { createConfig, type ConfigStore } from '$lib/config';
	import { invoke } from '$lib/tauri-wrapper';
	import type { Config } from '$models/Config';
	import { confirm } from '@tauri-apps/api/dialog';
	import type { UnionToTuple } from 'src/union-to-tuple';
	import { onDestroy, onMount } from 'svelte';
	import { writable } from 'svelte/store';

	let config: ConfigStore;

	const youtubeFormats: UnionToTuple<Config['youtubeFormat']> = ['mp3', 'webm', 'wav', 'ogg'];
	$: tempConfig = writable(structuredClone($config));

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

	function keydownListener(event: KeyboardEvent) {
		if (event.key === 'Escape') cancelChanges();
		else if (event.key === 'Enter' && event.ctrlKey) validateChanges();
	}

	onMount(() => {
		// Load config
		invoke('get_config', {}).then(_config => {
			config = createConfig(_config);
		});

		document.addEventListener('keydown', keydownListener);
	});

	onDestroy(() => {
		document.removeEventListener('keydown', keydownListener);
	});

	function validateChanges() {
		config.set(structuredClone($tempConfig));
	}

	function cancelChanges() {
		tempConfig.set(structuredClone($config));
	}
</script>

{#if $tempConfig !== undefined}
	<main class="is-unselectable">
		<section class="box">
			<h1 class="subtitle has-background-white"><i class="fa-solid fa-gear"></i> General</h1>

			<h2 class="subtitle has-text-grey-light">No settings</h2>
		</section>

		<section class="box">
			<h1 class="subtitle has-background-white">
				<i class="fa-brands fa-deezer"></i> Youtube
			</h1>

			<div class="field">
				<!-- svelte-ignore a11y-label-has-associated-control -->
				<label class="label">Music format</label>
				<div class="control has-icons-left">
					<div class="select is-primary is-small">
						<select bind:value={$tempConfig.youtubeFormat}>
							{#each youtubeFormats as youtubeFormat}
								<option value={youtubeFormat}>{youtubeFormat}</option>
							{/each}
						</select>
					</div>
					<div class="icon is-small is-left">
						<i class="fa-solid fa-file-audio"></i>
					</div>
				</div>
			</div>

			<div class="field">
				<!-- svelte-ignore a11y-label-has-associated-control -->
				<label class="label">Split by chapters</label>
				<div class="control">
					<label>
						<input
							class="checkbox is-primary is-small"
							type="checkbox"
							bind:checked={$tempConfig.splitByChaptersDefault} />
						Split videos with chapters
						<p class="help">
							All videos will be split into multiple files by default, using the
							chapters in the video description (if available).
							<br />
							Can be changed for each download manually.
						</p>
					</label>
				</div>
			</div>
		</section>

		<section class="box">
			<h1 class="subtitle has-background-white">
				<i class="fa-brands fa-youtube"></i> Deezer
			</h1>

			<h2 class="subtitle has-text-grey-light">No settings</h2>
		</section>

		<section class="box settings-buttons">
			<div class="buttons">
				<button
					title="<Ctrl+Enter> to validate changes."
					class="button is-primary"
					disabled={!unsavedChanges}
					on:click={validateChanges}>
					Confirm changes
				</button>
				<button
					title="<Esc> to cancel changes."
					class="button is-danger"
					disabled={!unsavedChanges}
					on:click={cancelChanges}>
					Cancel changes
				</button>
			</div>
		</section>
	</main>
{/if}

<style lang="scss">
	// h1.subtitle {
	// 	padding: 1.25rem 0 1.5rem 1.25rem;
	// 	margin: -1.25rem 0 0 -1.25rem;
	// 	position: sticky;
	// 	top: -1rem;
	// 	z-index: 10;
	// }

	.settings-buttons {
		position: sticky;
		bottom: 0;
		z-index: 11;
	}
</style>
