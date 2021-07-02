use crate::word::*;

#[inline]
fn sha2_u32(message: &[u8], iv: [u32; 8]) -> [Word32; 8] {
	//补位
	let padding = 64 - (message.len()+8)%64;
	let mut m: Vec<u8> = Vec::with_capacity(message.len() + padding + 8);
	m.extend_from_slice(message);
	m.push(0x80u8);
	m.extend(&vec![0; padding-1]);
	//增加长度
	m.extend_from_slice(&(message.len() as u64).wrapping_mul(8).to_be_bytes());
	//
	let m: &[Word32] = &Word32::slice_from_be_bytes(&m);
	//
	let mut a: Word32 = Word32::from(iv[0]);
	let mut b: Word32 = Word32::from(iv[1]);
	let mut c: Word32 = Word32::from(iv[2]);
	let mut d: Word32 = Word32::from(iv[3]);
	let mut e: Word32 = Word32::from(iv[4]);
	let mut f: Word32 = Word32::from(iv[5]);
	let mut g: Word32 = Word32::from(iv[6]);
	let mut h: Word32 = Word32::from(iv[7]);
	const K: [u32; 64] = [
		0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1, 0x923f82a4, 0xab1c5ed5,
		0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3, 0x72be5d74, 0x80deb1fe, 0x9bdc06a7, 0xc19bf174,
		0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc, 0x2de92c6f, 0x4a7484aa, 0x5cb0a9dc, 0x76f988da,
		0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7, 0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967,
		0x27b70a85, 0x2e1b2138, 0x4d2c6dfc, 0x53380d13, 0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85,
		0xa2bfe8a1, 0xa81a664b, 0xc24b8b70, 0xc76c51a3, 0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070,
		0x19a4c116, 0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f, 0x682e6ff3,
		0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208, 0x90befffa, 0xa4506ceb, 0xbef9a3f7, 0xc67178f2
	];
	for x in m.chunks_exact(16) {
		let (aa, bb, cc, dd, ee, ff, gg, hh) = (a, b, c, d, e, f, g, h);
		let w: [Word32; 64] = {
			let mut temp: [Word32; 64] = [Word32::from(0); 64];
			for i in 0..16 {
				temp[i] = x[i];
			}
			for i in 16..64 {
				let s0 = (temp[i-15] >>  7) ^ (temp[i-15] >> 18) ^ (temp[i-15] >>  "3");
				let s1 = (temp[i- 2] >> 17) ^ (temp[i- 2] >> 19) ^ (temp[i- 2] >> "10");
				temp[i] = temp[i-16] + s0 + temp[i-7] + s1;
			}
			temp
		};
		for i in 0..64 {
			let s1 = (e >> 6) ^ (e >> 11) ^ (e >> 25);
			let ch = (e & f) ^ (!e & g);
			let temp1 = h + s1 + ch + K[i] + w[i];
			let s0 = (a >> 2) ^ (a >> 13) ^ (a >> 22);
			let maj = (a & b) ^ (a & c) ^ (b & c);
			let temp2 = s0 + maj;
			h = g;
			g = f;
			f = e;
			e = d + temp1;
			d = c;
			c = b;
			b = a;
			a = temp1 + temp2;
		}
		//
		a = a + aa;
		b = b + bb;
		c = c + cc;
		d = d + dd;
		e = e + ee;
		f = f + ff;
		g = g + gg;
		h = h + hh;
	}
	//输出
	[a, b, c, d, e, f, g, h]
}

pub fn sha_224(message: &[u8]) -> [u8; 28] {
	let [a, b, c, d, e, f, g, _] = sha2_u32(message, [
		0xc1059ed8,
		0x367cd507,
		0x3070dd17,
		0xf70e5939,
		0xffc00b31,
		0x68581511,
		0x64f98fa7,
		0xbefa4fa4
	]);
	Word32::to_be_bytes_array([a, b, c, d, e, f, g])
}

