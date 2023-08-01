import { Entry, RecordEntry } from "@holochain/client";
import { Player, getZomeCaller } from "@holochain/tryorama";
import { Base64 } from "js-base64";

export function deepkeyZomeCall(actor: Player) {
  return getZomeCaller(actor.cells[0], "deepkey");
}

export function isPresent(
  recordEntry: RecordEntry
): recordEntry is { Present: Entry } {
  return "Present" in recordEntry;
}

export function base64(buf: unknown): string {
  return Base64.fromUint8Array(new Uint8Array(buf as Buffer));
}
