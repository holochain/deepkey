<script lang="ts">
	// Your selected Skeleton theme:
	// import '@skeletonlabs/skeleton/themes/theme-skeleton.css';
	import '@skeletonlabs/skeleton/themes/theme-rocket.css';
	// This contains the bulk of Skeletons required styles:
	// NOTE: this will be renamed skeleton.css in the v2.x release.
	import '@skeletonlabs/skeleton/styles/skeleton.css';

	// Finally, your application's global stylesheet (sometimes labeled 'app.css')
	import '../app.postcss';

	import { AppShell, AppBar } from '@skeletonlabs/skeleton';
	import { onMount } from 'svelte';
	import { holochain } from '$lib/store/holochain-client-store';
	import { deepkey } from '$lib/store/deepkey-client-store';

	onMount(async () => {
		await holochain.init();
		holochain.subscribe((hc) => console.log('hc change'));
		deepkey.subscribe((dk) => console.log('dk change'));
			
		// await deepkey.load;
		// const ksr = await $deepkey.queryKeysetRoot()
		// console.log(ksr);

		// const info = await $holochain?.appInfo();
		// console.log(info);
		// const dk = new DeepkeyClient($holochain, 'deepkey');
		// await dk.queryKeysetRoot();
	});

	// $: console.log($holochain, holochain.initComplete);
</script>

<AppShell>
	<svelte:fragment slot="header"
		><AppBar regionPage="relative" slotPageHeader="sticky top-0 z-10">Deepkey Explorer</AppBar
		></svelte:fragment
	>
	<!-- <svelte:fragment slot="sidebarLeft">Sidebar Left</svelte:fragment> -->
	<!-- (sidebarRight) -->
	<!-- (pageHeader) -->
	<!-- Router Slot -->
	<slot />
	<!-- ---- / ---- -->
	<svelte:fragment slot="pageFooter">
		<footer class="card mt-12 p-5 sticky bottom-0"><!-- Footer Goes here! --></footer>
	</svelte:fragment>
	<!-- (footer) -->
</AppShell>
