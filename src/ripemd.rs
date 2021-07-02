use crate::word::*;
pub fn ripemd(message: &[u8]) -> [u8; 16] {
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
		(@ $a:expr, $b:expr, $c:expr, $d:expr, $k:expr, $s:expr) =>
		{ ($a + f!($b, $c, $d) + $k + 0x50A28BE6u32) << $s };
	}
	macro_rules! g {
		($x:expr, $y:expr, $z:expr) => {($x & $y) | ($x & $z) | ($y & $z)};
		($a:expr, $b:expr, $c:expr, $d:expr, $k:expr, $s:expr) =>
		{ ($a + g!($b, $c, $d) + $k + 0x5A827999u32) << $s };
		(@ $a:expr, $b:expr, $c:expr, $d:expr, $k:expr, $s:expr) =>
		{ ($a + g!($b, $c, $d) + $k) << $s };
	}
	macro_rules! h {
		($x:expr, $y:expr, $z:expr) => {$x ^ $y ^ $z};
		($a:expr, $b:expr, $c:expr, $d:expr, $k:expr, $s:expr) =>
		{ ($a + h!($b, $c, $d) + $k + 0x6ED9EBA1u32) << $s };
		(@ $a:expr, $b:expr, $c:expr, $d:expr, $k:expr, $s:expr) =>
		{ ($a + h!($b, $c, $d) + $k + 0x5C4DD124u32) << $s };
	}
	const K: [usize; 48] = [
		 0, 1, 2, 3, 4, 5, 6, 7, 8, 9,10,11,12,13,14,15,
		 7, 4,13, 1,10, 6,15, 3,12, 0, 9, 5,14, 2,11, 8,
		 3,10, 2, 4, 9,15, 8, 1,14, 7, 0, 6,11,13, 5,12
	];
	const S: [u32; 48] = [
		11,14,15,12, 5, 8, 7, 9,11,13,14,15, 6, 7, 9, 8,
		 7, 6, 8,13,11, 9, 7,15, 7,12,15, 9, 7,11,13,12,
		11,13,14, 7,14, 9,13,15, 6, 8,13, 6,12, 5, 7, 5
	];
	for x in m.chunks_exact(16) {
		let (mut aa, mut bb, mut cc, mut dd) = (a, b, c, d);
		let (mut aaa, mut bbb, mut ccc, mut ddd) = (a, b, c, d);
		for i in 0..48 {
			let temp1 =
				if i < 16 { f!(aa,bb,cc,dd, x[K[i]], S[i]) }
				else if i < 32 { g!(aa,bb,cc,dd, x[K[i]], S[i]) }
				else { h!(aa,bb,cc,dd, x[K[i]], S[i]) };
			let temp2 =
				if i < 16 { f!(@aaa,bbb,ccc,ddd, x[K[i]], S[i]) }
				else if i < 32 { g!(@aaa,bbb,ccc,ddd, x[K[i]], S[i]) }
				else { h!(@aaa,bbb,ccc,ddd, x[K[i]], S[i]) };
			aa = dd;
			dd = cc;
			cc = bb;
			bb = temp1;
			aaa = ddd;
			ddd = ccc;
			ccc = bbb;
			bbb = temp2;
		}
		let at = b + cc + ddd;
		let bt = c + dd + aaa;
		let ct = d + aa + bbb;
		let dt = a + bb + ccc;
		a = at; b = bt; c = ct; d = dt;
	}
	//输出
	Word32::to_le_bytes_array([a, b, c, d])
}

