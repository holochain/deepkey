import { DeviceInviteAcceptance } from "./../../../../ui/src/deepkey/deepkey/types";
import { describe, expect, test } from "vitest";
import * as ed25519 from "@noble/ed25519";
import { runScenario, pause } from "@holochain/tryorama";
import {
  NewEntryAction,
  ActionHash,
  Record,
  AppBundleSource,
  AgentPubKey,
  Signature,
  fakeAgentPubKey,
  Entry,
  generateSigningKeyPair,
} from "@holochain/client";
import { decode, encode } from "@msgpack/msgpack";

import { deepkeyZomeCall, isPresent } from "../../utils.js";
import { KeyGeneration, KeyRegistration } from "../../../../ui/deepkey";

const DNA_PATH = process.cwd() + "/../workdir/deepkey.happ";

test("new_key_registration", async (t) => {
  await runScenario(async (scenario) => {
    try {
      const appSource = { appBundleSource: { path: DNA_PATH } };

      const [alice, bob] = await scenario.addPlayersWithApps([
        appSource,
        appSource,
      ]);

      await scenario.shareAllAgents();

      const [keypair, newKeyToRegister] = await generateSigningKeyPair();
      // Sign the KeyGeneration with the new key
      const keyGeneration: KeyGeneration = {
        new_key: newKeyToRegister,
        new_key_signing_of_author: await ed25519.signAsync(
          newKeyToRegister,
          keypair.privateKey
        ),
      };
      // The enum Create option from the KeyRegistration options.
      const keyRegistration = {
        Create: {
          ...keyGeneration,
        },
      };

      // Register the new key
      await deepkeyZomeCall(alice)<null>(
        "new_key_registration",
        keyRegistration
      );

      // Retrieve the KeyRegistration via its KeyAnchor
      const createdKeyRegistrationRecord = await deepkeyZomeCall(alice)<Record>(
        "get_key_registration_from_agent_pubkey_key_anchor",
        newKeyToRegister
      );

      expect(isPresent(createdKeyRegistrationRecord.entry));

      const createdKeyRegistrationEntry = (
        createdKeyRegistrationRecord.entry as { Present: Entry }
      ).Present.entry;
      const storedKeyRegistration = decode(
        createdKeyRegistrationEntry as Uint8Array
      ) as KeyRegistration;
      expect(storedKeyRegistration.Create.new_key).toEqual(
        Buffer.from(keyRegistration.Create.new_key)
      );
    } catch (e) {
      throw e;
    }
  });
});

test("update key registration", async (t) => {
  await runScenario(async (scenario) => {
    try {
      const appSource = { appBundleSource: { path: DNA_PATH } };

      const [alice, bob] = await scenario.addPlayersWithApps([
        appSource,
        appSource,
      ]);
      await scenario.shareAllAgents();

      const sysTime = Math.floor(Date.now() * 1000);

      const [existingKeypair, existingPubkey] = await generateSigningKeyPair();
      // Sign the KeyGeneration with the new key
      const keyGeneration: KeyGeneration = {
        new_key: existingPubkey,
        new_key_signing_of_author: await ed25519.signAsync(
          existingPubkey,
          existingKeypair.privateKey
        ),
      };

      const [nextKeypair, nextPubkey] = await generateSigningKeyPair();
      // Sign the KeyGeneration with the new key
      const nextKeyGeneration: KeyGeneration = {
        new_key: nextPubkey,
        new_key_signing_of_author: await ed25519.signAsync(
          nextPubkey,
          nextKeypair.privateKey
        ),
      };

      const keyReg1Action = await deepkeyZomeCall(alice)<ActionHash>(
        "new_key_registration",
        { Create: { ...keyGeneration } }
      );
      const keyAnchorRecord = await deepkeyZomeCall(alice)<any>(
        "get_agent_pubkey_key_anchor",
        existingPubkey
      );
      const keyAnchor = (decode(keyAnchorRecord.entry.Present.entry) as any)
        .bytes;

      let keyRevocation = await deepkeyZomeCall(alice)<any>(
        "instantiate_key_revocation",
        keyReg1Action
      );
      keyRevocation = await deepkeyZomeCall(alice)<any>(
        "authorize_key_revocation",
        keyRevocation
      );

      // key anchor query: key should be valid
      const keyStateBefore = await deepkeyZomeCall(alice)("key_state", [
        keyAnchor,
        sysTime,
      ]);
      expect(keyStateBefore).toHaveProperty("Valid");

      await deepkeyZomeCall(alice)<ActionHash>("update_key", [
        keyRevocation,
        nextKeyGeneration,
      ]);

      const keyStateAfter = await deepkeyZomeCall(alice)("key_state", [
        keyAnchor,
        sysTime,
      ]);
      expect(keyStateAfter).toHaveProperty("Invalidated");
    } catch (e) {
      console.error(e);
      throw e;
    }
  });
});
