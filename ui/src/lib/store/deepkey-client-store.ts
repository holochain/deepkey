import { DeepkeyClient } from '$lib/deepkey-client';
import { asyncDerived } from '@square/svelte-store';
import { holochain } from './holochain-client-store';

const app_role = 'deepkey';

export const deepkey = asyncDerived(holochain, async ($holochain) => {
	const hc = await $holochain;
	if (hc) {
		return new DeepkeyClient(hc, app_role);
	} else {
		throw 'Holochain client not defined.';
	}
});

export const keysetRootAuthority = asyncDerived(deepkey, async ($deepkey) => {
	return await $deepkey.queryKeysetRoot();
});

export const keysetMembers = asyncDerived(
	[deepkey, keysetRootAuthority],
	async ([$deepkey, $keysetRootAuthority]) => {
		return await $deepkey.queryKeysetMembers($keysetRootAuthority);
	}
);

// Messages:

// unsubscribe = deepkeyClient.on((data: any) => {
//   if (data.type === 'InvitationReceived') {
//     const dia = data.device_invite_acceptance;
//     $messages = [...$messages, { id: nanoid(), type: 'device_invite_acceptance', bytes: dia }];
//   }
// });
