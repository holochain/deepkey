import { DeepkeyClient } from '$lib/deepkey-client';
import { holochain } from './holochain-client-store';
import { asyncDerived } from './loadable';

const app_role = 'deepkey';

export const deepkey = asyncDerived([holochain.load], async ([$holochain]) => {
	const deepkey = new DeepkeyClient($holochain, app_role);
	return deepkey;
});

export const keysetRoot = asyncDerived([deepkey.load], async ([$deepkey]) => {
	return await $deepkey.queryKeysetRoot();
});

export const keysetMembers = asyncDerived(
	[deepkey.load, keysetRoot.load] as const,
	async ([$deepkey, $keysetRoot]) => {
		return await $deepkey.queryKeysetMembers($keysetRoot);
	}
);

// Messages:

// unsubscribe = deepkeyClient.on((data: any) => {
//   if (data.type === 'InvitationReceived') {
//     const dia = data.device_invite_acceptance;
//     $messages = [...$messages, { id: nanoid(), type: 'device_invite_acceptance', bytes: dia }];
//   }
// });
