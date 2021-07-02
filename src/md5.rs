use crate::word::*;
pub fn md5(message: &[u8]) -> [u8; 16] {
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
		($a:expr, $b:expr, $c:expr, $d:expr, $k:expr, $s:expr, $t:expr) =>
		{ (($a + f!($b, $c, $d) + $k + $t) << $s) + $b };
	}
	macro_rules! g {
		($x:expr, $y:expr, $z:expr) => {($x & $z) | ($y & !$z)};
		($a:expr, $b:expr, $c:expr, $d:expr, $k:expr, $s:expr, $t:expr) =>
		{ (($a + g!($b, $c, $d) + $k + $t) << $s) + $b };
	}
	macro_rules! h {
		($x:expr, $y:expr, $z:expr) => {$x ^ $y ^ $z};
		($a:expr, $b:expr, $c:expr, $d:expr, $k:expr, $s:expr, $t:expr) =>
		{ (($a + h!($b, $c, $d) + $k + $t) << $s) + $b };
	}
	macro_rules! i {
		($x:expr, $y:expr, $z:expr) => {$y ^ ($x | !$z)};
		($a:expr, $b:expr, $c:expr, $d:expr, $k:expr, $s:expr, $t:expr) =>
		{ (($a + i!($b, $c, $d) + $k + $t) << $s) + $b };
	}
	const T: [u32; 64] = [
		//F
		0xd76aa478, 0xe8c7b756, 0x242070db, 0xc1bdceee,
		0xf57c0faf, 0x4787c62a, 0xa8304613, 0xfd469501,
		0x698098d8, 0x8b44f7af, 0xffff5bb1, 0x895cd7be,
		0x6b901122, 0xfd987193, 0xa679438e, 0x49b40821,
		//G
		0xf61e2562, 0xc040b340, 0x265e5a51, 0xe9b6c7aa,
		0xd62f105d, 0x02441453, 0xd8a1e681, 0xe7d3fbc8,
		0x21e1cde6, 0xc33707d6, 0xf4d50d87, 0x455a14ed,
		0xa9e3e905, 0xfcefa3f8, 0x676f02d9, 0x8d2a4c8a,
		//H
		0xfffa3942, 0x8771f681, 0x6d9d6122, 0xfde5380c,
		0xa4beea44, 0x4bdecfa9, 0xf6bb4b60, 0xbebfbc70,
		0x289b7ec6, 0xeaa127fa, 0xd4ef3085, 0x04881d05,
		0xd9d4d039, 0xe6db99e5, 0x1fa27cf8, 0xc4ac5665,
		//I
		0xf4292244, 0x432aff97, 0xab9423a7, 0xfc93a039,
		0x655b59c3, 0x8f0ccc92, 0xffeff47d, 0x85845dd1,
		0x6fa87e4f, 0xfe2ce6e0, 0xa3014314, 0x4e0811a1,
		0xf7537e82, 0xbd3af235, 0x2ad7d2bb, 0xeb86d391
	];
	const K: [usize; 64] = [
		 0, 1, 2, 3, 4, 5, 6, 7, 8, 9,10,11,12,13,14,15,
		 1, 6,11, 0, 5,10,15, 4, 9,14, 3, 8,13, 2, 7,12,
		 5, 8,11,14, 1, 4, 7,10,13, 0, 3, 6, 9,12,15, 2,
		 0, 7,14, 5,12, 3,10, 1, 8,15, 6,13, 4,11, 2, 9
	];
	const S: [u32; 64] = [
		 7,12,17,22, 7,12,17,22, 7,12,17,22, 7,12,17,22,
		 5, 9,14,20, 5, 9,14,20, 5, 9,14,20, 5, 9,14,20,
		 4,11,16,23, 4,11,16,23, 4,11,16,23, 4,11,16,23,
		 6,10,15,21, 6,10,15,21, 6,10,15,21, 6,10,15,21
	];
	for x in m.chunks_exact(16) {
		let (aa, bb, cc, dd) = (a, b, c, d);
		for i in 0..64 {
			let temp =
				if i < 16 { f!(a,b,c,d, x[K[i]], S[i], T[i]) }
				else if i < 32 { g!(a,b,c,d, x[K[i]], S[i], T[i]) }
				else if i < 48 { h!(a,b,c,d, x[K[i]], S[i], T[i]) }
				else { i!(a,b,c,d, x[K[i]], S[i], T[i]) };
			a = d;
			d = c;
			c = b;
			b = temp;
		}
		a = a + aa;
		b = b + bb;
		c = c + cc;
		d = d + dd;
	}
	//输出
	Word32::to_le_bytes_array([a, b, c, d])
}
