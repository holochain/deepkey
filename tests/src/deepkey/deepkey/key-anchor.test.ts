import { assert, expect, test } from "vitest";

import { runScenario, pause, CallableCell } from "@holochain/tryorama";
import {
  NewEntryAction,
  ActionHash,
  Record,
  AppBundleSource,
  fakeActionHash,
  fakeAgentPubKey,
  fakeEntryHash,
  Entry,
  AgentPubKey,
  getSigningCredentials,
  generateSigningKeyPair,
  encodeHashToBase64,
  HoloHash,
  HoloHashed,
} from "@holochain/client";
import * as msgpack from "@msgpack/msgpack";

import { createKeyAnchor, sampleKeyAnchor } from "./common.js";
import { deepkeyZomeCall, isPresent } from "../../utils.js";
import * as ed25519 from "@noble/ed25519";

const DNA_PATH = process.cwd() + "/../workdir/deepkey.happ";
``;

type KeyAnchor = Uint8Array;
export function getKeyAnchor(pubkey: AgentPubKey): KeyAnchor {
  return pubkey.slice(4, 36);
}

type Authorization = [number, Buffer]; // u8 index, 64 byte signature

type KeyGeneration = {
  new_key: AgentPubKey;
  new_key_signing_of_author: Buffer; // 64 bytes
};

type KeyRevocation = {
  prior_key_registration: ActionHash;
  revocation_authorization: Authorization[];
};

type KeyRegistration =
  | {
      Create: KeyGeneration;
    }
  | {
      CreateOnly: KeyGeneration;
    }
  | {
      Update: [KeyRevocation, KeyGeneration];
    }
  | {
      Delete: KeyRevocation;
    };

type KeyState =
  | {
      NotFound: null;
    }
  | {
      Invalidated: { hashed: HoloHashed<KeyRegistration> };
    }
  | {
      Valid: { hashed: HoloHashed<KeyRegistration> };
    };

test("key_state", async (t) => {
  await runScenario(async (scenario) => {
    try {
      const appSource = { appBundleSource: { path: DNA_PATH } };

      const [alice, bob] = await scenario.addPlayersWithApps([
        appSource,
        appSource,
      ]);

      await scenario.shareAllAgents();

      // Create keypair
      const [{ privateKey: testPrivKey, publicKey: testPubKey }, _] =
        await generateSigningKeyPair();
      const testKey = bob.agentPubKey; // TODO: this should be a KeyAnchor

      let keyState: KeyState;

      keyState = await deepkeyZomeCall(alice)("key_state", [
        testKey,
        Date.now(),
      ]);
      // console.log(keyState);
      expect(keyState).toHaveProperty("NotFound");

      // Register the key
      const signature = await ed25519.signAsync(testKey, testPrivKey);

      const keyRegistration = {
        Create: {
          new_key: testKey,
          new_key_signing_of_author: Buffer.from(signature),
        },
      };

      const registration = await deepkeyZomeCall(alice)("register_test_key");

      const regHash: HoloHash = await deepkeyZomeCall(alice)(
        "new_key_registration",
        keyRegistration
      );

      keyState = await deepkeyZomeCall(alice)("key_state", [
        testKey,
        Date.now(),
      ]);
      expect(keyState).toHaveProperty("Valid");
      if (!("Valid" in keyState)) throw "invalid";
      // The key state action is the KeyAnchor.
      // The KeyRegistration is saved in the previous Action.
      const keyAnchorHash = keyState.Valid.hashed.hash;
      const registrationHash = (keyState.Valid.hashed.content as any).prev_action;

      // const regRecord: any = await deepkeyZomeCall(alice)("get_key_registration", registrationHash);
      // console.log(msgpack.decode(regRecord.entry.Present.entry));

      // Revoke the key
      const revocation: KeyRevocation = await deepkeyZomeCall(alice)(
        "authorize_key_revocation",
        {
          prior_key_registration: registrationHash,
          revocation_authorization: [],
        }
      );
      await deepkeyZomeCall(alice)("revoke_key", revocation);
      keyState = await deepkeyZomeCall(alice)("key_state", [
        testKey,
        Date.now(),
      ]);
      expect(keyState).toHaveProperty("Invalidated");

    } catch (e) {
      console.log(e);
      throw e.data;
    }
  });
});

test.skip("create KeyAnchor", async () => {
  await runScenario(async (scenario) => {
    // Construct proper paths for your app.
    // This assumes app bundle created by the `hc app pack` command.
    const testAppPath = process.cwd() + "/../workdir/dk-scaffold.happ";

    // Set up the app to be installed
    const appSource = { appBundleSource: { path: testAppPath } };

    // Add 2 players with the test app to the Scenario. The returned players
    // can be destructured.
    const [alice, bob] = await scenario.addPlayersWithApps([
      appSource,
      appSource,
    ]);

    // Shortcut peer discovery through gossip and register all agents in every
    // conductor of the scenario.
    await scenario.shareAllAgents();

    // Alice creates a KeyAnchor
    const record: Record = await createKeyAnchor(alice.cells[0]);
    assert.ok(record);
  });
});

