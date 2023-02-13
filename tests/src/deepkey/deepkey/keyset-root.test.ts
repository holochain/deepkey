import test from 'node:test';
import assert from 'node:assert';

import { runScenario, pause, CallableCell } from '@holochain/tryorama';
import { NewEntryAction, ActionHash, Record, AppBundleSource } from '@holochain/client';
import { decode } from '@msgpack/msgpack';


async function sampleKeysetRoot(cell: CallableCell, partialKeysetRoot = {}) {
    return {
        ...{
	  first_deepkey_agent: "",
	  root_pub_key: "",
	  fda_pubkey_signed_by_root_key: "Lorem ipsum dolor sit amet, consectetur adipiscing elit.",
        },
        ...partialKeysetRoot
    };
}

export async function createKeysetRoot(cell: CallableCell, keysetRoot = undefined): Promise<Record> {
    return cell.callZome({
      zome_name: "deepkey",
      fn_name: "create_keyset_root",
      payload: keysetRoot || await sampleKeysetRoot(cell),
    });
}

test('create KeysetRoot', { concurrency: 1 }, async t => {
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

    // Alice creates a KeysetRoot
    const record: Record = await createKeysetRoot(alice.cells[0]);
    assert.ok(record);
  });
});

test('create and read KeysetRoot', { concurrency: 1 }, async t => {
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

    const sample = await sampleKeysetRoot(alice.cells[0]);

    // Alice creates a KeysetRoot
    const record: Record = await createKeysetRoot(alice.cells[0], sample);
    assert.ok(record);

    // Wait for the created entry to be propagated to the other node.
    await pause(1200);

    // Bob gets the created KeysetRoot
    const createReadOutput: Record = await bob.cells[0].callZome({
      zome_name: "deepkey",
      fn_name: "get_keyset_root",
      payload: record.signed_action.hashed.hash,
    });
    assert.deepEqual(sample, decode((createReadOutput.entry as any).Present.entry) as any);
  });
});


