/* eslint-disable @typescript-eslint/ban-ts-comment */

const randseed = new Array(4); // Xorshift: [x, y, z, w] 32 bit values

// @ts-ignore
export function seedrand(seed) {
	randseed.fill(0);

	for (let i = 0; i < seed.length; i++) {
		randseed[i % 4] = (randseed[i % 4] << 5) - randseed[i % 4] + seed.charCodeAt(i);
	}
}

export function rand() {
	// based on Java's String.hashCode(), expanded to 4 32bit values
	const t = randseed[0] ^ (randseed[0] << 11);

	randseed[0] = randseed[1];
	randseed[1] = randseed[2];
	randseed[2] = randseed[3];
	randseed[3] = randseed[3] ^ (randseed[3] >> 19) ^ t ^ (t >> 8);

	return (randseed[3] >>> 0) / ((1 << 31) >>> 0);
}

export function createColor() {
	//saturation is the whole color spectrum
	const h = Math.floor(rand() * 360);
	//saturation goes from 40 to 100, it avoids greyish colors
	const s = rand() * 60 + 40;
	//lightness can be anything from 0 to 100, but probabilities are a bell curve around 50%
	const l = (rand() + rand() + rand() + rand()) * 25;

	return { h, s, l };
}

// @ts-ignore
export function mixColors(a, b, amt) {
	return {
		h: a.h * (1 - amt) + b.h * amt,
		s: a.s * (1 - amt) + b.s * amt,
		l: a.l * (1 - amt) + b.l * amt
	};
}

// @ts-ignore
export function encodeColor({ h, s, l }) {
	return `hsl(${h}, ${s}%, ${l}%)`;
}

// @ts-ignore
export function stringToBits(string) {
	return (
		// @ts-ignore
		Array.from(string)
			// @ts-ignore
			.reduce((acc, char) => acc.concat(char.charCodeAt().toString(2)), [])
			// @ts-ignore
			.map((bin) => '0'.repeat(8 - bin.length) + bin)
			.join('')
	);
}

// @ts-ignore
export function* getBitStream(string) {
	const bits = stringToBits(string);

	for (let i = 0; i < bits.length; i++) {
		yield parseInt(bits[i]);
	}
}