test.skip("create and read KeyAnchor", async () => {
  await runScenario(async (scenario) => {
    // Construct proper paths for your app.
    // This assumes app bundle created by the `hc app pack` command.
    const testAppPath = process.cwd() + "/../workdir/dk-scaffold.happ";

    // Set up the app to be installed
    const appSource = { appBundleSource: { path: testAppPath } };

    // Add 2 players with the test app to the Scenario. The returned players
    // can be destructured.
    const [alice, bob] = await scenario.addPlayersWithApps([
      appSource,
      appSource,
    ]);

    // Shortcut peer discovery through gossip and register all agents in every
    // conductor of the scenario.
    await scenario.shareAllAgents();

    const sample = await sampleKeyAnchor(alice.cells[0]);

    // Alice creates a KeyAnchor
    const record: Record = await createKeyAnchor(alice.cells[0], sample);
    assert.ok(record);

    // Wait for the created entry to be propagated to the other node.
    await pause(1200);

    // Bob gets the created KeyAnchor
    const createReadOutput: Record = await bob.cells[0].callZome({
      zome_name: "deepkey",
      fn_name: "get_key_anchor",
      payload: record.signed_action.hashed.hash,
    });
    assert.deepEqual(
      sample,
      decode((createReadOutput.entry as any).Present.entry) as any
    );
  });
});

test.skip("create and update KeyAnchor", async () => {
  await runScenario(async (scenario) => {
    // Construct proper paths for your app.
    // This assumes app bundle created by the `hc app pack` command.
    const testAppPath = process.cwd() + "/../workdir/dk-scaffold.happ";

    // Set up the app to be installed
    const appSource = { appBundleSource: { path: testAppPath } };

    // Add 2 players with the test app to the Scenario. The returned players
    // can be destructured.
    const [alice, bob] = await scenario.addPlayersWithApps([
      appSource,
      appSource,
    ]);

    // Shortcut peer discovery through gossip and register all agents in every
    // conductor of the scenario.
    await scenario.shareAllAgents();

    // Alice creates a KeyAnchor
    const record: Record = await createKeyAnchor(alice.cells[0]);
    assert.ok(record);

    const originalActionHash = record.signed_action.hashed.hash;

    // Alice updates the KeyAnchor
    let contentUpdate: any = await sampleKeyAnchor(alice.cells[0]);
    let updateInput = {
      previous_key_anchor_hash: originalActionHash,
      updated_key_anchor: contentUpdate,
    };

    let updatedRecord: Record = await alice.cells[0].callZome({
      zome_name: "deepkey",
      fn_name: "update_key_anchor",
      payload: updateInput,
    });
    assert.ok(updatedRecord);

    // Wait for the updated entry to be propagated to the other node.
    await pause(1200);

    // Bob gets the updated KeyAnchor
    const readUpdatedOutput0: Record = await bob.cells[0].callZome({
      zome_name: "deepkey",
      fn_name: "get_key_anchor",
      payload: updatedRecord.signed_action.hashed.hash,
    });
    assert.deepEqual(
      contentUpdate,
      decode((readUpdatedOutput0.entry as any).Present.entry) as any
    );

    // Alice updates the KeyAnchor again
    contentUpdate = await sampleKeyAnchor(alice.cells[0]);
    updateInput = {
      previous_key_anchor_hash: updatedRecord.signed_action.hashed.hash,
      updated_key_anchor: contentUpdate,
    };

    updatedRecord = await alice.cells[0].callZome({
      zome_name: "deepkey",
      fn_name: "update_key_anchor",
      payload: updateInput,
    });
    assert.ok(updatedRecord);

    // Wait for the updated entry to be propagated to the other node.
    await pause(1200);

    // Bob gets the updated KeyAnchor
    const readUpdatedOutput1: Record = await bob.cells[0].callZome({
      zome_name: "deepkey",
      fn_name: "get_key_anchor",
      payload: updatedRecord.signed_action.hashed.hash,
    });
    assert.deepEqual(
      contentUpdate,
      decode((readUpdatedOutput1.entry as any).Present.entry) as any
    );
  });
});

test.skip("create and delete KeyAnchor", async () => {
  await runScenario(async (scenario) => {
    // Construct proper paths for your app.
    // This assumes app bundle created by the `hc app pack` command.
    const testAppPath = process.cwd() + "/../workdir/dk-scaffold.happ";

    // Set up the app to be installed
    const appSource = { appBundleSource: { path: testAppPath } };

    // Add 2 players with the test app to the Scenario. The returned players
    // can be destructured.
    const [alice, bob] = await scenario.addPlayersWithApps([
      appSource,
      appSource,
    ]);

    // Shortcut peer discovery through gossip and register all agents in every
    // conductor of the scenario.
    await scenario.shareAllAgents();

    // Alice creates a KeyAnchor
    const record: Record = await createKeyAnchor(alice.cells[0]);
    assert.ok(record);

    // Alice deletes the KeyAnchor
    const deleteActionHash = await alice.cells[0].callZome({
      zome_name: "deepkey",
      fn_name: "delete_key_anchor",
      payload: record.signed_action.hashed.hash,
    });
    assert.ok(deleteActionHash);

    // Wait for the entry deletion to be propagated to the other node.
    await pause(1200);

    // Bob tries to get the deleted KeyAnchor
    const readDeletedOutput = await bob.cells[0].callZome({
      zome_name: "deepkey",
      fn_name: "get_key_anchor",
      payload: record.signed_action.hashed.hash,
    });
    assert.notOk(readDeletedOutput);
  });
});
