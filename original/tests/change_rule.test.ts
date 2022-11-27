import { pause, runScenario, Scenario } from "@holochain/tryorama"
import path from "path"
// import test from "tape-promise/tape"
import { describe, expect, test } from "vitest"
import { ActionHash, DnaSource } from "@holochain/client"

import { dirname } from "node:path"
import { fileURLToPath } from "node:url"

export const loadHcFile = (partialDir) =>
  path.join(__dirname, `../dist/${partialDir}`)

import { Base64 } from "js-base64"

function deserializeHash(hash: string): Uint8Array {
  return Base64.toUint8Array(hash.slice(1))
}

function serializeHash(hash: Uint8Array): string {
  return `u${Base64.fromUint8Array(hash, true)}`
}

describe("change rule", () => {
  test("Create 2 players and create and read an entry", async (t) => {
    await runScenario(async (scenario: Scenario) => {
      try {
        // Construct proper paths for your DNAs.
        // This assumes DNA files created by the `hc dna pack` command.
        const testDnaPath =
          dirname(fileURLToPath(import.meta.url)) + "/../dist/deepkey.dna"

        // Set up the array of DNAs to be installed, which only consists of the
        // test DNA referenced by path.
        const dnas: DnaSource[] = [{ path: testDnaPath }]

        // Add 2 players with the test DNA to the Scenario. The returned players
        // can be destructured.
        const [alice, bob] = await scenario.addPlayersWithHapps([dnas, dnas])

        // Shortcut peer discovery through gossip and register all agents in every
        // conductor of the scenario.
        await scenario.shareAllAgents()

        // Content to be passed to the zome function that create an entry,
        const content = "Hello Tryorama"

        // The cells of the installed hApp are returned in the same order as the DNAs
        // that were passed into the player creation.
        const createEntryHash: ActionHash = await alice.cells[0].callZome({
          zome_name: "coordinator",
          fn_name: "create",
          payload: content,
        })

        // Wait for the created entry to be propagated to the other node.
        await pause(100)

        // Using the same cell and zome as before, the second player reads the
        // created entry.
        const readContent: typeof content = await bob.cells[0].callZome({
          zome_name: "coordinator",
          fn_name: "read",
          payload: createEntryHash,
        })
        // t.equal(readContent, content);
      } catch (error) {
        console.error("error occurred during test", error)
        throw error
      }
    })
  })
  test.skip("changerule1", async (t) => {
    await runScenario(async (scenario) => {
      try {
        const alice = await scenario.addConductor()
        const alicePubKey = await alice.adminWs().generateAgentPubKey()

        const deepkeyDnaHash = await alice.adminWs().registerDna({
          path: loadHcFile("deepkey.dna"),
          // properties: {
          //   progenitor: serializeHash(alicePubKey),
          // },
        })

        await alice.adminWs().installApp({
          agent_key: alicePubKey,
          dnas: [
            {
              hash: deepkeyDnaHash,
              role_id: "",
            },
          ],
          installed_app_id: "deepkey",
        })
        await alice.adminWs().enableApp({
          installed_app_id: "deepkey",
        })

        await scenario.shareAllAgents()
      } catch (error) {
        console.error("error occurred during test", error)
        throw error
      }
      /*
      const progenitor = await alice.appWs().callZome({
        cap_secret: null,
        cell_id: [lobbyDnaHash, alicePubKey],
        fn_name: "progenitor",
        payload: null,
        provenance: alicePubKey,
        zome_name: "private_publication_lobby",
      })
      t.equal(serializeHash(alicePubKey), serializeHash(progenitor))

      await alice.appWs().callZome({
        cap_secret: null,
        cell_id: [privatePublicationDnaHash, alicePubKey],
        fn_name: "create_post",
        payload: {
          title: "Post 1",
          content: "Posts post",
        },
        provenance: alicePubKey,
        zome_name: "posts",
      })

      let allPosts = await alice.appWs().callZome({
        cap_secret: null,
        cell_id: [lobbyDnaHash, alicePubKey],
        fn_name: "request_read_all_posts",
        payload: null,
        provenance: alicePubKey,
        zome_name: "private_publication_lobby",
      })
      t.equal(allPosts.length, 1)

      try {
        const allPosts: any = await bob.appWs().callZome({
          cap_secret: null,
          cell_id: [lobbyDnaHash, bobPubKey],
          fn_name: "read_all_posts",
          payload: null,
          provenance: bobPubKey,
          zome_name: "private_publication_lobby",
        })
        t.ok(false)
      } catch (e) {
        t.ok(true)
      }

      const secret = await alice.appWs().callZome({
        cap_secret: null,
        cell_id: [lobbyDnaHash, alicePubKey],
        fn_name: "grant_capability_to_read",
        payload: bobPubKey,
        provenance: alicePubKey,
        zome_name: "private_publication_lobby",
      })

      await bob.appWs().callZome({
        cap_secret: null,
        cell_id: [lobbyDnaHash, bobPubKey],
        fn_name: "store_capability_claim",
        payload: secret,
        provenance: bobPubKey,
        zome_name: "private_publication_lobby",
      })

      allPosts = await bob.appWs().callZome({
        cap_secret: null,
        cell_id: [lobbyDnaHash, bobPubKey],
        fn_name: "read_all_posts",
        payload: null,
        provenance: bobPubKey,
        zome_name: "private_publication_lobby",
      })
      t.equal(allPosts.length, 1)
      */
    })
  })
})
