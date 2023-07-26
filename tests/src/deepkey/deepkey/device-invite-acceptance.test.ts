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

import { deepkeyZomeCall, isPresent } from "../../utils.js"

const DNA_PATH = process.cwd() + "/../workdir/deepkey.happ"

test("invite an agent, and have them accept the invite", async (t) => {
  await runScenario(async (scenario) => {
    try {
      const appSource = { appBundleSource: { path: DNA_PATH } }

      const [alice, bob] = await scenario.addPlayersWithApps([
        appSource,
        appSource,
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
    } catch (e) {
      throw e.data.data
    }
  })
})
