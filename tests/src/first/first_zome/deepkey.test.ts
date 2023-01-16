// import test from 'node:test';
// import assert from "node:assert"

import { runScenario, pause } from "@holochain/tryorama"
import {
  NewEntryAction,
  ActionHash,
  Record,
  DnaSource,
} from "@holochain/client"
import { decode } from "@msgpack/msgpack"

import { describe, expect, test } from "vitest"
const deepkeyDnaPath =
  process.cwd() + "/" + "../dnas/deepkey/workdir/deepkey.dna"
const dnas: DnaSource[] = [{ path: deepkeyDnaPath }]

// helpers
const base64 = (str: Uint8Array) => Buffer.from(str).toString("base64")

test("create a KeysetRoot entry", async (t) => {
  await runScenario(async (scenario) => {
    const [laptop, server] = await scenario.addPlayersWithHapps([dnas, dnas])
    await scenario.shareAllAgents()

    // Create the keyset root
    const [keysetRootHash, changeRuleHash]: ActionHash[] =
      await laptop.cells[0].callZome({
        zome_name: "deepkey",
        fn_name: "create_keyset_root",
      })

    // // Pre-generate invitation for invitee to write to their chain
    // const deviceInviteAcceptance: Record = await laptop.cells[0].callZome({
    //   zome_name: "deepkey",
    //   fn_name: "invite_agent",
    //   payload: laptop.agentPubKey,
    // })
    // expect(deviceInviteAcceptance).toBeTruthy()

    // // Write the invitate acceptance onto the invitee's chain
    // const inviteAcceptanceHash = await server.cells[0].callZome({
    //   zome_name: "deepkey",
    //   fn_name: "accept_invite",
    //   payload: deviceInviteAcceptance,
    // })
    // expect(inviteAcceptanceHash).toBeTruthy()

})

test.skip("Try to create two keyset roots", async (t) => {
  await runScenario(async (scenario) => {
    const [laptop] = await scenario.addPlayersWithHapps([dnas])
    await scenario.shareAllAgents()

    // Create the keyset root
    const [keysetRootHash, changeRuleHash]: ActionHash[] =
      await laptop.cells[0].callZome({
        zome_name: "deepkey",
        fn_name: "create_keyset_root",
      })
    console.log(base64(keysetRootHash))

    const [keysetRootHash2, changeRuleHash2]: ActionHash[] =
      await laptop.cells[0].callZome({
        zome_name: "deepkey",
        fn_name: "create_keyset_root",
      })

    console.log(base64(keysetRootHash2))
    // TODO: This should fail! Write the validation
  })
})
