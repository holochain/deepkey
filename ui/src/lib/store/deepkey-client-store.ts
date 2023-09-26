import { DeepkeyClient } from '$lib/deepkey-client';
import { holochain, type SelfAuthorizedHolochainWebsocket } from './holochain-client-store';
import { asyncDerived } from './loadable';

const app_role = 'deepkey';

export const deepkey = asyncDerived<[SelfAuthorizedHolochainWebsocket], DeepkeyClient>(
	[holochain.load],
	async ([$holochain]) => {
		const deepkey = new DeepkeyClient($holochain, app_role);
		return deepkey;
	}
);

export const keysetRoot = asyncDerived([deepkey.load], async ([$deepkey]) => {
	return await $deepkey.queryKeysetRoot();
});

// export const keysetMembers = asyncDerived(
// 	[deepkey, keysetRootAuthority],
// 	async ([$deepkey, $keysetRootAuthority]) => {
// 		return await $deepkey.queryKeysetMembers($keysetRootAuthority);
// 	}
// );

// Messages:

// unsubscribe = deepkeyClient.on((data: any) => {
//   if (data.type === 'InvitationReceived') {
//     const dia = data.device_invite_acceptance;
//     $messages = [...$messages, { id: nanoid(), type: 'device_invite_acceptance', bytes: dia }];
//   }
// });
