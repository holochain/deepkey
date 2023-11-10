// eslint-disable-next-line @typescript-eslint/ban-ts-comment
// x@ts-ignore
import MurmurHash3 from 'imurmurhash';

// Take in a Uint8Array, return a value that can be used as an index in a hash

const utf8decoder = new TextDecoder(); // default 'utf-8' or 'utf8'

export function indexableKey(key: Uint8Array) {
	return MurmurHash3(utf8decoder.decode(key)).result();
}
