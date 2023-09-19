import { type AppInfo, AdminWebsocket, CellType, AppAgentWebsocket } from '@holochain/client';

export const HOLOCHAIN_APP_ID = 'deepkey';
export const HOLOCHAIN_URL = `ws://localhost:${import.meta.env.VITE_HC_PORT}`;

export const setupHolochain = async () => {
	try {
		const client = await AppAgentWebsocket.connect(HOLOCHAIN_URL, HOLOCHAIN_APP_ID, 60000);

		if (typeof window === 'object' && !('__HC_LAUNCHER_ENV__' in window)) {
			const appInfo = await client.appInfo();
			await authorizeClient(appInfo);
		}

		return client;
	} catch (e) {
		console.log('Holochain client setup error', e);
		throw e;
	}
};

// set up zome call signing when run outside of launcher
export const authorizeClient = async (appInfo: AppInfo) => {
	if (typeof window === 'object' && !('__HC_LAUNCHER_ENV__' in window)) {
		// eslint-disable-next-line @typescript-eslint/ban-ts-comment
		// @ts-ignore
		const { cell_id } = appInfo.cell_info[HOLOCHAIN_APP_ID][0][CellType.Provisioned];
		const adminWs = await AdminWebsocket.connect(
			`ws://localhost:${import.meta.env.VITE_HC_ADMIN_PORT}`
		);
		await adminWs.authorizeSigningCredentials(cell_id);
		console.log('Holochain app client authorized for zome calls');
	}
};

// export function roleNameForCellId(appInfo: AppInfo, cellId: CellId): string | undefined {
// 	for (const [role, cells] of Object.entries(appInfo.cell_info)) {
// 		for (const c of cells) {
// 			if (CellType.Provisioned in c) {
// 				if (c[CellType.Provisioned].cell_id.toString() === cellId.toString()) {
// 					return role;
// 				}
// 			} else if (CellType.Cloned in c) {
// 				return c[CellType.Cloned].clone_id ? c[CellType.Cloned].clone_id : role;
// 			}
// 		}
// 	}
// 	return undefined;
// }

// export async function isSignalFromCellWithRole(
// 	client: AppAgentClient,
// 	roleName: string,
// 	signal: AppSignal
// ): Promise<boolean> {
// 	const appInfo = await client.appInfo();
// 	const role = roleNameForCellId(appInfo, signal.cell_id);

// 	return roleName === role;
// }
