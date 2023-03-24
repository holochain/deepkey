import { describe, expect, test } from "vitest"

import { runScenario, pause, CallableCell } from "@holochain/tryorama"
import {
  NewEntryAction,
  ActionHash,
  Record,
  AppBundleSource,
} from "@holochain/client"
import { decode } from "@msgpack/msgpack"
// import { ActionHash, AgentPubKey, HoloHash } from "@whi/holo-hash"
// import { Holochain } from "@whi/holochain-backdrop"
// import { ConductorError } from "@whi/holochain-client"

const DNA_PATH = process.cwd() + "/../workdir/deepkey.happ"

export async function inviteAgent(
  cell: CallableCell,
  agentPubKey: Uint8Array
): Promise<any> {
  return cell.callZome({
    zome_name: "deepkey",
    fn_name: "invite_agent",
    payload: agentPubKey,
  })
}

export async function getDeviceInvite(
  cell: CallableCell,
  deviceInviteHash: ActionHash
): Promise<Record> {
  return cell.callZome({
    zome_name: "deepkey",
    fn_name: "get_device_invite",
    payload: deviceInviteHash,
  })
}

test("invite an agent", async (t) => {
  // const holochain = new Holochain({
  //   timeout: 60_000,
  //   default_stdout_loggers: process.env.LOG_LEVEL === "silly",
  // })

  // const actors = await holochain.backdrop({
  //   deepkey: {
  //     deepkey: DNA_PATH,
  //   },
  // })
  // let whoami = await actors.alice.call(
  //   "appstore",
  //   "appstore_api",
  //   "whoami",
  //   null,
  //   30_000
  // )
  // let s = String(new HoloHash(whoami.agent_initial_pubkey))
  // console.log(`Alice whoami: {s}`)

  // await runScenario(async (scenario) => {
  //   const appSource = { appBundleSource: { path: DNA_PATH } }

  //   // Add 2 players with the test app to the Scenario. The returned players
  //   // can be destructured.
  //   const [alice, bob] = await scenario.addPlayersWithApps([
  //     appSource,
  //     appSource,
  //   ])

  //   // Shortcut peer discovery through gossip and register all agents in every
  //   // conductor of the scenario.
  //   await scenario.shareAllAgents()

  //   // Alice creates a DeviceInvite
  //   const inviteAcceptance: Record = await inviteAgent(
  //     alice.cells[0],
  //     bob.agentPubKey
  //   )
  //   // await pause(1200)

  //   const inviteHash = (inviteAcceptance as any).invite
  //   const inviteRecord = await getDeviceInvite(alice.cells[0], inviteHash)
  //   const invite = decode((inviteRecord.entry as any).Present.entry) as any
  //   // console.log(invite)
  //   expect(invite.invitee).toEqual(bob.agentPubKey)
  //   // Verify the signature is valid, the keyset_root_authority is the KSR of the invitor
  //   // TODO: How to get the KSR of the invitor?
  // })
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
