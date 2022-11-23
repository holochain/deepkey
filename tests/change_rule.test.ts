import { pause, runScenario } from "@holochain/tryorama"
import path from "path"
import test from "tape-promise/tape"

export const loadHcFile = (partialDir) =>
  path.join(__dirname, `../dist/partialDir`)

import { Base64 } from "js-base64"

export function deserializeHash(hash: string): Uint8Array {
  return Base64.toUint8Array(hash.slice(1))
}

export function serializeHash(hash: Uint8Array): string {
  return `u${Base64.fromUint8Array(hash, true)}`
}

export default () =>
  test("keyset root", async (t) => {
    await runScenario(async (scenario) => {
      const alice = await scenario.addConductor()
      const alicePubKey = await alice.adminWs().generateAgentPubKey()

      const keysetRootDnaHash = await alice.adminWs().registerDna({
        path: loadHcFile("keyset_root.dna"),
        // properties: {
        //   progenitor: serializeHash(alicePubKey),
        // },
      })

      await alice.adminWs().installApp({
        agent_key: alicePubKey,
        dnas: [
          {
            hash: keysetRootDnaHash,
            role_id: "",
          },
        ],
        installed_app_id: "deepkey",
      })
      await alice.adminWs().enableApp({
        installed_app_id: "deepkey",
      })

      await scenario.shareAllAgents()
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
