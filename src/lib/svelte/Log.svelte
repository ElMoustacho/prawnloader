<script lang="ts">
	import type { Log } from '$lib/ts/log';

	export let log: Log;

	const prefix = log.success ? 'SUCCESS' : 'ERROR';
	const tagColor = log.success ? 'is-success' : 'is-danger';
</script>

<p class="py-1">
	<span title={log.timestamp.toLocaleString()}>[{log.timestamp.toLocaleTimeString()}]</span>
	<span class="tag {tagColor}">{prefix}</span>
	<span>{log.content}</span>
</p>
