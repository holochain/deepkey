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

test("create a KeysetRoot entry", async (t) => {
  await runScenario(async (scenario) => {
    const testDnaPath =
      process.cwd() + "/" + "../dnas/deepkey/workdir/deepkey.dna"
    const dnas: DnaSource[] = [{ path: testDnaPath }]

    const [laptop, server] = await scenario.addPlayersWithHapps([dnas, dnas])

    await scenario.shareAllAgents()

    const [keysetRootHash, changeRuleHash]: ActionHash[] =
      await laptop.cells[0].callZome({
        zome_name: "deepkey",
        fn_name: "create_keyset_root",
      })

    // Alice creates a first_entry
    const deviceInviteAcceptance: Record = await laptop.cells[0].callZome({
      zome_name: "deepkey",
      fn_name: "invite_agent",
      payload: laptop.agentPubKey,
    })
    console.log(deviceInviteAcceptance);
    
    expect(deviceInviteAcceptance).toBeTruthy();
    
    const inviteAcceptanceHash = await server.cells[0].callZome({
      zome_name: "deepkey",
      fn_name: "accept_invite",
      payload: deviceInviteAcceptance,
    })
    console.log(inviteAcceptanceHash);
  })
})

test.skip("create and read first_entry", async (t) => {
  await runScenario(async (scenario) => {
    // Construct proper paths for your app.
    // This assumes app bundle created by the `hc app pack` command.
    const testDnaPath = process.cwd() + "/" + "../dnas/first/workdir/first.dna"

    // Set up the array of DNAs to be installed, which only consists of the
    // test DNA referenced by path.
    const dnas: DnaSource[] = [{ path: testDnaPath }]

    // Add 2 players with the test DNA to the Scenario. The returned players
    // can be destructured.
    const [alice, bob] = await scenario.addPlayersWithHapps([dnas, dnas])

    // Shortcut peer discovery through gossip and register all agents in every
    // conductor of the scenario.
    await scenario.shareAllAgents()

    const createInput: any = {
      name: "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed nec eros quis enim hendrerit aliquet.",
      description:
        "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed nec eros quis enim hendrerit aliquet.",
      age: 68,
    }

    // Alice creates a first_entry
    const record: Record = await alice.cells[0].callZome({
      zome_name: "first_zome",
      fn_name: "create_first_entry",
      payload: createInput,
    })
    // assert.ok(record)
    expect(record).toBeTruthy()

    // Wait for the created entry to be propagated to the other node.
    await pause(300)

    // Bob gets the created first_entry
    const createReadOutput: Record = await bob.cells[0].callZome({
      zome_name: "first_zome",
      fn_name: "get_first_entry",
      payload: record.signed_action.hashed.hash,
    })
    // assert.deepEqual(
    //   createInput,
    //   decode((createReadOutput.entry as any).Present.entry) as any
    // )
    expect(createInput).toEqual(
      decode((createReadOutput.entry as any).Present.entry) as any
    )
  })
})

