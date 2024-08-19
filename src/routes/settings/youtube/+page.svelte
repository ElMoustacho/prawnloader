<script lang="ts">
	import type { Config } from '$models/Config';
	import type { UnionToTuple } from 'src/union-to-tuple';
	import type { PageData } from '../downloader/$types';

	export let data: PageData;

	const youtubeFormats: UnionToTuple<Config['youtubeFormat']> = ['MP3', 'WEBM', 'WAV', 'OGG'];
</script>

<h2 class="subtitle">Youtube</h2>

<label>
	Music format:
	<select value={data.config.youtubeFormat}>
		{#each youtubeFormats as youtubeFormat}
			<option value={youtubeFormat}>{youtubeFormat}</option>
		{/each}
	</select>
</label>