pub fn ripemd_160(message: &[u8]) -> [u8; 20] {
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
	let mut e: Word32 = Word32::from(0xc3d2e1f0);
	macro_rules! f {
		($x:expr, $y:expr, $z:expr) => {$x ^ $y ^ $z};
		($a:expr, $b:expr, $c:expr, $d:expr, $e:expr, $x:expr, $s:expr) =>
		{ (($a + f!($b, $c, $d) + $x) << $s) + $e };
		(@ $a:expr, $b:expr, $c:expr, $d:expr, $e:expr, $x:expr, $s:expr) =>
		{ (($a + f!($b, $c, $d) + $x) << $s) + $e };
	}
	macro_rules! g {
		($x:expr, $y:expr, $z:expr) => {($x & $y) | (!$x & $z)};
		($a:expr, $b:expr, $c:expr, $d:expr, $e:expr, $x:expr, $s:expr) =>
		{ (($a + g!($b, $c, $d) + $x + 0x5A827999u32) << $s) + $e };
		(@ $a:expr, $b:expr, $c:expr, $d:expr, $e:expr, $x:expr, $s:expr) =>
		{ (($a + g!($b, $c, $d) + $x + 0x7A6D76E9u32) << $s) + $e };
	}
	macro_rules! h {
		($x:expr, $y:expr, $z:expr) => {($x | !$y) ^ $z};
		($a:expr, $b:expr, $c:expr, $d:expr, $e:expr, $x:expr, $s:expr) =>
		{ (($a + h!($b, $c, $d) + $x + 0x6ED9EBA1u32) << $s) + $e };
		(@ $a:expr, $b:expr, $c:expr, $d:expr, $e:expr, $x:expr, $s:expr) =>
		{ (($a + h!($b, $c, $d) + $x + 0x6D703EF3u32) << $s) + $e };
	}
	macro_rules! i {
		($x:expr, $y:expr, $z:expr) => {($x & $z) | ($y & !$z)};
		($a:expr, $b:expr, $c:expr, $d:expr, $e:expr, $x:expr, $s:expr) =>
		{ (($a + i!($b, $c, $d) + $x + 0x8F1BBCDCu32) << $s) + $e };
		(@ $a:expr, $b:expr, $c:expr, $d:expr, $e:expr, $x:expr, $s:expr) =>
		{ (($a + i!($b, $c, $d) + $x + 0x5C4DD124u32) << $s) + $e };
	}
	macro_rules! j {
		($x:expr, $y:expr, $z:expr) => {$x ^ ($y | !$z)};
		($a:expr, $b:expr, $c:expr, $d:expr, $e:expr, $x:expr, $s:expr) =>
		{ (($a + j!($b, $c, $d) + $x + 0xA953FD4Eu32) << $s) + $e };
		(@ $a:expr, $b:expr, $c:expr, $d:expr, $e:expr, $x:expr, $s:expr) =>
		{ (($a + j!($b, $c, $d) + $x + 0x50A28BE6u32) << $s) + $e };
	}
	const K: [usize; 80] = [
		 0, 1, 2, 3, 4, 5, 6, 7, 8, 9,10,11,12,13,14,15,
		 7, 4,13, 1,10, 6,15, 3,12, 0, 9, 5, 2,14,11, 8,
		 3,10,14, 4, 9,15, 8, 1, 2, 7, 0, 6,13,11, 5,12,
		 1, 9,11,10, 0, 8,12, 4,13, 3, 7,15,14, 5, 6, 2,
		 4, 0, 5, 9, 7,12, 2,10,14, 1, 3, 8,11, 6,15,13,
	];
	const KK: [usize; 80] = [
		 5,14, 7, 0, 9, 2,11, 4,13, 6,15, 8, 1,10, 3,12,
		 6,11, 3, 7, 0,13, 5,10,14,15, 8,12, 4, 9, 1, 2,
		15, 5, 1, 3, 7,14, 6, 9,11, 8,12, 2,10, 0, 4,13,
		 8, 6, 4, 1, 3,11,15, 0, 5,12, 2,13, 9, 7,10,14,
		12,15,10, 4, 1, 5, 8, 7, 6, 2,13,14, 0, 3, 9,11
	];
	const S: [u32; 80] = [
		11,14,15,12, 5, 8, 7, 9,11,13,14,15, 6, 7, 9, 8,
		 7, 6, 8,13,11, 9, 7,15, 7,12,15, 9,11, 7,13,12,
		11,13, 6, 7,14, 9,13,15,14, 8,13, 6, 5,12, 7, 5,
		11,12,14,15,14,15, 9, 8, 9,14, 5, 6, 8, 6, 5,12,
		 9,15, 5,11, 6, 8,13,12, 5,12,13,14,11, 8, 5, 6
	];
	const SS: [u32; 80] = [
		 8, 9, 9,11,13,15,15, 5, 7, 7, 8,11,14,14,12, 6,
		 9,13,15, 7,12, 8, 9,11, 7, 7,12, 7, 6,15,13,11,
		 9, 7,15,11, 8, 6, 6,14,12,13, 5,14,13,13, 7, 5,
		15, 5, 8,11,14,14, 6,14, 6, 9,12, 9,12, 5,15, 8,
		 8, 5,12, 9,12, 5,14, 6, 8,13, 6, 5,15,13,11,11
	];
	for x in m.chunks_exact(16) {
		let (mut aa, mut bb, mut cc, mut dd, mut ee) = (a, b, c, d, e);
		let (mut aaa, mut bbb, mut ccc, mut ddd, mut eee) = (a, b, c, d, e);
		for i in 0..80 {
			let temp1 =
				if i < 16 { f!(aa,bb,cc,dd,ee, x[K[i]], S[i]) }
				else if i < 32 { g!(aa,bb,cc,dd,ee, x[K[i]], S[i]) }
				else if i < 48 { h!(aa,bb,cc,dd,ee, x[K[i]], S[i]) }
				else if i < 64 { i!(aa,bb,cc,dd,ee, x[K[i]], S[i]) }
				else { j!(aa,bb,cc,dd,ee, x[K[i]], S[i]) };
			let temp2 =
				if i < 16 { j!(@aaa,bbb,ccc,ddd,eee, x[KK[i]], SS[i]) }
				else if i < 32 { i!(@aaa,bbb,ccc,ddd,eee, x[KK[i]], SS[i]) }
				else if i < 48 { h!(@aaa,bbb,ccc,ddd,eee, x[KK[i]], SS[i]) }
				else if i < 64 { g!(@aaa,bbb,ccc,ddd,eee, x[KK[i]], SS[i]) }
				else { f!(@aaa,bbb,ccc,ddd,eee, x[KK[i]], SS[i]) };
			aa = ee;
			ee = dd;
			dd = cc << 10;
			cc = bb;
			bb = temp1;
			aaa = eee;
			eee = ddd;
			ddd = ccc << 10;
			ccc = bbb;
			bbb = temp2;
		}
		let at = b + cc + ddd;
		let bt = c + dd + eee;
		let ct = d + ee + aaa;
		let dt = e + aa + bbb;
		let et = a + bb + ccc;
		a = at; b = bt; c = ct; d = dt; e = et;
	}
	//输出
	Word32::to_le_bytes_array([a, b, c, d, e])
}

