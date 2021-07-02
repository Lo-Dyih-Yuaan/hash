use crate::word::*;
fn f1(x6: Word32, x5: Word32, x4: Word32, x3: Word32, x2: Word32, x1: Word32, x0: Word32) -> Word32 {
	x1 & x4 ^ x2 & x5 ^ x3 & x6 ^ x0 & x1 ^ x0
}
fn f2(x6: Word32, x5: Word32, x4: Word32, x3: Word32, x2: Word32, x1: Word32, x0: Word32) -> Word32 {
	x1 & x2 & x3 ^ x2 & x4 & x5 ^ x1 & x2 ^ x1 & x4 ^ x2 & x6 ^ x3 & x5 ^ x4 & x5 ^ x0 & x2 ^ x0
}
fn f3(x6: Word32, x5: Word32, x4: Word32, x3: Word32, x2: Word32, x1: Word32, x0: Word32) -> Word32 {
	x1 & x2 & x3 ^ x1 & x4 ^ x2 & x5 ^ x3 & x6 ^ x0 & x3 ^ x0
}
fn f4(x6: Word32, x5: Word32, x4: Word32, x3: Word32, x2: Word32, x1: Word32, x0: Word32) -> Word32 {
	x1 & x2 & x3 ^ x2 & x4 & x5 ^ x3 & x4 & x6 ^ x1 & x4 ^ x2 & x6 ^ x3 & x4 ^ x3 & x5 ^ x3 & x6 ^ x4 & x5 ^ x4 & x6 ^ x0 & x4 ^ x0
}
fn f5(x6: Word32, x5: Word32, x4: Word32, x3: Word32, x2: Word32, x1: Word32, x0: Word32) -> Word32 {
	x1 & x4 ^ x2 & x5 ^ x3 & x6 ^ x0 & x1 & x2 & x3 ^ x0 & x5 ^ x0
}
const ORD: [usize; 160] = [
	 0,  1,  2,  3,  4,  5,  6,  7,  8,  9, 10, 11, 12, 13, 14, 15,
	16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31,
	 5, 14, 26, 18, 11, 28,  7, 16,  0, 23, 20, 22,  1, 10,  4,  8,
	30,  3, 21,  9, 17, 24, 29,  6, 19, 12, 15, 13,  2, 25, 31, 27,
	19,  9,  4, 20, 28, 17,  8, 22, 29, 14, 25, 12, 24, 30, 16, 26,
	31, 15,  7,  3,  1,  0, 18, 27, 13,  6, 21, 10, 23, 11,  5,  2,
	24,  4,  0, 14,  2,  7, 28, 23, 26,  6, 30, 20, 18, 25, 19,  3,
	22, 11, 31, 21,  8, 27, 12,  9,  1, 29,  5, 15, 17, 10, 16, 13,
	27,  3, 21, 26, 17, 11, 20, 29, 19,  0, 12,  7, 13,  8, 31, 10,
	 5,  9, 14, 30, 18,  6, 28, 24,  2, 23, 16, 22,  4,  1, 25, 15
];
const K: [u32; 160] = [
	0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
	0x452821E6, 0x38D01377, 0xBE5466CF, 0x34E90C6C, 0xC0AC29B7, 0xC97C50DD, 0x3F84D5B5, 0xB5470917,
	0x9216D5D9, 0x8979FB1B, 0xD1310BA6, 0x98DFB5AC, 0x2FFD72DB, 0xD01ADFB7, 0xB8E1AFED, 0x6A267E96,
	0xBA7C9045, 0xF12C7F99, 0x24A19947, 0xB3916CF7, 0x0801F2E2, 0x858EFC16, 0x636920D8, 0x71574E69,
	0xA458FEA3, 0xF4933D7E, 0x0D95748F, 0x728EB658, 0x718BCD58, 0x82154AEE, 0x7B54A41D, 0xC25A59B5,
	0x9C30D539, 0x2AF26013, 0xC5D1B023, 0x286085F0, 0xCA417918, 0xB8DB38EF, 0x8E79DCB0, 0x603A180E,
	0x6C9E0E8B, 0xB01E8A3E, 0xD71577C1, 0xBD314B27, 0x78AF2FDA, 0x55605C60, 0xE65525F3, 0xAA55AB94,
	0x57489862, 0x63E81440, 0x55CA396A, 0x2AAB10B6, 0xB4CC5C34, 0x1141E8CE, 0xA15486AF, 0x7C72E993,
	0xB3EE1411, 0x636FBC2A, 0x2BA9C55D, 0x741831F6, 0xCE5C3E16, 0x9B87931E, 0xAFD6BA33, 0x6C24CF5C,
	0x7A325381, 0x28958677, 0x3B8F4898, 0x6B4BB9AF, 0xC4BFE81B, 0x66282193, 0x61D809CC, 0xFB21A991,
	0x487CAC60, 0x5DEC8032, 0xEF845D5D, 0xE98575B1, 0xDC262302, 0xEB651B88, 0x23893E81, 0xD396ACC5,
	0x0F6D6FF3, 0x83F44239, 0x2E0B4482, 0xA4842004, 0x69C8F04A, 0x9E1F9B5E, 0x21C66842, 0xF6E96C9A,
	0x670C9C61, 0xABD388F0, 0x6A51A0D2, 0xD8542F68, 0x960FA728, 0xAB5133A3, 0x6EEF0B6C, 0x137A3BE4,
	0xBA3BF050, 0x7EFB2A98, 0xA1F1651D, 0x39AF0176, 0x66CA593E, 0x82430E88, 0x8CEE8619, 0x456F9FB4,
	0x7D84A5C3, 0x3B8B5EBE, 0xE06F75D8, 0x85C12073, 0x401A449F, 0x56C16AA6, 0x4ED3AA62, 0x363F7706,
	0x1BFEDF72, 0x429B023D, 0x37D0D724, 0xD00A1248, 0xDB0FEAD3, 0x49F1C09B, 0x075372C9, 0x80991B7B,
	0x25D479D8, 0xF6E8DEF7, 0xE3FE501A, 0xB6794C3B, 0x976CE0BD, 0x04C006BA, 0xC1A94FB6, 0x409F60C4,
];
pub fn haval_3_256(message: &[u8]) -> [u8; 32] {
	const VERSION: u32 = 1;
	const PASS: u32 = 3;
	const DGSTLENG: u32 = 256;
	//补位
	let padding = 128 - (message.len()+2+8)%128;
	let mut m: Vec<u8> = Vec::with_capacity(message.len() + padding + 2 + 8);
	m.extend_from_slice(message);
	m.push(0x01u8);
	m.extend(&vec![0; padding-1]);
	m.push((((DGSTLENG & 0x3) << 6) | ((PASS & 0x7) << 3) | (VERSION & 0x7)) as u8);
	m.push(((DGSTLENG >> 2) & 0xFF) as u8);
	//增加长度
	m.extend_from_slice(&(message.len() as u64).wrapping_mul(8).to_le_bytes());
	//
	let m: &[Word32] = &Word32::slice_from_le_bytes(&m);
	//
	let mut x0: Word32 = Word32::from(0x243F6A88);
	let mut x1: Word32 = Word32::from(0x85A308D3);
	let mut x2: Word32 = Word32::from(0x13198A2E);
	let mut x3: Word32 = Word32::from(0x03707344);
	let mut x4: Word32 = Word32::from(0xA4093822);
	let mut x5: Word32 = Word32::from(0x299F31D0);
	let mut x6: Word32 = Word32::from(0x082EFA98);
	let mut x7: Word32 = Word32::from(0xEC4E6C89);
	for x in m.chunks_exact(32) {
		let (y0,y1,y2,y3,y4,y5,y6,y7)= (x0,x1,x2,x3,x4,x5,x6,x7);
		for i in 0..96 {
			let p = 
				if i < 32 {f1(x1, x0, x3, x5, x6, x2 ,x4)}
				else if i < 64 {f2(x4, x2, x1, x0, x5, x3, x6)}
				else {f3(x6, x1, x2, x3, x4, x5, x0)};
			let temp = (p >> 7) + (x7 >> 11) + x[ORD[i]] + K[i];
			x7 = x6;
			x6 = x5;
			x5 = x4;
			x4 = x3;
			x3 = x2;
			x2 = x1;
			x1 = x0;
			x0 = temp;
		}
		x0 = x0+y0;
		x1 = x1+y1;
		x2 = x2+y2;
		x3 = x3+y3;
		x4 = x4+y4;
		x5 = x5+y5;
		x6 = x6+y6;
		x7 = x7+y7;
	}
	//输出
	Word32::to_le_bytes_array([x0,x1,x2,x3,x4,x5,x6,x7])
}

