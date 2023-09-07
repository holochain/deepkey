import { describe, expect, test } from "vitest";

import {
  runScenario,
  pause,
  CallableCell,
  Player,
  getZomeCaller,
} from "@holochain/tryorama";
import {
  ActionHash,
  Record,
  Entry,
  fakeAgentPubKey,
  generateSigningKeyPair,
  AgentPubKey,
} from "@holochain/client";
import { decode, encode } from "@msgpack/msgpack";

import { base64, deepkeyZomeCall, isPresent } from "../../utils.js";
import * as ed25519 from "@noble/ed25519";

const DNA_PATH = process.cwd() + "/../workdir/deepkey.happ";

// So, we should have a place that we're referencing Typescript types from
// Accessible to our UI code, and accessible to our tests.
// Pulled this from deepkey-client.ts
type KeyAnchor = { bytes: Uint8Array };
export function getKeyAnchor(pubkey: AgentPubKey): KeyAnchor {
  return { bytes: pubkey.slice(3, 35) };
}

test("keyset_root: query_keyset_members and query_keyset_keys", async (t) => {
  await runScenario(async (scenario) => {
    try {
      const appSource = { appBundleSource: { path: DNA_PATH } };

      const [alice, bob] = await scenario.addPlayersWithApps([
        appSource,
        appSource,
      ]);

      await scenario.shareAllAgents();

      // Invite Bob into Alice's Keyset
      const inviteAcceptance = await deepkeyZomeCall(alice)(
        "invite_agent",
        bob.agentPubKey
      );
      await pause(500);

      const acceptanceHash = await deepkeyZomeCall(bob)(
        "accept_invite",
        inviteAcceptance
      );

      await pause(500);

      const aliceKeysetRoot = await deepkeyZomeCall(alice)(
        "query_keyset_authority_action_hash"
      );

      const keysetMembers = await deepkeyZomeCall(alice)(
        "query_keyset_members",
        aliceKeysetRoot
      );
      // console.log("keysetMembers", keysetMembers);
      expect(keysetMembers).toEqual([bob.agentPubKey, alice.agentPubKey]);

      // Register a key
      const [keypair, newKeyToRegister] = await generateSigningKeyPair();
      await registerKey(alice, keypair.privateKey, newKeyToRegister);
      await pause(500);

      let key_anchors: KeyAnchor[] = await deepkeyZomeCall(alice)(
        "query_keyset_keys",
        aliceKeysetRoot
      );
      expect(base64KeyAnchors(key_anchors)).toEqual(
        base64KeyAnchors([getKeyAnchor(newKeyToRegister)])
      );
      // Bob should be able to see this.
      key_anchors = await deepkeyZomeCall(bob)(
        "query_keyset_keys",
        aliceKeysetRoot
      );

      // TODO: This isn't working, but it should be. Does the DHT need more time to sync?
      // The link KeysetRootToKeyAnchors is not being propagated across the DHT somehow.

      // expect(base64KeyAnchors(key_anchors), "Bob can't see key registered on Alice").toEqual(
      //   base64KeyAnchors([getKeyAnchor(newKeyToRegister)])
      // );

      // Now let's register on Bob's side
      const [keypair2, newKeyToRegister2] = await generateSigningKeyPair();
      await registerKey(bob, keypair2.privateKey, newKeyToRegister2);
      await pause(500);

      const aliceKeyAnchors: KeyAnchor[] = await deepkeyZomeCall(alice)(
        "query_keyset_keys",
        aliceKeysetRoot
      );
      const bobKeyAnchors: KeyAnchor[] = await deepkeyZomeCall(bob)(
        "query_keyset_keys",
        aliceKeysetRoot
      );
      console.log("aliceKeyAnchors", aliceKeyAnchors);
      console.log("bobKeyAnchors", bobKeyAnchors);

      // expect(base64KeyAnchors(aliceKeyAnchors)).toEqual(
      //   base64KeyAnchors(bobKeyAnchors)
      // );
      // expect(key_anchors.map((ka) => base64(ka.bytes))).toEqual(
      //   [newKeyToRegister, newKeyToRegister2].map((key_bytes) =>
      //     base64(getKeyAnchor(key_bytes).bytes)
      //   )
      // );
    } catch (e) {
      console.log("error", e);
      throw e.data;
    }
  });
});

function base64KeyAnchors(keyAnchors: KeyAnchor[]): string[] {
  return keyAnchors.map((ka) => base64(ka.bytes));
}

async function registerKey(agent, privKey, agentPubKey) {
  // Sign the KeyGeneration with the new key
  const keyGeneration = {
    new_key: agentPubKey,
    new_key_signing_of_author: await ed25519.signAsync(agentPubKey, privKey),
  };
  // The enum Create option from the KeyRegistration options.
  const keyRegistration = {
    Create: {
      ...keyGeneration,
    },
  };

  // Register the new key
  await deepkeyZomeCall(agent)<null>("new_key_registration", keyRegistration);
}