pub fn sha_256(message: &[u8]) -> [u8; 32] {
	let [a, b, c, d, e, f, g, h] = sha2_u32(message, [
		0x6a09e667,
		0xbb67ae85,
		0x3c6ef372,
		0xa54ff53a,
		0x510e527f,
		0x9b05688c,
		0x1f83d9ab,
		0x5be0cd19
	]);
	Word32::to_be_bytes_array([a, b, c, d, e, f, g, h])
}

#[inline]
fn sha2_u64(message: &[u8], iv: [u64; 8]) -> [Word64; 8] {
	//补位
	let padding = 128 - (message.len()+16)%128;
	let mut m: Vec<u8> = Vec::with_capacity(message.len() + padding + 16);
	m.extend_from_slice(message);
	m.push(0x80u8);
	m.extend(&vec![0; padding-1]);
	//增加长度
	m.extend_from_slice(&(message.len() as u128).wrapping_mul(8).to_be_bytes());
	//
	let m: &[Word64] = &Word64::slice_from_be_bytes(&m);
	//
	let mut a: Word64 = Word64::from(iv[0]);
	let mut b: Word64 = Word64::from(iv[1]);
	let mut c: Word64 = Word64::from(iv[2]);
	let mut d: Word64 = Word64::from(iv[3]);
	let mut e: Word64 = Word64::from(iv[4]);
	let mut f: Word64 = Word64::from(iv[5]);
	let mut g: Word64 = Word64::from(iv[6]);
	let mut h: Word64 = Word64::from(iv[7]);
	const K: [u64; 80] = [
		0x428a2f98d728ae22, 0x7137449123ef65cd, 0xb5c0fbcfec4d3b2f, 0xe9b5dba58189dbbc,
		0x3956c25bf348b538, 0x59f111f1b605d019, 0x923f82a4af194f9b, 0xab1c5ed5da6d8118,
		0xd807aa98a3030242, 0x12835b0145706fbe, 0x243185be4ee4b28c, 0x550c7dc3d5ffb4e2,
		0x72be5d74f27b896f, 0x80deb1fe3b1696b1, 0x9bdc06a725c71235, 0xc19bf174cf692694,
		0xe49b69c19ef14ad2, 0xefbe4786384f25e3, 0x0fc19dc68b8cd5b5, 0x240ca1cc77ac9c65,
		0x2de92c6f592b0275, 0x4a7484aa6ea6e483, 0x5cb0a9dcbd41fbd4, 0x76f988da831153b5,
		0x983e5152ee66dfab, 0xa831c66d2db43210, 0xb00327c898fb213f, 0xbf597fc7beef0ee4,
		0xc6e00bf33da88fc2, 0xd5a79147930aa725, 0x06ca6351e003826f, 0x142929670a0e6e70,
		0x27b70a8546d22ffc, 0x2e1b21385c26c926, 0x4d2c6dfc5ac42aed, 0x53380d139d95b3df,
		0x650a73548baf63de, 0x766a0abb3c77b2a8, 0x81c2c92e47edaee6, 0x92722c851482353b,
		0xa2bfe8a14cf10364, 0xa81a664bbc423001, 0xc24b8b70d0f89791, 0xc76c51a30654be30,
		0xd192e819d6ef5218, 0xd69906245565a910, 0xf40e35855771202a, 0x106aa07032bbd1b8,
		0x19a4c116b8d2d0c8, 0x1e376c085141ab53, 0x2748774cdf8eeb99, 0x34b0bcb5e19b48a8,
		0x391c0cb3c5c95a63, 0x4ed8aa4ae3418acb, 0x5b9cca4f7763e373, 0x682e6ff3d6b2b8a3,
		0x748f82ee5defb2fc, 0x78a5636f43172f60, 0x84c87814a1f0ab72, 0x8cc702081a6439ec,
		0x90befffa23631e28, 0xa4506cebde82bde9, 0xbef9a3f7b2c67915, 0xc67178f2e372532b,
		0xca273eceea26619c, 0xd186b8c721c0c207, 0xeada7dd6cde0eb1e, 0xf57d4f7fee6ed178,
		0x06f067aa72176fba, 0x0a637dc5a2c898a6, 0x113f9804bef90dae, 0x1b710b35131c471b,
		0x28db77f523047d84, 0x32caab7b40c72493, 0x3c9ebe0a15c9bebc, 0x431d67c49c100d4c,
		0x4cc5d4becb3e42b6, 0x597f299cfc657e2a, 0x5fcb6fab3ad6faec, 0x6c44198c4a475817
	];
	for x in m.chunks_exact(16) {
		let (aa, bb, cc, dd, ee, ff, gg, hh) = (a, b, c, d, e, f, g, h);
		let w: [Word64; 80] = {
			let mut temp: [Word64; 80] = [Word64::from(0); 80];
			for i in 0..16 {
				temp[i] = x[i];
			}
			for i in 16..80 {
				let s0 = (temp[i-15] >>  1) ^ (temp[i-15] >>  8) ^ (temp[i-15] >> "7");
				let s1 = (temp[i- 2] >> 19) ^ (temp[i- 2] >> 61) ^ (temp[i- 2] >> "6");
				temp[i] = temp[i-16] + s0 + temp[i-7] + s1;
			}
			temp
		};
		for i in 0..80 {
			let s1 = (e >> 14) ^ (e >> 18) ^ (e >> 41);
			let ch = (e & f) ^ (!e & g);
			let temp1 = h + s1 + ch + K[i] + w[i];
			let s0 = (a >> 28) ^ (a >> 34) ^ (a >> 39);
			let maj = (a & b) ^ (a & c) ^ (b & c);
			let temp2 = s0 + maj;
			h = g;
			g = f;
			f = e;
			e = d + temp1;
			d = c;
			c = b;
			b = a;
			a = temp1 + temp2;
		}
		//
		a = a + aa;
		b = b + bb;
		c = c + cc;
		d = d + dd;
		e = e + ee;
		f = f + ff;
		g = g + gg;
		h = h + hh;
	}
	//输出
	[a, b, c, d, e, f, g, h]
}