pub fn haval_4_256(message: &[u8]) -> [u8; 32] {
	const VERSION: u32 = 1;
	const PASS: u32 = 4;
	const DGSTLENG: u32 = 256;
	//补位
	let padding = 128 - (message.len()+2+8)%128;
	let mut m: Vec<u8> = Vec::with_capacity(message.len() + padding + 2 + 8);
	m.extend_from_slice(message);
	m.push(0x01u8);
	m.extend(&vec![0; padding-1]);
	m.push((((DGSTLENG & 0x3) << 6) | ((PASS & 0x7) << 3) | (VERSION & 0x7)) as u8);
	m.push(((DGSTLENG >> 2) & 0xFF) as u8);
	//增加长度
	m.extend_from_slice(&(message.len() as u64).wrapping_mul(8).to_le_bytes());
	//
	let m: &[Word32] = &Word32::slice_from_le_bytes(&m);
	//
	let mut x0: Word32 = Word32::from(0x243F6A88);
	let mut x1: Word32 = Word32::from(0x85A308D3);
	let mut x2: Word32 = Word32::from(0x13198A2E);
	let mut x3: Word32 = Word32::from(0x03707344);
	let mut x4: Word32 = Word32::from(0xA4093822);
	let mut x5: Word32 = Word32::from(0x299F31D0);
	let mut x6: Word32 = Word32::from(0x082EFA98);
	let mut x7: Word32 = Word32::from(0xEC4E6C89);
	for x in m.chunks_exact(32) {
		let (y0,y1,y2,y3,y4,y5,y6,y7)= (x0,x1,x2,x3,x4,x5,x6,x7);
		for i in 0..128 {
			let p = 
				if i < 32 {f1(x2, x6, x1, x4, x5, x3, x0)}
				else if i < 64 {f2(x3, x5, x2, x0, x1, x6, x4)}
				else if i < 96 {f3(x1, x4, x3, x6, x0, x2, x5)}
				else {f4(x6, x4, x0, x5, x2, x1, x3)};
			let temp = (p >> 7) + (x7 >> 11) + x[ORD[i]] + K[i];
			x7 = x6;
			x6 = x5;
			x5 = x4;
			x4 = x3;
			x3 = x2;
			x2 = x1;
			x1 = x0;
			x0 = temp;
		}
		x0 = x0+y0;
		x1 = x1+y1;
		x2 = x2+y2;
		x3 = x3+y3;
		x4 = x4+y4;
		x5 = x5+y5;
		x6 = x6+y6;
		x7 = x7+y7;
	}
	//输出
	Word32::to_le_bytes_array([x0,x1,x2,x3,x4,x5,x6,x7])
}

