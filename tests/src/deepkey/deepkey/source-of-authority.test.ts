import { describe, expect, test } from "vitest";

import {
  runScenario,
  pause,
  CallableCell,
  Player,
  getZomeCaller,
} from "@holochain/tryorama";
import {
  NewEntryAction,
  ActionHash,
  Record,
  AppBundleSource,
  Entry,
} from "@holochain/client";
import { decode, encode } from "@msgpack/msgpack";

import { base64, deepkeyZomeCall, isPresent } from "../../utils.js";
import { Base64 } from "js-base64";

const DNA_PATH = process.cwd() + "/../workdir/deepkey.happ";

test("query source of authority - keyset root", async (t) => {
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
      await pause(400);

      const acceptanceHash = await deepkeyZomeCall(bob)(
        "accept_invite",
        inviteAcceptance
      );

      await pause(500);
      // Alice's Keyset Root is the source of authority for herself
      const aliceKeysetRoot = await deepkeyZomeCall(alice)(
        "query_keyset_authority_action_hash"
      );
      // console.log("aliceKeysetRoot", base64(aliceKeysetRoot));

      // TODO: test the keyset members querying
      // const aliceKeysetMembers = await deepkeyZomeCall(alice)(
      //   "query_keyset_members",
      //   aliceKeysetRoot
      // );
      // expect(aliceKeysetMembers).toEqual([alice.agentPubKey]);

      // Alice's Keyset Root is the source of authority for Bob's Keyset
      const bobKeysetRoot = await deepkeyZomeCall(bob)(
        "query_keyset_authority_action_hash"
      );
      // console.log("bobKeysetRoot", base64(bobKeysetRoot));

      // bobKeysetRoot is the same as aliceKeysetRoot
      expect(base64(bobKeysetRoot)).toEqual(base64(aliceKeysetRoot));
    } catch (e) {
      throw e.data;
    }
  });
});
