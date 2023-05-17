import { describe, expect, test } from "vitest";

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
} from "@holochain/client";
import { decode, encode } from "@msgpack/msgpack";

import { deepkeyZomeCall, isPresent } from "../../utils.js";

const DNA_PATH = process.cwd() + "/../workdir/deepkey.happ";

type KeyGeneration = {
  new_key: AgentPubKey;
  new_key_signing_of_author: Signature;
};

type KeyRegistration = {
  new_key: Buffer;
  new_key_signing_of_author: Buffer;
};

type KeyRevocation = {
  prior_key_registration: ActionHash;
  revoction_authorization: [];
};

// pub enum KeyState {
//   NotFound,
//   Invalidated,
//   Valid,
// }

test("revoke key registration", async (t) => {
  await runScenario(async (scenario) => {
    try {
      const appSource = { appBundleSource: { path: DNA_PATH } };

      const [alice, bob] = await scenario.addPlayersWithApps([
        appSource,
        appSource,
      ]);

      await Promise.all([
        deepkeyZomeCall(alice)("create_keyset_root"),
        deepkeyZomeCall(bob)("create_keyset_root"),
      ]);

      await scenario.shareAllAgents();

      // What's the time?
      // const sysTime = await deepkeyZomeCall(alice)<number>("now", [])
      // console.log("sysTime", sysTime)
      // const sysTime = Buffer.alloc(8);
      // sysTime.writeBigInt64LE(BigInt(Math.floor(Date.now() * 1000)));
      const sysTime = Math.floor(Date.now() * 1000);

      const keyToRevoke = await fakeAgentPubKey();
      // First get the KeyGeneration, with valid signature of the new key from the Deepkey chain agent
      const keyGeneration = await deepkeyZomeCall(alice)<KeyGeneration>(
        "instantiate_key_generation",
        keyToRevoke
      );
      const keyReg1Action = await deepkeyZomeCall(alice)<ActionHash>(
        "new_key_registration",
        { Create: { ...keyGeneration } }
      );
      const keyAnchorRecord = await deepkeyZomeCall(alice)<any>(
        "get_agent_pubkey_key_anchor",
        keyToRevoke
      );
      // expect(keyAnchorRecord).toBeDefined()
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
      expect(keyStateBefore).toEqual({ Valid: null });

      await deepkeyZomeCall(alice)<ActionHash>("revoke_key", keyRevocation);

      // scenario.awaitDhtSync(alice.cells[0].cell_id)

      // key anchor query: key revoked
      // const keyStateAfter = await deepkeyZomeCall(alice)("key_state", [
      //   keyAnchor,
      //   sysTime,
      // ])
      // expect(keyStateAfter).toEqual({ Invalidated: null })

      // const keyReg = await deepkeyZomeCall(alice)<Record>(
      //   "get_key_registration_from_key_anchor",
      //   newKeyToRegister
      // )
      // expect(isPresent(keyReg.entry))
    } catch (e) {
      console.error(e);
      throw e;
    }
  });
});
