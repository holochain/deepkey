import test from 'node:test';
import assert from 'node:assert';

import { runScenario, pause, CallableCell } from '@holochain/tryorama';
import { NewEntryAction, ActionHash, Record, AppBundleSource } from '@holochain/client';
import { decode } from '@msgpack/msgpack';

import { createKeysetRoot } from './keyset-root.test.js';

async function sampleDeviceInvite(cell: CallableCell, partialDeviceInvite = {}) {
    return {
        ...{
          keyset_root: (await createKeysetRoot(cell)).signed_action.hashed.hash,
	  parent: ,
          invitee: cell.cell_id[1],
        },
        ...partialDeviceInvite
    };
}

export async function createDeviceInvite(cell: CallableCell, deviceInvite = undefined): Promise<Record> {
    return cell.callZome({
      zome_name: "deepkey",
      fn_name: "create_device_invite",
      payload: deviceInvite || await sampleDeviceInvite(cell),
    });
}

test('create DeviceInvite', { concurrency: 1 }, async t => {
  await runScenario(async scenario => {
    // Construct proper paths for your app.
    // This assumes app bundle created by the `hc app pack` command.
    const testAppPath = process.cwd() + '/../workdir/dk-scaffold.happ';

    // Set up the app to be installed 
    const appSource = { appBundleSource: { path: testAppPath } };

    // Add 2 players with the test app to the Scenario. The returned players
    // can be destructured.
    const [alice, bob] = await scenario.addPlayersWithApps([appSource, appSource]);

    // Shortcut peer discovery through gossip and register all agents in every
    // conductor of the scenario.
    await scenario.shareAllAgents();

    // Alice creates a DeviceInvite
    const record: Record = await createDeviceInvite(alice.cells[0]);
    assert.ok(record);
  });
});

test('create and read DeviceInvite', { concurrency: 1 }, async t => {
  await runScenario(async scenario => {
    // Construct proper paths for your app.
    // This assumes app bundle created by the `hc app pack` command.
    const testAppPath = process.cwd() + '/../workdir/dk-scaffold.happ';

    // Set up the app to be installed 
    const appSource = { appBundleSource: { path: testAppPath } };

    // Add 2 players with the test app to the Scenario. The returned players
    // can be destructured.
    const [alice, bob] = await scenario.addPlayersWithApps([appSource, appSource]);

    // Shortcut peer discovery through gossip and register all agents in every
    // conductor of the scenario.
    await scenario.shareAllAgents();

    const sample = await sampleDeviceInvite(alice.cells[0]);

    // Alice creates a DeviceInvite
    const record: Record = await createDeviceInvite(alice.cells[0], sample);
    assert.ok(record);

    // Wait for the created entry to be propagated to the other node.
    await pause(1200);

    // Bob gets the created DeviceInvite
    const createReadOutput: Record = await bob.cells[0].callZome({
      zome_name: "deepkey",
      fn_name: "get_device_invite",
      payload: record.signed_action.hashed.hash,
    });
    assert.deepEqual(sample, decode((createReadOutput.entry as any).Present.entry) as any);
  });
});


