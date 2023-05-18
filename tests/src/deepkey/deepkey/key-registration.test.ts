import { DeviceInviteAcceptance } from "./../../../../ui/src/deepkey/deepkey/types"
import { describe, expect, test } from "vitest"

import { runScenario, pause } from "@holochain/tryorama"
import {
  NewEntryAction,
  ActionHash,
  Record,
  AppBundleSource,
  AgentPubKey,
  Signature,
  fakeAgentPubKey,
  Entry,
} from "@holochain/client"
import { decode, encode } from "@msgpack/msgpack"

import { deepkeyZomeCall, isPresent } from "../../utils.js"

const DNA_PATH = process.cwd() + "/../workdir/deepkey.happ"

type KeyGeneration = {
  new_key: AgentPubKey
  new_key_signing_of_author: Signature
}

type KeyRegistration = {}

test("new_key_registration", async (t) => {
  await runScenario(async (scenario) => {
    try {
      const appSource = { appBundleSource: { path: DNA_PATH } }

      const [alice, bob] = await scenario.addPlayersWithApps([
        appSource,
        appSource,
      ])

      // await Promise.all([
      //   deepkeyZomeCall(alice)("create_keyset_root"),
      //   deepkeyZomeCall(bob)("create_keyset_root"),
      // ])

      await scenario.shareAllAgents()

      const newKeyToRegister = await fakeAgentPubKey()
      // First get the KeyGeneration, with valid signature of the new key from the Deepkey chain agent
      const keyGeneration = await deepkeyZomeCall(alice)<KeyGeneration>(
        "instantiate_key_generation",
        newKeyToRegister
      )
      // The enum Create option from the KeyRegistration options.
      const keyRegistration = {
        Create: {
          ...keyGeneration,
        },
      }

      // Register the new key
      await deepkeyZomeCall(alice)<null>(
        "new_key_registration",
        keyRegistration
      )

      // Retrieve the KeyRegistration via its KeyAnchor
      const createdKeyRegistrationRecord = await deepkeyZomeCall(alice)<Record>(
        "get_key_registration_from_key_anchor",
        newKeyToRegister
      )

      expect(isPresent(createdKeyRegistrationRecord.entry)).toBeTruthy()

      const createdKeyRegistrationEntry = (
        createdKeyRegistrationRecord.entry as { Present: Entry }
      ).Present.entry
      const storedKeyRegistration = decode(
        createdKeyRegistrationEntry as Uint8Array
      )
      expect(storedKeyRegistration).toEqual(keyRegistration)
    } catch (e) {
      throw e.data.data
    }
  })
})
