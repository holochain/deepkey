import { Entry, RecordEntry } from "@holochain/client";
import { Player, getZomeCaller } from "@holochain/tryorama"

export function deepkeyZomeCall(actor: Player) {
  return getZomeCaller(actor.cells[0], "deepkey")
}

export function isPresent(recordEntry: RecordEntry): recordEntry is { Present: Entry } {
  return 'Present' in recordEntry;
}