pub fn haval_5_256(message: &[u8]) -> [u8; 32] {
	const VERSION: u32 = 1;
	const PASS: u32 = 5;
	const DGSTLENG: u32 = 256;
	//补位
	let padding = 128 - (message.len()+2+8)%128;
	let mut m: Vec<u8> = Vec::with_capacity(message.len() + padding + 2 + 8);
	m.extend_from_slice(message);
	m.push(0x01u8);
	m.extend(&vec![0; padding-1]);
	m.push((((DGSTLENG & 0x3) << 6) | ((PASS & 0x7) << 3) | (VERSION & 0x7)) as u8);
	m.push(((DGSTLENG >> 2) & 0xFF) as u8);
	//增加长度
	m.extend_from_slice(&(message.len() as u64).wrapping_mul(8).to_le_bytes());
	//
	let m: &[Word32] = &Word32::slice_from_le_bytes(&m);
	//
	let mut x0: Word32 = Word32::from(0x243F6A88);
	let mut x1: Word32 = Word32::from(0x85A308D3);
	let mut x2: Word32 = Word32::from(0x13198A2E);
	let mut x3: Word32 = Word32::from(0x03707344);
	let mut x4: Word32 = Word32::from(0xA4093822);
	let mut x5: Word32 = Word32::from(0x299F31D0);
	let mut x6: Word32 = Word32::from(0x082EFA98);
	let mut x7: Word32 = Word32::from(0xEC4E6C89);
	for x in m.chunks_exact(32) {
		let (y0,y1,y2,y3,y4,y5,y6,y7)= (x0,x1,x2,x3,x4,x5,x6,x7);
		for i in 0..160 {
			let p = 
				if i < 32 {f1(x3, x4, x1, x0, x5, x2, x6)}
				else if i < 64 {f2(x6, x2, x1, x0, x3, x4, x5)}
				else if i < 96 {f3(x2, x6, x0, x4, x3, x1, x5)}
				else if i < 128 {f4(x1, x5, x3, x2, x0, x4, x6)}
				else {f5(x2, x5, x0, x6, x4, x3, x1)};
			let temp = (p >> 7) + (x7 >> 11) + x[ORD[i]] + K[i];
			x7 = x6;
			x6 = x5;
			x5 = x4;
			x4 = x3;
			x3 = x2;
			x2 = x1;
			x1 = x0;
			x0 = temp;
		}
		x0 = x0+y0;
		x1 = x1+y1;
		x2 = x2+y2;
		x3 = x3+y3;
		x4 = x4+y4;
		x5 = x5+y5;
		x6 = x6+y6;
		x7 = x7+y7;
	}
	//输出
	Word32::to_le_bytes_array([x0,x1,x2,x3,x4,x5,x6,x7])
}