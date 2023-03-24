import { describe, expect, test } from "vitest"

import { runScenario, pause, CallableCell } from "@holochain/tryorama"
import {
  NewEntryAction,
  ActionHash,
  Record,
  AppBundleSource,
} from "@holochain/client"
import { decode } from "@msgpack/msgpack"

// export async function createKeysetRoot(cell: CallableCell): Promise<Record[]> {
//   return cell.callZome({
//     zome_name: "deepkey",
//     fn_name: "create_keyset_root",
//   })
// }

test.skip("create KeysetRoot", async (t) => {
  await runScenario(async (scenario) => {
    // Construct proper paths for your app.
    // This assumes app bundle created by the `hc app pack` command.
    const testAppPath = process.cwd() + "/../workdir/deepkey.happ"

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

    // TODO: Creating a Keyset Root manually should fail! Because the only KSR that can be created
    // Should be created upon initialization.
    // const records: Record[] = await createKeysetRoot(alice.cells[0])
    // expect(records.length).toBe(2)
    // expect(records[0]).toBeTruthy()
    // expect(records[1]).toBeTruthy()
  })
})

test.skip("create and read KeysetRoot", async (t) => {
  await runScenario(async (scenario) => {
    // Construct proper paths for your app.
    // This assumes app bundle created by the `hc app pack` command.
    const testAppPath = process.cwd() + "/../workdir/deepkey.happ"

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

    // Alice creates a KeysetRoot
    // const records: Record[] = await createKeysetRoot(alice.cells[0])
    // expect(records[0]).toBeTruthy()

    // Wait for the created entry to be propagated to the other node.
    await pause(1200)

    // Bob gets the created KeysetRoot
    // const createReadOutput: Record = await bob.cells[0].callZome({
    //   zome_name: "deepkey",
    //   fn_name: "get_keyset_root",
    //   payload: records[0].signed_action.hashed.hash,
    // })

    // const original_keyset = decode(
    //   (records[0].entry as any).Present.entry
    // ) as any
    // expect(original_keyset).toEqual(
    //   decode((createReadOutput.entry as any).Present.entry) as any
    // )
  })
})
