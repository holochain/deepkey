import { Player, getZomeCaller } from "@holochain/tryorama"

export function deepkeyZomeCall(actor: Player) {
  return getZomeCaller(actor.cells[0], "deepkey")
}
