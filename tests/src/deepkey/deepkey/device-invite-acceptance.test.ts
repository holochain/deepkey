import { describe, expect, test } from "vitest"

import { runScenario, pause, CallableCell } from "@holochain/tryorama"
import {
  NewEntryAction,
  ActionHash,
  Record,
  AppBundleSource,
} from "@holochain/client"
import { decode } from "@msgpack/msgpack"

import { createDeviceInvite } from "./device-invite.test.js"

async function sampleDeviceInviteAcceptance(
  cell: CallableCell,
  partialDeviceInviteAcceptance = {}
) {
  return {
    ...{
      // keyset_root_authority: ,
      invite: (await createDeviceInvite(cell)).signed_action.hashed.hash,
    },
    ...partialDeviceInviteAcceptance,
  }
}

export async function createDeviceInviteAcceptance(
  cell: CallableCell,
  deviceInviteAcceptance = undefined
): Promise<Record> {
  return cell.callZome({
    zome_name: "deepkey",
    fn_name: "create_device_invite_acceptance",
    payload:
      deviceInviteAcceptance || (await sampleDeviceInviteAcceptance(cell)),
  })
}

test.skip("create DeviceInviteAcceptance", async (t) => {
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

    // Alice creates a DeviceInviteAcceptance
    const record: Record = await createDeviceInviteAcceptance(alice.cells[0])
    expect(record).toBeTruthy()
  })
})

test.skip("create and read DeviceInviteAcceptance", async (t) => {
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

    const sample = await sampleDeviceInviteAcceptance(alice.cells[0])

    // Alice creates a DeviceInviteAcceptance
    const record: Record = await createDeviceInviteAcceptance(
      alice.cells[0],
      sample
    )
    expect(record).toBeTruthy()

    // Wait for the created entry to be propagated to the other node.
    await pause(1200)

    // Bob gets the created DeviceInviteAcceptance
    const createReadOutput: Record = await bob.cells[0].callZome({
      zome_name: "deepkey",
      fn_name: "get_device_invite_acceptance",
      payload: record.signed_action.hashed.hash,
    })
    expect(sample).toEqual(
      decode((createReadOutput.entry as any).Present.entry) as any
    )
  })
})
