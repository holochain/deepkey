import { authorizeClient, locallyAuthorizeClient, setupHolochain } from '$lib/holochain-client';
import {
	AppAgentWebsocket,
	CellType,
	type CellId,
	type SigningCredentials
} from '@holochain/client';
import { asyncDerived, lateInitLoadable } from './loadable';
import { browser } from '$app/environment';

const HOLOCHAIN_APP_ID = 'deepkey';
// const HOLOCHAIN_URL = new URL(`ws://localhost:${import.meta.env.VITE_HC_PORT}`);

export type SelfAuthorizedHolochainWebsocket = AppAgentWebsocket & {
	cellId: CellId;
	creds: SigningCredentials | undefined;
};

export const holochain = lateInitLoadable(async () => {
	const client = await setupHolochain();

	const appInfo = await client.appInfo();
	// eslint-disable-next-line @typescript-eslint/ban-ts-comment
	// @ts-ignore
	const { cell_id: cellId } = appInfo.cell_info[HOLOCHAIN_APP_ID][0][CellType.Provisioned];
	await authorizeClient(cellId);

	return client;
});

// Self-Authorizing, Self-Signing Zome Calls:
export const selfAuthenticatedHolochain = lateInitLoadable(async () => {
	const client = await setupHolochain();
	const appInfo = await client.appInfo();
	// eslint-disable-next-line @typescript-eslint/ban-ts-comment
	// @ts-ignore
	const { cell_id: cellId } = appInfo.cell_info[HOLOCHAIN_APP_ID][0][CellType.Provisioned];
	const creds = await locallyAuthorizeClient(cellId);
	// Just store these right on the client.
	// If we were communicating with multiple Zomes, we'd need this to be indexed by the Zome.
	// e.g. client.cell['deepkey']; client.creds['deepkey']
	// But we're just doing this for deepkey right now.
	(client as SelfAuthorizedHolochainWebsocket).cellId = cellId;
	(client as SelfAuthorizedHolochainWebsocket).creds = creds;
	return client as SelfAuthorizedHolochainWebsocket;
});

export const appInfo = asyncDerived([holochain], async ([$holochain]) => {
	return await $holochain.appInfo();
});

export const cellId = asyncDerived([appInfo], async ([$appInfo]) => {
	// eslint-disable-next-line @typescript-eslint/ban-ts-comment
	// @ts-ignore
	const { cell_id } = $appInfo.cell_info[HOLOCHAIN_APP_ID][0][CellType.Provisioned];
	return cell_id;
});

export const deepkeyAgentPubkey = asyncDerived(
	[appInfo],
	async ([$appInfo]) => $appInfo.agent_pub_key
);

if (browser) {
	holochain.init();
}
