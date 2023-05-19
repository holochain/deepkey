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
import type { KeyGeneration } from "../../../../ui/deepkey.d.ts";

const DNA_PATH = process.cwd() + "/../workdir/deepkey.happ";

test("revoke key registration", async (t) => {
  await runScenario(async (scenario) => {
    try {
      const appSource = { appBundleSource: { path: DNA_PATH } };

      const [alice, bob] = await scenario.addPlayersWithApps([
        appSource,
        appSource,
      ]);

      // await Promise.all([
      //   deepkeyZomeCall(alice)("create_keyset_root"),
      //   deepkeyZomeCall(bob)("create_keyset_root"),
      // ]);

      await scenario.shareAllAgents();

      const sysTime = Math.floor(Date.now() * 1000);

      const [keypair, keyToRevoke] = await generateSigningKeyPair();
      // Sign the KeyGeneration with the new key
      const keyGeneration: KeyGeneration = {
        new_key: keyToRevoke,
        new_key_signing_of_author: await ed25519.signAsync(
          keyToRevoke,
          keypair.privateKey
        ),
      };

      const keyReg1Action = await deepkeyZomeCall(alice)<ActionHash>(
        "new_key_registration",
        { Create: { ...keyGeneration } }
      );
      const keyAnchorRecord = await deepkeyZomeCall(alice)<any>(
        "get_agent_pubkey_key_anchor",
        keyToRevoke
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
      await deepkeyZomeCall(alice)<ActionHash>("revoke_key", keyRevocation);

      // scenario.awaitDhtSync(alice.cells[0].cell_id)

      // key anchor query: key revoked
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
