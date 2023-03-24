import { describe, expect, test } from "vitest"

import { runScenario, pause, CallableCell } from "@holochain/tryorama"
import {
  NewEntryAction,
  ActionHash,
  Record,
  AppBundleSource,
} from "@holochain/client"
import { decode } from "@msgpack/msgpack"

async function sampleAuthoritySpec(
  cell: CallableCell,
  partialAuthoritySpec = {}
) {
  return {
    ...{
      sigs_required: 10,
      signers: cell.cell_id[1],
    },
    ...partialAuthoritySpec,
  }
}

export async function createAuthoritySpec(
  cell: CallableCell,
  authoritySpec = undefined
): Promise<Record> {
  return cell.callZome({
    zome_name: "deepkey",
    fn_name: "create_authority_spec",
    payload: authoritySpec || (await sampleAuthoritySpec(cell)),
  })
}

test.skip("create AuthoritySpec", async (t) => {
  await runScenario(async (scenario) => {
    // Construct proper paths for your app.
    // This assumes app bundle created by the `hc app pack` command.
    const testAppPath = process.cwd() + "/../workdir/dk-scaffold.happ"

    // Set up the app to be installed
    const appSource = { appBundleSource: { path: testAppPath } }

    // Add 2 players with the test app to the Scenario. The returned players
    // can be destructured.
    const [alice, bob] = await scenario.addPlayersWithApps([
      appSource,
      appSource,
    ])

    // Shortcut peer discovery through gossip and register all agents in every
    // conductor of the scenario.
    await scenario.shareAllAgents()

    // Alice creates a AuthoritySpec
    const record: Record = await createAuthoritySpec(alice.cells[0])
    expect(record).toBeTruthy()
  })
})

test.skip("create and read AuthoritySpec", async (t) => {
  await runScenario(async (scenario) => {
    // Construct proper paths for your app.
    // This assumes app bundle created by the `hc app pack` command.
    const testAppPath = process.cwd() + "/../workdir/dk-scaffold.happ"

    // Set up the app to be installed
    const appSource = { appBundleSource: { path: testAppPath } }

    // Add 2 players with the test app to the Scenario. The returned players
    // can be destructured.
    const [alice, bob] = await scenario.addPlayersWithApps([
      appSource,
      appSource,
    ])

    // Shortcut peer discovery through gossip and register all agents in every
    // conductor of the scenario.
    await scenario.shareAllAgents()

    const sample = await sampleAuthoritySpec(alice.cells[0])

    // Alice creates a AuthoritySpec
    const record: Record = await createAuthoritySpec(alice.cells[0], sample)
    expect(record).toBeTruthy()

    // Wait for the created entry to be propagated to the other node.
    await pause(1200)

    // Bob gets the created AuthoritySpec
    const createReadOutput: Record = await bob.cells[0].callZome({
      zome_name: "deepkey",
      fn_name: "get_authority_spec",
      payload: record.signed_action.hashed.hash,
    })
    expect(sample).toEqual(
      decode((createReadOutput.entry as any).Present.entry) as any
    )
  })
})
