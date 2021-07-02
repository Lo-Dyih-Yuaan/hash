use crate::word::*;
pub fn md4(message: &[u8]) -> [u8; 16] {
	//补位
	let padding = 64 - (message.len()+8)%64;
	let mut m: Vec<u8> = Vec::with_capacity(message.len() + padding + 8);
	m.extend_from_slice(message);
	m.push(0x80u8);
	m.extend(&vec![0; padding-1]);
	//增加长度
	m.extend_from_slice(&(message.len() as u64).wrapping_mul(8).to_le_bytes());
	//
	let m: &[Word32] = &Word32::slice_from_le_bytes(&m);
	//
	let mut a: Word32 = Word32::from(0x67452301);
	let mut b: Word32 = Word32::from(0xefcdab89);
	let mut c: Word32 = Word32::from(0x98badcfe);
	let mut d: Word32 = Word32::from(0x10325476);
	macro_rules! f {
		($x:expr, $y:expr, $z:expr) => {($x & $y) | (!$x & $z)};
		($a:expr, $b:expr, $c:expr, $d:expr, $k:expr, $s:expr) =>
		{ ($a + f!($b, $c, $d) + $k) << $s };
	}
	macro_rules! g {
		($x:expr, $y:expr, $z:expr) => {($x & $y) | ($x & $z) | ($y & $z)};
		($a:expr, $b:expr, $c:expr, $d:expr, $k:expr, $s:expr) =>
		{ ($a + g!($b, $c, $d) + $k + 0x5A827999u32) << $s };
	}
	macro_rules! h {
		($x:expr, $y:expr, $z:expr) => {$x ^ $y ^ $z};
		($a:expr, $b:expr, $c:expr, $d:expr, $k:expr, $s:expr) =>
		{ ($a + h!($b, $c, $d) + $k + 0x6ED9EBA1u32) << $s };
	}
	const K: [usize; 48] = [
		 0, 1, 2, 3, 4, 5, 6, 7, 8, 9,10,11,12,13,14,15,
		 0, 4, 8,12, 1, 5, 9,13, 2, 6,10,14, 3, 7,11,15,
		 0, 8, 4,12, 2,10, 6,14, 1, 9, 5,13, 3,11, 7,15
	];
	const S: [u32; 48] = [
		 3, 7,11,19, 3, 7,11,19, 3, 7,11,19, 3, 7,11,19,
		 3, 5, 9,13, 3, 5, 9,13, 3, 5, 9,13, 3, 5, 9,13,
		 3, 9,11,15, 3, 9,11,15, 3, 9,11,15, 3, 9,11,15
	];
	for x in m.chunks_exact(16) {
		let (aa, bb, cc, dd) = (a, b, c, d);
		//
		for i in 0..48 {
			let temp =
				if i < 16 { f!(a,b,c,d, x[K[i]], S[i]) }
				else if i < 32 { g!(a,b,c,d, x[K[i]], S[i]) }
				else { h!(a,b,c,d, x[K[i]], S[i]) };
			a = d;
			d = c;
			c = b;
			b = temp;
		}
		//
		a = a+aa;
		b = b+bb;
		c = c+cc;
		d = d+dd;
	}
	//输出
	Word32::to_le_bytes_array([a, b, c, d])
}
