use crate::word::*;
pub fn sha0(message: &[u8]) -> [u8; 20] {
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
	let mut a: Word32 = Word32::from(0x67452301);
	let mut b: Word32 = Word32::from(0xefcdab89);
	let mut c: Word32 = Word32::from(0x98badcfe);
	let mut d: Word32 = Word32::from(0x10325476);
	let mut e: Word32 = Word32::from(0xc3d2e1f0);
	macro_rules! f {
		($x:expr, $y:expr, $z:expr) => {($x & $y) | (!$x & $z)};
		($a:expr, $b:expr, $c:expr, $d:expr, $e:expr, $m:expr) =>
		{ ($a << 5) + f!($b, $c, $d) + $e + $m + 0x5a827999u32 };
	}
	macro_rules! g {
		($x:expr, $y:expr, $z:expr) => {$x ^ $y ^ $z};
		($a:expr, $b:expr, $c:expr, $d:expr, $e:expr, $m:expr) =>
		{ ($a << 5) + g!($b, $c, $d) + $e + $m + 0x6ed9eba1u32 };
	}
	macro_rules! h {
		($x:expr, $y:expr, $z:expr) => {($x & $y) | ($x & $z) | ($y & $z)};
		($a:expr, $b:expr, $c:expr, $d:expr, $e:expr, $m:expr) =>
		{ ($a << 5) + h!($b, $c, $d) + $e + $m + 0x8f1bbcdcu32 };
	}
	macro_rules! i {
		($x:expr, $y:expr, $z:expr) => {$x ^ $y ^ $z};
		($a:expr, $b:expr, $c:expr, $d:expr, $e:expr, $m:expr) =>
		{ ($a << 5) + i!($b, $c, $d) + $e + $m + 0xca62c1d6u32 };
	}
	for x in m.chunks_exact(16) {
		let mut x = x.to_owned();
		let (aa, bb, cc, dd, ee) = (a, b, c, d, e);
		for i in 0..80 {
			if i >= 16 {
				x[i%16] = x[(i+13)%16] ^ x[(i+8)%16] ^ x[(i+2)%16] ^ x[i%16];
			}
			let temp =
				if i < 20 { f!(a,b,c,d,e, x[i%16]) }
				else if i < 40 { g!(a,b,c,d,e, x[i%16]) }
				else if i < 60 { h!(a,b,c,d,e, x[i%16]) }
				else { i!(a,b,c,d,e, x[i%16]) };
			e = d;
			d = c;
			c = b >> 2;
			b = a;
			a = temp;
		}
		//
		a = a + aa;
		b = b + bb;
		c = c + cc;
		d = d + dd;
		e = e + ee;
	}
	//输出
	Word32::to_be_bytes_array([a, b, c, d, e])
}
