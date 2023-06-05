<script lang="ts">
	let visible: boolean = true;
	let message: string = 'This is a notification message';

	import KeyAltIcon from '~icons/iconoir/key-alt-remove';
	import KeyPlusIcon from '~icons/iconoir/key-alt-plus';

	import Tree from 'svelte-tree';
	const tree = [
		{
			name: 'This is a root node',
			children: [
				{
					name: 'And it has'
				},
				{
					name: 'two children'
				}
			]
		},
		{
			name: 'This is another root node',
			children: [
				{
					name: 'This one is alone'
				},
				{
					name: 'But this one has nested children',
					children: [
						{
							name: 'Like this'
						},
						{
							name: 'and this'
						}
					]
				}
			]
		}
	];
</script>

<!-- Top section -->
<div class="bg-cover bg-center h-1/3" style="background-image: url('/deepkey-hero.jpg')">
	<div class="text-center pt-40 pb-20">
		<h1 class="text-4xl font-bold">
			<span class="gradient-heading">Deepkey Explorer</span>
		</h1>
	</div>
</div>

{#if visible}
	<aside class="alert variant-ghost m-5 bg-gradient-to-br variant-gradient-secondary-primary">
		<!-- Icon -->
		<KeyAltIcon class="h-12 w-12" />
		<!-- Message -->
		<div class="alert-message">
			<h3 class="h3">Key Revocation Request</h3>
			<p>You have received a request to revoke key 0x239828h2ff</p>
		</div>
		<!-- Actions -->
		<div class="alert-actions">
			<button type="button" class="btn variant-ghost-error">Sign Revocation</button>
			<button type="button" class="btn variant-ghost-surface">Delete Request</button>
		</div>
	</aside>
{/if}

{#if visible}
	<aside class="alert variant-ghost m-5 bg-gradient-to-br variant-gradient-secondary-primary">
		<!-- Icon -->
		<KeyPlusIcon class="h-12 w-12" />
		<!-- Message -->
		<div class="alert-message">
			<h3 class="h3">Device Invitation Received</h3>
			<p>You have received a request to join the Keyset of Root Agent #298fFfA9</p>
		</div>
		<!-- Actions -->
		<div class="alert-actions">
			<button type="button" class="btn variant-ghost-success">Accept Invitation</button>
			<button type="button" class="btn variant-ghost-surface">Delete Request</button>
		</div>
	</aside>
{/if}

<main class="card p-4 m-5">
	<Tree {tree} let:node>
		<div class="name">{node.name}</div>
	</Tree>
</main>
<footer class="h-32 m-12" />
