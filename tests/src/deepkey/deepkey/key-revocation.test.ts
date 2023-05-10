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

type KeyRegistration = {
  new_key: Buffer
  new_key_signing_of_author: Buffer
}

type KeyRevocation = {
  prior_key_registration: ActionHash
  revoction_authorization: []
}

test("revoke key registration", async (t) => {
  await runScenario(async (scenario) => {
    try {
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

      const newKeyToRegister = await fakeAgentPubKey()
      // // First get the KeyGeneration, with valid signature of the new key from the Deepkey chain agent
      const keyGeneration = await deepkeyZomeCall(alice)<KeyGeneration>(
        "instantiate_key_generation",
        newKeyToRegister
      )
      const keyReg1Action = await deepkeyZomeCall(alice)<ActionHash>(
        "new_key_registration",
        { Create: { ...keyGeneration } }
      )
      let keyRevocation = await deepkeyZomeCall(
        alice
      )<any>("instantiate_key_revocation", keyReg1Action)

      keyRevocation = await deepkeyZomeCall(alice)<any>(
        "authorize_key_revocation",
        keyRevocation
      )
      console.log(keyRevocation)
      const keyReg2Action = await deepkeyZomeCall(alice)<ActionHash>(
        "create_key_revocation_record",
        keyRevocation
      )
      // const keyReg = await deepkeyZomeCall(alice)<Record>(
      //   "get_key_registration_from_key_anchor",
      //   newKeyToRegister
      // )
      // expect(isPresent(keyReg.entry))
    } catch (e) {
      console.error(e)
      throw e.data.data
    }
  })
})
