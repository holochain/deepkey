<script lang="ts">
	import type { DeviceInviteAcceptance } from '$lib/deepkey-client';
	import { messages } from '$lib/store/messages';
	import { decode } from '@msgpack/msgpack';
	import { derived, type Readable } from 'svelte/store';
	import InvitationAlert from './invitation-alert.svelte';
	import type { Invitation } from '../app';

	const invitations: Readable<Invitation[]> = derived(messages, (messages) => {
		const invites = messages
			.filter((msg) => msg.type === 'device_invite_acceptance')
			.map((msg) => ({
				id: msg.id,
				dia: decode(msg.bytes) as DeviceInviteAcceptance
			}));
		return invites;
	});

	invitations.subscribe((i) => {
		console.log(i);
	});
</script>

{#each $invitations as invite}
	<InvitationAlert {invite} />
{/each}