pub fn sha_384(message: &[u8]) -> [u8; 48] {
	let [a, b, c, d, e, f, _, _] = sha2_u64(message, [
		0xcbbb9d5dc1059ed8,
		0x629a292a367cd507,
		0x9159015a3070dd17,
		0x152fecd8f70e5939,
		0x67332667ffc00b31,
		0x8eb44a8768581511,
		0xdb0c2e0d64f98fa7,
		0x47b5481dbefa4fa4
	]);
	Word64::to_be_bytes_array([a, b, c, d, e, f])
}

pub fn sha_512(message: &[u8]) -> [u8; 64] {
	let [a, b, c, d, e, f, g, h] = sha2_u64(message, [
		0x6a09e667f3bcc908,
		0xbb67ae8584caa73b,
		0x3c6ef372fe94f82b,
		0xa54ff53a5f1d36f1,
		0x510e527fade682d1,
		0x9b05688c2b3e6c1f,
		0x1f83d9abfb41bd6b,
		0x5be0cd19137e2179
	]);
	Word64::to_be_bytes_array([a, b, c, d, e, f, g, h])
}

// t = 8*N
pub fn sha_512_t<const N: usize>(message: &[u8]) -> [u8; N] {
	assert!(N < 64);
	assert_ne!(N, 48);
	let iv = sha2_u64(format!("SHA-512/{}", 8*N).as_bytes(), [
		0xcfac43c256196cad,
		0x1ec20b20216f029e,
		0x99cb56d75b315d8e,
		0x00ea509ffab89354,
		0xf4abf7da08432774,
		0x3ea0cd298e9bc9ba,
		0xba267c0e5ee418ce,
		0xfe4568bcb6db84dc
	]);
	let [a, b, c, d, e, f, g, h] = sha2_u64(message, [
		iv[0].into(), iv[1].into(), iv[2].into(), iv[3].into(),
		iv[4].into(), iv[5].into(), iv[6].into(), iv[7].into()
	]);
	let temp: [u8; 64] = Word64::to_be_bytes_array([a, b, c, d, e, f, g, h]);
	let mut result: [u8; N] = [0; N];
	for i in 0..N {
		result[i] = temp[i];
	}
	result
}