test.skip("create and update first_entry", async (t) => {
  await runScenario(async (scenario) => {
    // Construct proper paths for your app.
    // This assumes app bundle created by the `hc app pack` command.
    const testDnaPath = process.cwd() + "/" + "../dnas/first/workdir/first.dna"

    // Set up the array of DNAs to be installed, which only consists of the
    // test DNA referenced by path.
    const dnas: DnaSource[] = [{ path: testDnaPath }]

    // Add 2 players with the test DNA to the Scenario. The returned players
    // can be destructured.
    const [alice, bob] = await scenario.addPlayersWithHapps([dnas, dnas])

    // Shortcut peer discovery through gossip and register all agents in every
    // conductor of the scenario.
    await scenario.shareAllAgents()

    const createInput = {
      name: "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed nec eros quis enim hendrerit aliquet.",
      description:
        "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed nec eros quis enim hendrerit aliquet.",
      age: 1,
    }

    // Alice creates a first_entry
    const record: Record = await alice.cells[0].callZome({
      zome_name: "first_zome",
      fn_name: "create_first_entry",
      payload: createInput,
    })
    // assert.ok(record)
    expect(record).toBeTruthy()

    const originalActionHash = record.signed_action.hashed.hash

    // Alice updates the first_entry
    let contentUpdate: any = {
      name: "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed nec eros quis enim hendrerit aliquet.",
      description:
        "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed nec eros quis enim hendrerit aliquet.",
      age: 21,
    }
    let updateInput = {
      original_first_entry_hash: originalActionHash,
      previous_first_entry_hash: originalActionHash,
      updated_first_entry: contentUpdate,
    }

    let updatedRecord: Record = await alice.cells[0].callZome({
      zome_name: "first_zome",
      fn_name: "update_first_entry",
      payload: updateInput,
    })
    // assert.ok(updatedRecord)
    expect(updatedRecord).toBeTruthy()

    // Wait for the updated entry to be propagated to the other node.
    await pause(300)

    // Bob gets the updated first_entry
    const readUpdatedOutput0: Record = await bob.cells[0].callZome({
      zome_name: "first_zome",
      fn_name: "get_first_entry",
      payload: updatedRecord.signed_action.hashed.hash,
    })
    // assert.deepEqual(
    //   contentUpdate,
    //   decode((readUpdatedOutput0.entry as any).Present.entry) as any
    // )
    expect(contentUpdate).toEqual(
      decode((readUpdatedOutput0.entry as any).Present.entry) as any
    )

    // Alice updates the first_entry again
    contentUpdate = {
      name: "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed nec eros quis enim hendrerit aliquet.",
      description:
        "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed nec eros quis enim hendrerit aliquet.",
      age: 88,
    }
    updateInput = {
      original_first_entry_hash: originalActionHash,
      previous_first_entry_hash: updatedRecord.signed_action.hashed.hash,
      updated_first_entry: contentUpdate,
    }

    updatedRecord = await alice.cells[0].callZome({
      zome_name: "first_zome",
      fn_name: "update_first_entry",
      payload: updateInput,
    })
    // assert.ok(updatedRecord)
    expect(updatedRecord).toBeTruthy()

    // Wait for the updated entry to be propagated to the other node.
    await pause(300)

    // Bob gets the updated first_entry
    const readUpdatedOutput1: Record = await bob.cells[0].callZome({
      zome_name: "first_zome",
      fn_name: "get_first_entry",
      payload: updatedRecord.signed_action.hashed.hash,
    })
    // assert.deepEqual(
    //   contentUpdate,
    //   decode((readUpdatedOutput1.entry as any).Present.entry) as any
    // )
    expect(contentUpdate).toEqual(
      decode((readUpdatedOutput1.entry as any).Present.entry) as any
    )
  })
})

test.skip("create and delete first_entry", async (t) => {
  await runScenario(async (scenario) => {
    // Construct proper paths for your app.
    // This assumes app bundle created by the `hc app pack` command.
    const testDnaPath = process.cwd() + "/" + "../dnas/first/workdir/first.dna"

    // Set up the array of DNAs to be installed, which only consists of the
    // test DNA referenced by path.
    const dnas: DnaSource[] = [{ path: testDnaPath }]

    // Add 2 players with the test DNA to the Scenario. The returned players
    // can be destructured.
    const [alice, bob] = await scenario.addPlayersWithHapps([dnas, dnas])

    // Shortcut peer discovery through gossip and register all agents in every
    // conductor of the scenario.
    await scenario.shareAllAgents()

    const createInput = {
      name: "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed nec eros quis enim hendrerit aliquet.",
      description:
        "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed nec eros quis enim hendrerit aliquet.",
      age: 3,
    }

    // Alice creates a first_entry
    const record: Record = await alice.cells[0].callZome({
      zome_name: "first_zome",
      fn_name: "create_first_entry",
      payload: createInput,
    })
    // assert.ok(record)
    expect(record).toBeTruthy()

    // Alice deletes the first_entry
    const deleteActionHash = await alice.cells[0].callZome({
      zome_name: "first_zome",
      fn_name: "delete_first_entry",
      payload: record.signed_action.hashed.hash,
    })
    // assert.ok(deleteActionHash)
    expect(deleteActionHash).toBeTruthy()

    // Wait for the entry deletion to be propagated to the other node.
    await pause(300)

    // Bob tries to get the deleted first_entry
    const readDeletedOutput = await bob.cells[0].callZome({
      zome_name: "first_zome",
      fn_name: "get_first_entry",
      payload: record.signed_action.hashed.hash,
    })
    // assert.equal(readDeletedOutput, undefined)
    expect(readDeletedOutput).toBeNull()
  })
})
