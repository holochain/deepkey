import { authorizeClient, setupHolochain } from '$lib/holochain-client';
import {
	AppAgentWebsocket,
	CellType,
	type CellId,
	type SigningCredentials
} from '@holochain/client';
import { asyncDerived, lateInitLoadable } from './loadable';

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
	const creds = await authorizeClient(cellId);

	// Just store these right on the client.
	// If we were communicating with multiple Zomes, we'd need this to be indexed by the Zome.
	// e.g. client.cell['deepkey']; client.creds['deepkey']
	// But we're just doing this for deepkey right now.
	(client as SelfAuthorizedHolochainWebsocket).cellId = cellId;
	(client as SelfAuthorizedHolochainWebsocket).creds = creds;
	return client as SelfAuthorizedHolochainWebsocket;
});

export const appInfo = asyncDerived([holochain.load], async ([$holochain]) => {
	return await $holochain.appInfo();
});

export const deepkeyAgentPubkey = asyncDerived(
	[appInfo.load],
	async ([$appInfo]) => $appInfo.agent_pub_key
);
