use crate::word::*;
pub fn sm3(message: &[u8]) -> [u8; 32] {
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
	let mut a: Word32 = Word32::from(0x7380166f);
	let mut b: Word32 = Word32::from(0x4914b2b9);
	let mut c: Word32 = Word32::from(0x172442d7);
	let mut d: Word32 = Word32::from(0xda8a0600);
	let mut e: Word32 = Word32::from(0xa96f30bc);
	let mut f: Word32 = Word32::from(0x163138aa);
	let mut g: Word32 = Word32::from(0xe38dee4d);
	let mut h: Word32 = Word32::from(0xb0fb0e4e);
	macro_rules! f {
		($x:expr, $y:expr, $z:expr) => {$x ^ $y ^ $z};
	}
	macro_rules! g {
		($x:expr, $y:expr, $z:expr) => {($x & $y ) | ($x & $z) | ($y & $z)};
	}
	macro_rules! h {
		($x:expr, $y:expr, $z:expr) => {($x & $y ) | (!$x & $z)};
	}
	for x in m.chunks_exact(16) {
		let (aa, bb, cc, dd, ee, ff, gg, hh) = (a, b, c, d, e, f, g, h);
		let w1: [Word32; 68] = {
			let mut temp: [Word32; 68] = [Word32::from(0); 68];
			for i in 0..16 {
				temp[i] = x[i];
			}
			for i in 16..68 {
				temp[i] = temp[i-16] ^ temp[i-9] ^ (temp[i-3] << 15);
				temp[i] = temp[i] ^ (temp[i] << 15) ^ (temp[i] << 23);
				temp[i] = temp[i] ^ (temp[i-13] << 7) ^ temp[i-6];
			}
			temp
		};
		let w2: [Word32; 64] = {
			let mut temp: [Word32; 64] = [Word32::from(0); 64];
			for i in 0..64 {
				temp[i] = w1[i] ^ w1[i+4];
			}
			temp
		};
		for i in 0..64 {
			let t = Word32::from(if i < 16 {0x79cc4519} else {0x7a879d8a});
			let s1 = ((a << 12) + e + (t  << i as u32)) << 7;
			let s2 = s1 ^ (a << 12);
			let temp1 =
				if i < 16 { f!(a,b,c) } else { g!(a,b,c) } + d + s2 + w2[i];
			let temp2 =
				if i < 16 { f!(e,f,g) } else { h!(e,f,g) } + h + s1 + w1[i];
			d = c;
			c = b << 9;
			b = a;
			a = temp1;
			h = g;
			g = f << 19;
			f = e;
			e = temp2 ^ (temp2 << 9) ^ (temp2 << 17);
			for z in &[a, b, c, d, e, f, g, h] {
				for x in &z.to_be_bytes() {
					print!("{:02x}", x);
				}
				print!(" ");
			}
			println!();
		}
		//
		a = a ^ aa;
		b = b ^ bb;
		c = c ^ cc;
		d = d ^ dd;
		e = e ^ ee;
		f = f ^ ff;
		g = g ^ gg;
		h = h ^ hh;
	}
	//输出
	Word32::to_be_bytes_array([a, b, c, d, e, f, g, h])
}
