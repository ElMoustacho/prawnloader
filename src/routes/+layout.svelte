<script lang="ts">
	import '../scss/app.scss';

	const links = [
		['/', 'Home'],
		['/settings', 'Settings'],
	];
</script>

<main>
	<div class="tabs">
		<ul>
			{#each links as link}
				<!-- TODO: Add "is-active" when on a page under this link -->
				<a
					class="navbar-item"
					data-sveltekit-preload-data
					href={link[0]}
				>
					{link[1]}
				</a>
			{/each}
		</ul>
	</div>

	<section class="section container">
		<slot />
	</section>
</main>
