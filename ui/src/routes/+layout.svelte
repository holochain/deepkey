<script lang="ts">
	import { Toast, initializeStores } from '@skeletonlabs/skeleton';
	import '../app.postcss';
	import { AppShell, AppBar } from '@skeletonlabs/skeleton';

	initializeStores();
</script>

<Toast />

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
