import test from 'node:test';
import assert from 'node:assert';

import { runScenario, pause, CallableCell } from '@holochain/tryorama';
import { NewEntryAction, ActionHash, Record, AppBundleSource } from '@holochain/client';
import { decode } from '@msgpack/msgpack';


async function sampleChangeRule(cell: CallableCell, partialChangeRule = {}) {
    return {
        ...{
	  keyset_root: ,
	  keyset_leaf: ,
	  spec_change: ,
        },
        ...partialChangeRule
    };
}

export async function createChangeRule(cell: CallableCell, changeRule = undefined): Promise<Record> {
    return cell.callZome({
      zome_name: "deepkey",
      fn_name: "create_change_rule",
      payload: changeRule || await sampleChangeRule(cell),
    });
}

test('create ChangeRule', { concurrency: 1 }, async t => {
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

    // Alice creates a ChangeRule
    const record: Record = await createChangeRule(alice.cells[0]);
    assert.ok(record);
  });
});

test('create and read ChangeRule', { concurrency: 1 }, async t => {
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

    const sample = await sampleChangeRule(alice.cells[0]);

    // Alice creates a ChangeRule
    const record: Record = await createChangeRule(alice.cells[0], sample);
    assert.ok(record);

    // Wait for the created entry to be propagated to the other node.
    await pause(1200);

    // Bob gets the created ChangeRule
    const createReadOutput: Record = await bob.cells[0].callZome({
      zome_name: "deepkey",
      fn_name: "get_change_rule",
      payload: record.signed_action.hashed.hash,
    });
    assert.deepEqual(sample, decode((createReadOutput.entry as any).Present.entry) as any);
  });
});

test('create and update ChangeRule', { concurrency: 1 }, async t => {
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

    // Alice creates a ChangeRule
    const record: Record = await createChangeRule(alice.cells[0]);
    assert.ok(record);
        
    const originalActionHash = record.signed_action.hashed.hash;
 
    // Alice updates the ChangeRule
    let contentUpdate: any = await sampleChangeRule(alice.cells[0]);
    let updateInput = {
      original_change_rule_hash: originalActionHash,
      previous_change_rule_hash: originalActionHash,
      updated_change_rule: contentUpdate,
    };

    let updatedRecord: Record = await alice.cells[0].callZome({
      zome_name: "deepkey",
      fn_name: "update_change_rule",
      payload: updateInput,
    });
    assert.ok(updatedRecord);

    // Wait for the updated entry to be propagated to the other node.
    await pause(1200);
        
    // Bob gets the updated ChangeRule
    const readUpdatedOutput0: Record = await bob.cells[0].callZome({
      zome_name: "deepkey",
      fn_name: "get_change_rule",
      payload: updatedRecord.signed_action.hashed.hash,
    });
    assert.deepEqual(contentUpdate, decode((readUpdatedOutput0.entry as any).Present.entry) as any);

    // Alice updates the ChangeRule again
    contentUpdate = await sampleChangeRule(alice.cells[0]);
    updateInput = { 
      original_change_rule_hash: originalActionHash,
      previous_change_rule_hash: updatedRecord.signed_action.hashed.hash,
      updated_change_rule: contentUpdate,
    };

    updatedRecord = await alice.cells[0].callZome({
      zome_name: "deepkey",
      fn_name: "update_change_rule",
      payload: updateInput,
    });
    assert.ok(updatedRecord);

    // Wait for the updated entry to be propagated to the other node.
    await pause(1200);
        
    // Bob gets the updated ChangeRule
    const readUpdatedOutput1: Record = await bob.cells[0].callZome({
      zome_name: "deepkey",
      fn_name: "get_change_rule",
      payload: updatedRecord.signed_action.hashed.hash,
    });
    assert.deepEqual(contentUpdate, decode((readUpdatedOutput1.entry as any).Present.entry) as any);
  });
});

