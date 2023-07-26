<script lang="ts">
	import { onMount, setContext } from 'svelte';
	import TreeView from 'svelte-tree-view';
	
	import type { AppAgentClient } from '@holochain/client';
	import { decode, encode } from '@msgpack/msgpack';
	// import Fa from 'svelte-fa'
	// import { faMap, faUser, faGear, faCalendar, faPlus, faHome, faSync, faArrowRightFromBracket, faArrowRotateBack } from '@fortawesome/free-solid-svg-icons';
	// import { getCookie, deleteCookie, setCookie } from 'svelte-cookie';
	import { Base64 } from 'js-base64';
	import KeyAltIcon from '~icons/iconoir/key-alt-remove';
	import KeyPlusIcon from '~icons/iconoir/key-alt-plus';

	import { DeepkeyClient } from '../lib/deepkey-client';
	import { authorizeClient, setupHolochain } from '$lib/holochain-client';

	const data = {
		a: [1, 2, 3],
		b: new Map([
			['c', { d: null }],
			['e', { f: [9, 8, 7] }]
		])
	};

	let client: AppAgentClient | undefined;
	// let loading = true;
	// let state: 'initial' | 'authorizing' | 'loading' | 'error' | 'success' = 'authorizing';
	// let syncing = false;
	// let error: any = undefined;
	let creds: any;
	let deepkeyCellId;
	let appInfo;

	// $: error;
	$: client;
	// $: loading;
	// $: state;

	// const base64ToUint8 = (b64: string) => Base64.toUint8Array(b64);

	onMount(async () => {
		let app_role = 'deepkey';

		client = await setupHolochain();

		const deepkey = new DeepkeyClient(client, app_role);

		const res2 = await deepkey.key_state(client.myPubKey);
		console.log('res2', res2);

		const keyset_authority = await deepkey.keyset_authority();
		console.log(keyset_authority);

	});

	let visible: boolean = true;
	let message: string = 'This is a notification message';
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
	<TreeView
		{data}
		recursionOpts={{
			maxDepth: 16,
			shouldExpandNode: () => true
		}}
	/>
</main>

<footer class="h-32 m-12" />

