import * as ed from '@noble/ed25519';
import {
	type AppInfo,
	AdminWebsocket,
	CellType,
	AppAgentWebsocket,
	type CapSecret,
	type AgentPubKey,
	type CellId,
	type GrantedFunctions,
	GrantedFunctionsType,
	type KeyPair,
	type CallZomeRequest,
	type CallZomeRequestUnsigned,
	randomNonce,
	getNonceExpiration,
	hashZomeCall,
	type CallZomeRequestSigned,
	type SigningCredentials
} from '@holochain/client';
import { encode } from '@msgpack/msgpack';
import type { Sign } from 'crypto';

export const HOLOCHAIN_APP_ID = 'deepkey';
export const HOLOCHAIN_URL = new URL(`ws://localhost:${import.meta.env.VITE_HC_PORT}`);

export const setupHolochain = async () => {
	try {
		const client = await AppAgentWebsocket.connect(HOLOCHAIN_URL, HOLOCHAIN_APP_ID, 60000);

		// if (typeof window === 'object' && !('__HC_LAUNCHER_ENV__' in window)) {
		// 	const appInfo = await client.appInfo();
		// 	await authorizeClient(appInfo);
		// }

		return client;
	} catch (e) {
		console.error('Holochain client setup error', e);
		throw e;
	}
};

// set up zome call signing when run outside of launcher
export const authorizeClient = async (cellId: CellId) => {
	if (typeof window === 'object' && !('__HC_LAUNCHER_ENV__' in window)) {
		// eslint-disable-next-line @typescript-eslint/ban-ts-comment
		// @ts-ignore
		// const { cell_id: cellId } = appInfo.cell_info[HOLOCHAIN_APP_ID][0][CellType.Provisioned];
		const adminWs = await AdminWebsocket.connect(
			new URL(`ws://localhost:${import.meta.env.VITE_HC_ADMIN_PORT}`)
		);

		// await adminWs.authorizeSigningCredentials(cellId);
		const creds = await locallyAuthorizeSigningCredentials(adminWs, cellId);

		// console.log('Holochain app client authorized for zome calls');
		return creds;
	}
};

const locallyGenerateSigningKeyPair: (
	agentPubKey?: AgentPubKey
) => Promise<[KeyPair, AgentPubKey]> = async (agentPubKey?: AgentPubKey) => {
	const privateKey = ed.utils.randomPrivateKey();
	const publicKey = await ed.getPublicKeyAsync(privateKey); // Sync methods below
	const keyPair = { publicKey, privateKey };

	const locationBytes = agentPubKey ? agentPubKey.subarray(35) : [0, 0, 0, 0];
	const signingKey = new Uint8Array(
		[132, 32, 36].concat(...keyPair.publicKey).concat(...locationBytes)
	);
	return [keyPair, signingKey];
};

// Return signing creds instead of storing in the js client
export const locallyAuthorizeSigningCredentials = async (
	adminWs: AdminWebsocket,
	cellId: CellId,
	functions?: GrantedFunctions,
) => {
	const [keyPair, signingKey] = await locallyGenerateSigningKeyPair();
	const capSecret = await adminWs.grantSigningKey(
		cellId,
		functions || { [GrantedFunctionsType.All]: null },
		signingKey
	);
	// setSigningCredentials(cellId, { capSecret, keyPair, signingKey });
	return { capSecret, keyPair, signingKey };
};

export const locallySignZomeCall = async (
	request: CallZomeRequest,
	signingCredentialsForCell: SigningCredentials
) => {
	const unsignedZomeCallPayload: CallZomeRequestUnsigned = {
		cap_secret: signingCredentialsForCell.capSecret,
		cell_id: request.cell_id,
		zome_name: request.zome_name,
		fn_name: request.fn_name,
		provenance: signingCredentialsForCell.signingKey,
		payload: encode(request.payload),
		nonce: await randomNonce(),
		expires_at: getNonceExpiration()
	};
	const hashedZomeCall = await hashZomeCall(unsignedZomeCallPayload);
	const signature = await ed.signAsync(
		hashedZomeCall,
		signingCredentialsForCell.keyPair.privateKey
	);

	const signedZomeCall: CallZomeRequestSigned = {
		...unsignedZomeCallPayload,
		signature
	};
	return signedZomeCall;
};
