use crate::word::*;

fn k(n: Word32) -> Word32 {
	const PI: [[u8; 16]; 8] = [
		[0x4, 0xA, 0x9, 0x2, 0xD, 0x8, 0x0, 0xE, 0x6, 0xB, 0x1, 0xC, 0x7, 0xF, 0x5, 0x3],
		[0xE, 0xB, 0x4, 0xC, 0x6, 0xD, 0xF, 0xA, 0x2, 0x3, 0x8, 0x1, 0x0, 0x7, 0x5, 0x9],
		[0x5, 0x8, 0x1, 0xD, 0xA, 0x3, 0x4, 0x2, 0xE, 0xF, 0xC, 0x7, 0x6, 0x0, 0x9, 0xB],
		[0x7, 0xD, 0xA, 0x1, 0x0, 0x8, 0x9, 0xF, 0xE, 0x4, 0x6, 0xC, 0xB, 0x2, 0x5, 0x3],
		[0x6, 0xC, 0x7, 0x1, 0x5, 0xF, 0xD, 0x8, 0x4, 0xA, 0x9, 0xE, 0x0, 0x3, 0xB, 0x2],
		[0x4, 0xB, 0xA, 0x0, 0x7, 0x2, 0x1, 0xD, 0x3, 0x6, 0x8, 0x5, 0x9, 0xC, 0xF, 0xE],
		[0xD, 0xB, 0x4, 0x1, 0x3, 0xF, 0x5, 0x9, 0x0, 0xA, 0xE, 0x7, 0x6, 0x8, 0x2, 0xC],
		[0x1, 0xF, 0xD, 0x0, 0x5, 0x7, 0xA, 0x4, 0x9, 0x2, 0x3, 0xE, 0x6, 0xB, 0x8, 0xC]
	];
	let mut n = n.to_le_bytes();
	for i in 0..4 {
		n[i] =
			PI[2*i][(n[i]&0xf) as usize] |
			(PI[2*i+1][(n[i]>>4) as usize] << 4);
	}
	Word32::from_le_bytes(n)
}

fn e(block: Word64, key: Word256) -> Word64 {
	const X: [usize; 32] = [
		0, 1, 2, 3, 4, 5, 6, 7,
		0, 1, 2, 3, 4, 5, 6, 7,
		0, 1, 2, 3, 4, 5, 6, 7,
		7, 6, 5, 4, 3, 2, 1, 0
	];
	let block = Word32::slice_from_le_bytes(&block.to_le_bytes());
	let key = Word32::slice_from_le_bytes(&key.to_le_bytes());
	let (mut a, mut b) = (block[0], block[1]);
	for i in 0..32 {
		let temp = a;
		a = (k(a + key[X[i]]) << 11) ^ b;
		b = temp;
	}
	Word64::from_le_bytes(Word32::to_le_bytes_array([b, a]))
}

fn psi(y: [Word16; 16]) -> [Word16; 16] {
	[
		y[1],y[2],y[3],y[4],y[5],
		y[6],y[7],y[8],y[9],y[10],
		y[11],y[12],y[13],y[14],y[15],
		y[0]^y[1]^y[2]^y[3]^y[12]^y[15]
	]
}

fn f(h: Word256, m: Word256) -> Word256 {
	fn a(y: Word256) -> Word256 {
		let y = Word64::slice_from_le_bytes(&y.to_le_bytes());
		Word256::from_le_bytes(Word64::to_le_bytes_array([y[1], y[2], y[3], y[0]^y[1]]))
	}
	const PHI: [usize; 32] = [
		 0,  8, 16, 24,  1,  9, 17, 25,
		 2, 10, 18, 26,  3, 11, 19, 27,
		 4, 12, 20, 28,  5, 13, 21, 29,
		 6, 14, 22, 30,  7, 15, 23, 31
	];
	fn p(y: Word256) -> Word256 {
		let y = y.to_le_bytes();
		let mut temp: [u8; 32] = [0; 32];
		for i in 0..32 {
			temp[i] = y[PHI[i]];
		}
		Word256::from_le_bytes(temp)
	}
	const C: [Word256; 3] = [
		Word256::new("0x0000000000000000000000000000000000000000000000000000000000000000"),
		Word256::new("0xff00ffff000000ffff0000ff00ffff0000ff00ff00ff00ffff00ff00ff00ff00"),
		Word256::new("0x0000000000000000000000000000000000000000000000000000000000000000")
	];
	let (mut u, mut v) = (h, m);
	let mut k: [Word256; 4] = [Word256::from(0); 4];
	k[0] = p(u ^ v);
	for i in 0..3 {
		u = a(u) ^ C[i];
		v = a(a(v));
		k[i+1] = p(u ^ v);
	}
	let s = Word64::slice_from_le_bytes(&h.to_le_bytes());
	let s = [e(s[0], k[0]), e(s[1], k[1]), e(s[2], k[2]), e(s[3], k[3])];
	let s = Word16::slice_from_le_bytes(&Word64::to_le_bytes_array::<4,32>(s));
	let mut temp: [Word16; 16] = [Word16::new(0); 16];
	temp.copy_from_slice(&s);
	for _ in 0..12 {
		temp = psi(temp);
	}
	let m = Word16::slice_from_le_bytes(&m.to_le_bytes());
	let h = Word16::slice_from_le_bytes(&h.to_le_bytes());
	for i in 0..16 {
		temp[i] = temp[i] ^ m[i];
	}
	temp = psi(temp);
	for i in 0..16 {
		temp[i] = temp[i] ^ h[i];
	}
	for _ in 0..61 {
		temp = psi(temp);
	}
	Word256::from_le_bytes(Word16::to_le_bytes_array(temp))
}

pub fn gost94(message: &[u8]) -> [u8; 32] {
	let l_max = Word256::from_bits(Bits::from(message.len()) << 3);
	//补位
	let padding = (32 - message.len()%32) % 32;
	let mut m: Vec<u8> = Vec::with_capacity(message.len() + padding);
	m.extend_from_slice(message);
	m.extend(&vec![0; padding]);
	//
	let m: &[Word256] = &Word256::slice_from_le_bytes(&m);
	//
	let mut h: Word256 = Word256::from(0);
	let mut sigma: Word256 = Word256::from(0);
	for m in m {
		h = f(h, *m);
		sigma = sigma + *m;
	}
	h = f(h, l_max);
	h = f(h, sigma);
	//输出
	h.to_le_bytes()
}