pub fn ripemd_128(message: &[u8]) -> [u8; 16] {
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
		($x:expr, $y:expr, $z:expr) => {$x ^ $y ^ $z};
		($a:expr, $b:expr, $c:expr, $d:expr, $x:expr, $s:expr) =>
		{ ($a + f!($b, $c, $d) + $x) << $s };
		(@ $a:expr, $b:expr, $c:expr, $d:expr, $x:expr, $s:expr) =>
		{ ($a + f!($b, $c, $d) + $x) << $s };
	}
	macro_rules! g {
		($x:expr, $y:expr, $z:expr) => {($x & $y) | (!$x & $z)};
		($a:expr, $b:expr, $c:expr, $d:expr, $x:expr, $s:expr) =>
		{ ($a + g!($b, $c, $d) + $x + 0x5A827999u32) << $s };
		(@ $a:expr, $b:expr, $c:expr, $d:expr, $x:expr, $s:expr) =>
		{ ($a + g!($b, $c, $d) + $x + 0x6D703EF3u32) << $s };
	}
	macro_rules! h {
		($x:expr, $y:expr, $z:expr) => {($x | !$y) ^ $z};
		($a:expr, $b:expr, $c:expr, $d:expr, $x:expr, $s:expr) =>
		{ ($a + h!($b, $c, $d) + $x + 0x6ED9EBA1u32) << $s };
		(@ $a:expr, $b:expr, $c:expr, $d:expr, $x:expr, $s:expr) =>
		{ ($a + h!($b, $c, $d) + $x + 0x5C4DD124u32) << $s };
	}
	macro_rules! i {
		($x:expr, $y:expr, $z:expr) => {($x & $z) | ($y & !$z)};
		($a:expr, $b:expr, $c:expr, $d:expr, $x:expr, $s:expr) =>
		{ ($a + i!($b, $c, $d) + $x + 0x8F1BBCDCu32) << $s };
		(@ $a:expr, $b:expr, $c:expr, $d:expr, $x:expr, $s:expr) =>
		{ ($a + i!($b, $c, $d) + $x + 0x50A28BE6u32) << $s };
	}
	const K: [usize; 80] = [
		 0, 1, 2, 3, 4, 5, 6, 7, 8, 9,10,11,12,13,14,15,
		 7, 4,13, 1,10, 6,15, 3,12, 0, 9, 5, 2,14,11, 8,
		 3,10,14, 4, 9,15, 8, 1, 2, 7, 0, 6,13,11, 5,12,
		 1, 9,11,10, 0, 8,12, 4,13, 3, 7,15,14, 5, 6, 2,
		 4, 0, 5, 9, 7,12, 2,10,14, 1, 3, 8,11, 6,15,13,
	];
	const KK: [usize; 80] = [
		 5,14, 7, 0, 9, 2,11, 4,13, 6,15, 8, 1,10, 3,12,
		 6,11, 3, 7, 0,13, 5,10,14,15, 8,12, 4, 9, 1, 2,
		15, 5, 1, 3, 7,14, 6, 9,11, 8,12, 2,10, 0, 4,13,
		 8, 6, 4, 1, 3,11,15, 0, 5,12, 2,13, 9, 7,10,14,
		12,15,10, 4, 1, 5, 8, 7, 6, 2,13,14, 0, 3, 9,11
	];
	const S: [u32; 80] = [
		11,14,15,12, 5, 8, 7, 9,11,13,14,15, 6, 7, 9, 8,
		 7, 6, 8,13,11, 9, 7,15, 7,12,15, 9,11, 7,13,12,
		11,13, 6, 7,14, 9,13,15,14, 8,13, 6, 5,12, 7, 5,
		11,12,14,15,14,15, 9, 8, 9,14, 5, 6, 8, 6, 5,12,
		 9,15, 5,11, 6, 8,13,12, 5,12,13,14,11, 8, 5, 6
	];
	const SS: [u32; 80] = [
		 8, 9, 9,11,13,15,15, 5, 7, 7, 8,11,14,14,12, 6,
		 9,13,15, 7,12, 8, 9,11, 7, 7,12, 7, 6,15,13,11,
		 9, 7,15,11, 8, 6, 6,14,12,13, 5,14,13,13, 7, 5,
		15, 5, 8,11,14,14, 6,14, 6, 9,12, 9,12, 5,15, 8,
		 8, 5,12, 9,12, 5,14, 6, 8,13, 6, 5,15,13,11,11
	];
	for x in m.chunks_exact(16) {
		let (mut aa, mut bb, mut cc, mut dd) = (a, b, c, d);
		let (mut aaa, mut bbb, mut ccc, mut ddd) = (a, b, c, d);
		for i in 0..64 {
			let temp1 =
				if i < 16 { f!(aa,bb,cc,dd, x[K[i]], S[i]) }
				else if i < 32 { g!(aa,bb,cc,dd, x[K[i]], S[i]) }
				else if i < 48 { h!(aa,bb,cc,dd, x[K[i]], S[i]) }
				else { i!(aa,bb,cc,dd, x[K[i]], S[i]) };
			let temp2 =
				if i < 16 { i!(@aaa,bbb,ccc,ddd, x[KK[i]], SS[i]) }
				else if i < 32 { h!(@aaa,bbb,ccc,ddd, x[KK[i]], SS[i]) }
				else if i < 48 { g!(@aaa,bbb,ccc,ddd, x[KK[i]], SS[i]) }
				else { f!(@aaa,bbb,ccc,ddd, x[KK[i]], SS[i]) };
				aa = dd;
				dd = cc;
				cc = bb;
				bb = temp1;
				aaa = ddd;
				ddd = ccc;
				ccc = bbb;
				bbb = temp2;
		}
		let at = b + cc + ddd;
		let bt = c + dd + aaa;
		let ct = d + aa + bbb;
		let dt = a + bb + ccc;
		a = at; b = bt; c = ct; d = dt;
	}
	//输出
	Word32::to_le_bytes_array([a, b, c, d,])
}