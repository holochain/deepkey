import { describe, expect, test } from "vitest"

import {
  runScenario,
  pause,
  CallableCell,
  Player,
  getZomeCaller,
} from "@holochain/tryorama"
import {
  NewEntryAction,
  ActionHash,
  Record,
  AppBundleSource,
  Entry,
} from "@holochain/client"
import { decode, encode } from "@msgpack/msgpack"

import { inviteAgent } from "./device-invite.test.js"
import { deepkeyZomeCall, isPresent } from "../../utils.js"

const DNA_PATH = process.cwd() + "/../workdir/deepkey.happ"

test("invite an agent, and have them accept the invite", async (t) => {
  try {
    await runScenario(async (scenario) => {
      const appSource = { appBundleSource: { path: DNA_PATH } }

      const [alice, bob] = await scenario.addPlayersWithApps([
        appSource,
        appSource,
      ])

      await Promise.all([
        deepkeyZomeCall(alice)("create_keyset_root"),
        deepkeyZomeCall(bob)("create_keyset_root"),
      ])

      await scenario.shareAllAgents()

      const inviteAcceptance = await deepkeyZomeCall(alice)(
        "invite_agent",
        bob.agentPubKey
      )

      const acceptanceHash = await deepkeyZomeCall(bob)(
        "accept_invite",
        inviteAcceptance
      )
      const acceptanceRecord = await deepkeyZomeCall(bob)<Record>(
        "get_device_invite_acceptance",
        acceptanceHash
      )

      expect(isPresent(acceptanceRecord.entry)).toBeTruthy()

      const acceptanceEntry = (acceptanceRecord.entry as { Present: Entry })
        .Present.entry
      const storedAcceptance = decode(acceptanceEntry as Uint8Array)

      expect(storedAcceptance).toEqual(inviteAcceptance)
    })
  } catch (e) {
    throw e.data.data
  }
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
