// Take in a Uint8Array, return a value that can be used as an index in a hash

const utf8decoder = new TextDecoder(); // default 'utf-8' or 'utf8'
const textEncoder = new TextEncoder();

export function indexableKey(key: Uint8Array) {
	return utf8decoder.decode(key);
}

// Reverse the above function, getting the original Uint8Arrat back.
export function getUint8ArrayFromString(str: string) {
	return textEncoder.encode(str);
}
