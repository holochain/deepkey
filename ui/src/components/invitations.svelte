<script lang="ts">
	import type { DeviceInviteAcceptance } from "$lib/deepkey-client";
	import { messages } from "$lib/store/messages";
	import { decode } from "@msgpack/msgpack";
	import { derived } from "svelte/store";
	import InvitationAlert from "./invitation-alert.svelte";

	const invitations = derived(
		messages,
		(ms) =>
			ms
				.filter((msg) => msg.type === 'device_invite_acceptance')
				.map((msg) => decode(msg.bytes) as DeviceInviteAcceptance)
		// .map((msg) => (msg.bytes ? (decode(msg.bytes) as DeviceInviteAcceptance) : null))
		// .filter(Boolean)
	);

	invitations.subscribe((i) => {
		console.log(i);
	});

</script>


{#each $invitations as invite}
  <InvitationAlert {invite} />
{/each}