<!-- 
<main>
	{#if error}
		<span class="notice">
			<h3>I'm sorry to say it, but there has been an error ☹️</h3>
			<div style="padding:10px; margin:10px; background:lightcoral;border-radius: 10px;">
				{error}
			</div>
			{#if creds}
				<div>
					You are signed in to the holochain multiplexer with reg key: <strong
						>{creds.regkey}</strong
					>
				</div>
				<sl-button
					style="margin-left: 8px;"
					on:click={() => {
						deleteCookie('creds');
						window.location.assign('/');
					}}
				>
					<Fa icon={faArrowRightFromBracket} /> Logout
				</sl-button>
			{/if}
			<sl-button style="margin-left: 8px;" on:click={() => window.location.assign('/')}>
				<Fa icon={faArrowRotateBack} /> Reload
			</sl-button>
		</span>
	{:else if loading}
		<div style="display: flex; flex: 1; align-items: center; justify-content: center">
			<sl-spinner />
		</div>
	{:else}
		<profiles-context store={profilesStore}>
			{#if $prof && ($prof.status !== 'complete' || $prof.value === undefined)}
				<div class="app-info">
					<img
						style="margin-right:20px"
						width="100"
						src="/images/emergence-vertical.svg"
						on:click={() => adminCheck()}
					/>
					<p>
						Decentralized & local scheduling, collaboration & connection{#if $uiProps.amSteward}!{/if}
					</p>
				</div>
			{/if}
			<profile-prompt>
				{#if (!sitemaps || $sitemaps.length == 0) && !$uiProps.amSteward}
					<div class="app-info">
						<img width="100" src="/images/emergence-vertical.svg" on:click={() => adminCheck()} />
						<p>
							Either your node hasn't synchronized yet with the network, or the conference data
							hasn't yet been set up. Please be patient!
						</p>
						<sl-button on:click={() => doSync()}>
							<span class:spinning={true}> <Fa icon={faArrowRotateBack} /> </span>Reload
						</sl-button>
						{#if syncing}<span class:spinning={true}> <Fa icon={faSync} /></span>{/if}
					</div>
				{:else}
					<div class="nav">
						<div class="button-group">
							<div
								class="nav-button {pane === 'discover' ? 'selected' : ''}"
								title="Discover"
								on:keypress={() => {
									store.setPane('discover');
								}}
								on:click={() => {
									store.setPane('discover');
								}}
							>
								<Fa class="nav-icon" icon={faHome} size="2x" />
								<span class="button-title">Discover</span>
							</div>
							<div
								class="nav-button {pane.startsWith('sessions') ? 'selected' : ''}"
								title="Sessions"
								on:keypress={() => {
									store.setPane('sessions');
								}}
								on:click={() => {
									store.setPane('sessions');
								}}
							>
								<Fa class="nav-icon" icon={faCalendar} size="2x" />
								<span class="button-title">Sessions</span>
							</div>

							<div
								class="nav-button {pane.startsWith('spaces') ? 'selected' : ''}"
								title="Spaces"
								on:keypress={() => {
									store.setPane('spaces');
								}}
								on:click={() => {
									store.setPane('spaces');
								}}
							>
								<Fa class="nav-icon" icon={faMap} size="2x" />
								<span class="button-title">Spaces</span>
							</div>
						</div>
						<div class="button-group settings">
							{#if store && $uiProps.amSteward}
								<div
									class="nav-button {pane.startsWith('admin') ? 'selected' : ''}"
									title="Admin"
									on:keypress={() => {
										store.setPane('admin');
									}}
									on:click={() => {
										store.setPane('admin');
									}}
								>
									<Fa class="nav-icon" icon={faGear} size="2x" />
									<span class="button-title settings">Settings</span>
								</div>
							{/if}
							<div
								class="nav-button {pane == 'you' ? 'selected' : ''}"
								title="You"
								on:keypress={() => {
									store.setPane('you');
								}}
								on:dblclick={(e) => e.stopPropagation()}
								on:click={(e) => {
									e.stopPropagation();
									if (pane == 'you') adminCheck();
									else store.setPane('you');
								}}
							>
								<Fa class="nav-icon" icon={faUser} size="2x" />
								<span class="button-title you">You</span>
							</div>

							<div
								class="nav-button"
								class:spinning={syncing}
								title="Sync"
								on:keypress={() => {
									doSync();
								}}
								on:click={() => {
									doSync();
								}}
							>
								<Fa class="nav-icon " icon={faSync} size="2x" />
								<span class="button-title sync">Sync</span>
							</div>
							{#if getCookie('creds')}
								<div
									class="nav-button"
									title="Logout"
									on:click={() => {
										window.location.assign('/reset');
									}}
								>
									<Fa class="nav-icon" icon={faArrowRightFromBracket} size="2x" />
									<span class="button-title">Logout</span>
								</div>
							{/if}
						</div>
					</div>

					<file-storage-context client={fileStorageClient}>
						{#if store && $uiProps.detailsStack[0] && $uiProps.detailsStack[0].type == DetailsType.ProxyAgent}
							<div class="session-details">
								<ProxyAgentDetail
									on:proxyagent-deleted={() => store.closeDetails()}
									on:proxyagent-close={() => store.closeDetails()}
									proxyAgentHash={$uiProps.detailsStack[0].hash}
								/>
							</div>
						{/if}
						{#if store && $uiProps.detailsStack[0] && $uiProps.detailsStack[0].type == DetailsType.Space}
							<div class="session-details">
								<SpaceDetail
									on:space-deleted={() => store.closeDetails()}
									on:space-close={() => store.closeDetails()}
									space={store.getSpace($uiProps.detailsStack[0].hash)}
								/>
							</div>
						{/if}
						{#if store && $uiProps.detailsStack[0] && $uiProps.detailsStack[0].type == DetailsType.Session}
							<div class="session-details">
								<SessionDetail
									on:session-deleted={() => store.closeDetails()}
									on:session-close={() => store.closeDetails()}
									sessionHash={$uiProps.detailsStack[0].hash}
								/>
							</div>
						{/if}
						{#if store && $uiProps.detailsStack[0] && $uiProps.detailsStack[0].type == DetailsType.Folk}
							<div class="session-details">
								<Folk
									on:folk-close={() => store.closeDetails()}
									agentPubKey={$uiProps.detailsStack[0].hash}
								/>
							</div>
						{/if}

						<div id="content" style="display: flex; flex-direction: column; flex: 1;">
							{#if pane == 'sessions'}
								<div class="pane">
									<AllSessions />
									<div
										class="create-session"
										on:click={() => {
											createSessionDialog.open(undefined);
										}}
									>
										<div class="summary">
											<div class="slot">
												<div class="slot-wrapper">+</div>
											</div>
											<div class="info">
												<div class="top-area">
													<div class="left-side">
														<span><strong>Create a session</strong></span>
														<p>
															What are you excited to share with this community? What special
															insights and wisdom are you ready to share?
														</p>
													</div>
													<div class="right-side" />
												</div>
											</div>
										</div>
									</div>

									<SessionCrud bind:this={createSessionDialog} on:session-created={() => {}} />
								</div>
							{/if}

							{#if pane == 'schedule'}
								<div class="pane">
									<ScheduleUpcoming on:open-slotting={() => store.setPane('schedule.slotting')} />
								</div>
							{/if}

							{#if pane == 'schedule.slotting'}
								<div class="pane">
									<ScheduleSlotting on:slotting-close={() => store.setPane('admin')} />
								</div>
							{/if}

							{#if pane == 'spaces'}
								<div class="pane sitemap">
									{#if store.getCurrentSiteMap()}
										<SiteMapDisplay
											sitemap={store.getCurrentSiteMap()}
											on:show-all-spaces={() => store.setPane('spaces.list')}
										/>
									{:else}
										<h5>No Sitemap configured yet</h5>
									{/if}
								</div>
							{/if}

							{#if pane == 'spaces.list'}
								<div class="pane spaces">
									<AllSpaces on:all-spaces-close={() => store.setPane('spaces')} />

									<SpaceCrud bind:this={createSpaceDialog} on:space-created={() => {}} />
								</div>
							{/if}

							{#if pane == 'you'}
								<div class="pane you">
									<You />
								</div>
							{/if}
							{#if pane == 'admin'}
								<div class="pane">
									<Admin
										on:open-sitemaps={() => (pane = 'admin.sitemaps')}
										on:open-proxyagents={() => (pane = 'admin.proxyagents')}
										on:open-slotting={() => store.setPane('schedule.slotting')}
									/>
								</div>
							{/if}
							{#if pane == 'admin.sitemaps'}
								<div class="pane">
									<AllSiteMaps on:sitemaps-close={() => (pane = 'admin')} />
								</div>
							{/if}
							{#if pane == 'admin.proxyagents'}
								<div class="pane">
									<ProxyAgentCrud on:proxyagent-created={() => {}} />

									<AllProxyAgents on:proxyagents-close={() => (pane = 'admin')} />
								</div>
							{/if}
							{#if pane == 'discover'}
								<div class="pane">
									<Discover />
								</div>
							{/if}
						</div>
					</file-storage-context>
				{/if}
			</profile-prompt>
		</profiles-context>
	{/if}
</main> -->
<!-- 
<style>
	.app-info {
		display: flex;
		justify-content: center;
		align-items: center;
		flex-direction: column;
		max-width: 320px;
		margin: 0 auto;
		text-align: center;
		margin-bottom: 30px;
	}

	.app-info p {
		opacity: 0.6;
		padding-top: 30px;
	}

	.sitemap {
		display: flex;
		flex-direction: row;
	}

	.notice {
		display: block;
		text-align: center;
		max-width: 1000px;
		padding: 25px;
		border: 1px solid;
		border-radius: 20px;
		margin: auto;
	}
</style> -->
