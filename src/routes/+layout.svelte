<script lang="ts">
	import '../app.scss';

	const links = [
		['/', 'Home'],
		['/settings', 'Settings'],
	];
</script>

<main>
	<nav class="navbar" aria-label="main navigation">
		<div class="navbar-brand">
			<div class="navbar-item">
				<img src="/favicon.png" alt="" />
			</div>
		</div>

		<div class="navbar-start">
			{#each links as link}
				<a
					class="navbar-item"
					data-sveltekit-preload-data
					href={link[0]}
				>
					{link[1]}
				</a>
			{/each}
		</div>
	</nav>

	<section class="section container">
		<slot />
	</section>
</main>
