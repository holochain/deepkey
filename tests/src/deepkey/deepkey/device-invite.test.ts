import { describe, expect, test } from "vitest"

import { runScenario, pause, CallableCell } from "@holochain/tryorama"
import {
  NewEntryAction,
  ActionHash,
  Record,
  AppBundleSource,
} from "@holochain/client"
import { decode } from "@msgpack/msgpack"

export async function inviteAgent(
  cell: CallableCell,
  agentPubKey: Uint8Array
): Promise<Record> {
  return cell.callZome({
    zome_name: "deepkey",
    fn_name: "invite_agent",
    payload: agentPubKey,
  })
}

test("invite an agent", async (t) => {
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

    // Alice creates a DeviceInvite
    const inviteAcceptance: Record = await inviteAgent(
      alice.cells[0],
      bob.agentPubKey
    )

    // Get an inviteAcceptance object from the invitor
    // Pull & decode the invite from the inviteAcceptance
    // Verify the signature is valid, the keyset_root_authority is the KSR of the invitor,
    // and
    // console.log(inviteAcceptance)
    const inviteHash = (inviteAcceptance as any).invite
    console.log(inviteHash)
  })
})

test.skip("create and read DeviceInvite", async (t) => {
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

    const sample = await sampleDeviceInvite(alice.cells[0])

    // Alice creates a DeviceInvite
    const record: Record = await createDeviceInvite(alice.cells[0], sample)
    expect(record).toBeTruthy()

    // Wait for the created entry to be propagated to the other node.
    await pause(1200)

    // Bob gets the created DeviceInvite
    const createReadOutput: Record = await bob.cells[0].callZome({
      zome_name: "deepkey",
      fn_name: "get_device_invite",
      payload: record.signed_action.hashed.hash,
    })
    expect(sample).toEqual(
      decode((createReadOutput.entry as any).Present.entry) as any
    )
  })
})
