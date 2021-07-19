use crate::word::*;

fn mu(a: Word512) -> [[u8; 8]; 8] {
	let a = a.to_be_bytes();
	let mut temp: [[u8; 8]; 8] = [[0; 8]; 8];
	for i in 0..8 {
		for j in 0..8 {
			temp[i][j] = a[8*i+j];
		}
	}
	temp
}
fn amu(b: [[u8; 8]; 8]) -> Word512 {
	let mut temp: [u8; 64] = [0; 64];
	for i in 0..8 {
		for j in 0..8 {
			temp[8*i+j] = b[i][j];
		}
	}
	Word512::from_be_bytes(temp)
}

const S: [u8; 256] = [
	0x18, 0x23, 0xc6, 0xE8, 0x87, 0xB8, 0x01, 0x4F, 0x36, 0xA6, 0xd2, 0xF5, 0x79, 0x6F, 0x91, 0x52,
	0x60, 0xBc, 0x9B, 0x8E, 0xA3, 0x0c, 0x7B, 0x35, 0x1d, 0xE0, 0xd7, 0xc2, 0x2E, 0x4B, 0xFE, 0x57,
	0x15, 0x77, 0x37, 0xE5, 0x9F, 0xF0, 0x4A, 0xdA, 0x58, 0xc9, 0x29, 0x0A, 0xB1, 0xA0, 0x6B, 0x85,
	0xBd, 0x5d, 0x10, 0xF4, 0xcB, 0x3E, 0x05, 0x67, 0xE4, 0x27, 0x41, 0x8B, 0xA7, 0x7d, 0x95, 0xd8,
	0xFB, 0xEE, 0x7c, 0x66, 0xdd, 0x17, 0x47, 0x9E, 0xcA, 0x2d, 0xBF, 0x07, 0xAd, 0x5A, 0x83, 0x33,
	0x63, 0x02, 0xAA, 0x71, 0xc8, 0x19, 0x49, 0xd9, 0xF2, 0xE3, 0x5B, 0x88, 0x9A, 0x26, 0x32, 0xB0,
	0xE9, 0x0F, 0xd5, 0x80, 0xbe, 0xcd, 0x34, 0x48, 0xFF, 0x7A, 0x90, 0x5F, 0x20, 0x68, 0x1A, 0xAE,
	0xB4, 0x54, 0x93, 0x22, 0x64, 0xF1, 0x73, 0x12, 0x40, 0x08, 0xc3, 0xEc, 0xdB, 0xA1, 0x8d, 0x3d,
	0x97, 0x00, 0xcF, 0x2B, 0x76, 0x82, 0xd6, 0x1B, 0xB5, 0xAF, 0x6A, 0x50, 0x45, 0xF3, 0x30, 0xEF,
	0x3F, 0x55, 0xA2, 0xEA, 0x65, 0xBA, 0x2F, 0xc0, 0xdE, 0x1c, 0xFd, 0x4d, 0x92, 0x75, 0x06, 0x8A,
	0xB2, 0xE6, 0x0E, 0x1F, 0x62, 0xd4, 0xA8, 0x96, 0xF9, 0xc5, 0x25, 0x59, 0x84, 0x72, 0x39, 0x4c,
	0x5E, 0x78, 0x38, 0x8c, 0xd1, 0xA5, 0xE2, 0x61, 0xB3, 0x21, 0x9c, 0x1E, 0x43, 0xc7, 0xFc, 0x04,
	0x51, 0x99, 0x6d, 0x0d, 0xFA, 0xdF, 0x7E, 0x24, 0x3B, 0xAB, 0xcE, 0x11, 0x8F, 0x4E, 0xB7, 0xEB,
	0x3c, 0x81, 0x94, 0xF7, 0xB9, 0x13, 0x2c, 0xd3, 0xE7, 0x6E, 0xc4, 0x03, 0x56, 0x44, 0x7F, 0xA9,
	0x2A, 0xBB, 0xc1, 0x53, 0xdc, 0x0B, 0x9d, 0x6c, 0x31, 0x74, 0xF6, 0x46, 0xAc, 0x89, 0x14, 0xE1,
	0x16, 0x3A, 0x69, 0x09, 0x70, 0xB6, 0xd0, 0xEd, 0xcc, 0x42, 0x98, 0xA4, 0x28, 0x5c, 0xF8, 0x86
];
fn gamma(n: [[u8; 8]; 8]) -> [[u8; 8]; 8] {
	let mut temp: [[u8; 8]; 8] = [[0; 8]; 8];
	for i in 0..8 {
		for j in 0..8 {
			temp[i][j] = S[n[i][j] as usize];
		}
	}
	temp
}
fn pi(n: [[u8; 8]; 8]) -> [[u8; 8]; 8] {
	let mut temp: [[u8; 8]; 8] = [[0; 8]; 8];
	for i in 0..8 {
		for j in 0..8 {
			temp[i][j] = n[(i+8-j)%8][j];
		}
	}
	temp
}
fn theta(n: [[u8; 8]; 8]) -> [[u8; 8]; 8] {
	fn mul(a: u8, b: u8) -> u8 {
		let (mut a, mut b) = (a, b);
		let mut p = 0u8;
		while a != 0 && b != 0 {
			if b & 1 != 0 {
				p ^= a;
			}
			a = (a << 1) ^ if a > 0x7f {0x1d} else {0};
			b >>= 1;
		}
		p
	}
	const C: [[u8; 8]; 8] = [
		[1, 1, 4, 1, 8, 5, 2, 9],
		[9, 1, 1, 4, 1, 8, 5, 2],
		[2, 9, 1, 1, 4, 1, 8, 5],
		[5, 2, 9, 1, 1, 4, 1, 8],
		[8, 5, 2, 9, 1, 1, 4, 1],
		[1, 8, 5, 2, 9, 1, 1, 4],
		[4, 1, 8, 5, 2, 9, 1, 1],
		[1, 4, 1, 8, 5, 2, 9, 1]
	];
	let mut temp: [[u8; 8]; 8] = [[0; 8]; 8];
	for i in 0..8 {
		for j in 0..8 {
			for k in 0..8 {
				temp[i][j] = temp[i][j]^mul(n[i][k], C[k][j]);
			}
		}
	}
	temp
}
fn sigma(k: [[u8; 8]; 8], a: [[u8; 8]; 8]) -> [[u8; 8]; 8] {
	let mut temp: [[u8; 8]; 8] = [[0; 8]; 8];
	for i in 0..8 {
		for j in 0..8 {
			temp[i][j] = k[i][j] ^ a[i][j];
		}
	}
	temp
}
fn rho(k: [[u8; 8]; 8], a: [[u8; 8]; 8]) -> [[u8; 8]; 8] {
	sigma(k, theta(pi(gamma(a))))
}
fn w(k: [[u8; 8]; 8], a: [[u8; 8]; 8]) -> [[u8; 8]; 8] {
	let mut temp: [[u8; 8]; 8] = sigma(k, a);
	let mut k = k;
	for r in 0..10 {
		let mut c: [[u8; 8]; 8] = [[0; 8]; 8];
		for i in 0..8 {
			c[0][i] = S[8*r+i];
		}
		k = rho(c, k);
		temp = rho(k, temp);
	}
	temp
}



pub fn whirlpool(message: &[u8]) -> [u8; 64] {
	let len = Word256::from_bits(Bits::from(message.len()) << 3);
	//补位
	let padding = 64 - (message.len() + 32)%64;
	let mut m: Vec<u8> = Vec::with_capacity(message.len() + padding);
	m.extend_from_slice(message);
	m.push(0x80u8);
	m.extend(&vec![0; padding-1]);
	//增加长度
	m.extend_from_slice(&len.to_be_bytes());
	//
	let m: &[Word512] = &Word512::slice_from_be_bytes(&m);
	//
	let mut h: Word512 = Word512::from(0);
	for m in m {
		h = amu(w(mu(h), mu(*m))) ^ h ^ *m;
	}
	//输出
	h.to_be_bytes()
}
