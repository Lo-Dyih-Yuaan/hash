use core::convert::From;
use core::ops::{BitAnd, BitOr, BitXor, Not, Shl, Shr, Add, Sub, Mul};

#[derive(PartialEq, Eq, Clone)]
pub struct Bits(Vec<bool>);
impl Bits {
	pub fn len(&self) -> usize {
		self.0.len()
	}
}
macro_rules! bits_from_unsigned_integer {
	{$t:tt} => {
		impl From<$t> for Bits {
			fn from(n: $t) -> Self {
				let mut n = n;
				let mut v: Vec<bool> = Vec::new();
				while n > 0 {
					v.push(n&1 == 1);
					n >>= 1;
				}
				Self(v)
			}
		}
	}
}
bits_from_unsigned_integer!{u8}
bits_from_unsigned_integer!{u16}
bits_from_unsigned_integer!{u32}
bits_from_unsigned_integer!{u64}
bits_from_unsigned_integer!{u128}
bits_from_unsigned_integer!{usize}
impl Shl<usize> for Bits {
	type Output = Self;
	fn shl(self, n: usize) -> Self::Output {
		if self.0.is_empty() {
			Self(Vec::new())
		} else {
			let mut v = Vec::with_capacity(self.0.len() + n);
			v.extend(vec![false; n]);
			v.extend(self.0);
			Self(v)
		}
	}
}
impl Shr<usize> for Bits {
	type Output = Self;
	fn shr(self, n: usize) -> Self::Output {
		if self.0.len() <= n {
			Self(Vec::new())
		} else {
			Self(Vec::from(&self.0[n..]))
		}
	}
}

pub trait ConvertBytes<const BYTES: usize>: Sized + Copy {
	fn from_be_bytes(bytes: [u8; BYTES]) -> Self;
	fn from_le_bytes(bytes: [u8; BYTES]) -> Self;
	fn to_be_bytes(self) -> [u8; BYTES];
	fn to_le_bytes(self) -> [u8; BYTES];
	fn slice_from_be_bytes(bytes: &[u8]) -> Vec<Self> {
		let mut i: usize = 0;
		let mut n: [u8; BYTES] = [0; BYTES];
		let mut result = Vec::<Self>::with_capacity(bytes.len() / BYTES);
		for b in bytes {
			n[i] = *b;
			i += 1;
			if i == BYTES {
				i = 0;
				result.push(Self::from_be_bytes(n));
			}
		}
		result
	}
	fn slice_from_le_bytes(bytes: &[u8]) -> Vec<Self> {
		let mut i: usize = 0;
		let mut n: [u8; BYTES] = [0; BYTES];
		let mut result = Vec::<Self>::with_capacity(bytes.len() / BYTES);
		for b in bytes {
			n[i] = *b;
			i += 1;
			if i == BYTES {
				i = 0;
				result.push(Self::from_le_bytes(n));
			}
		}
		result
	}
	fn to_be_bytes_array<const N: usize, const M: usize>(a: [Self; N]) -> [u8; M] {
		assert_eq!(BYTES*N, M);
		let mut result: [u8; M] = [0; M];
		for i in 0..N {
			let bytes = a[i].to_be_bytes();
			for j in 0..BYTES {
				result[BYTES*i+j] = bytes[j];
			}
		}
		result
	}
	fn to_le_bytes_array<const N: usize, const M: usize>(a: [Self; N]) -> [u8; M] {
		assert_eq!(BYTES*N, M);
		let mut result: [u8; M] = [0; M];
		for i in 0..N {
			let bytes = a[i].to_le_bytes();
			for j in 0..BYTES {
				result[BYTES*i+j] = bytes[j];
			}
		}
		result
	}
	fn from_bits(bs: Bits) -> Self {
		assert!(bs.len() <= 8*BYTES);
		let mut bytes: [u8; BYTES] = [0; BYTES];
		let mut i: usize = 0;
		let mut n: usize = 0;
		for b in bs.0 {
			if b {
				bytes[n] |= 1 << i;
			}
			i += 1;
			if i == 8 {
				i = 0;
				n += 1;
			}
		}
		Self::from_le_bytes(bytes)
	}
}

macro_rules! simple_word_convert_impl {
	{$t:tt, $u:tt} => {
		impl $t {
			pub const fn new(n: $u) -> Self {
				Self(n)
			}
		}
		impl From<$u> for $t {
			fn from(n: $u) -> Self {
				Self(n)
			}
		}
		impl Into<$u> for $t {
			fn into(self) -> $u {
				self.0
			}
		}
	}
}
macro_rules! simple_word_convert_bytes_impl {
	{$t:tt, $u:tt, $n:literal} => {
		impl ConvertBytes<$n> for $t {
			fn from_be_bytes(bytes: [u8; $n]) -> Self {
				Self($u::from_be_bytes(bytes))
			}
			fn from_le_bytes(bytes: [u8; $n]) -> Self {
				Self($u::from_le_bytes(bytes))
			}
			fn to_be_bytes(self) -> [u8; $n] {
				self.0.to_be_bytes()
			}
			fn to_le_bytes(self) -> [u8; $n] {
				self.0.to_le_bytes()
			}
		}
	}
}
macro_rules! simple_word_ops_impl {
	{$t:tt, $u:tt} => {
		impl BitAnd for $t {
			type Output = Self;
			fn bitand(self, rhs: Self) -> Self::Output {
				Self(self.0 & rhs.0)
			}
		}
		impl BitAnd<$u> for $t {
			type Output = Self;
			fn bitand(self, rhs: $u) -> Self::Output {
				Self(self.0 & rhs)
			}
		}
		impl BitOr for $t {
			type Output = Self;
			fn bitor(self, rhs: Self) -> Self::Output {
				Self(self.0 | rhs.0)
			}
		}
		impl BitOr<$u> for $t {
			type Output = Self;
			fn bitor(self, rhs: $u) -> Self::Output {
				Self(self.0 | rhs)
			}
		}
		impl BitXor for $t {
			type Output = Self;
			fn bitxor(self, rhs: Self) -> Self::Output {
				Self(self.0 ^ rhs.0)
			}
		}
		impl BitXor<$u> for $t {
			type Output = Self;
			fn bitxor(self, rhs: $u) -> Self::Output {
				Self(self.0 ^ rhs)
			}
		}
		impl Not for $t {
			type Output = Self;
			fn not(self) -> Self::Output {
				Self(!self.0)
			}
		}
		impl Shl<u32> for $t {
			type Output = Self;
			fn shl(self, n: u32) -> Self::Output {
				Self(self.0.rotate_left(n))
			}
		}
		impl Shr<u32> for $t {
			type Output = Self;
			fn shr(self, n: u32) -> Self::Output {
				Self(self.0.rotate_right(n))
			}
		}
		impl Shl<&str> for $t {
			type Output = Self;
			fn shl(self, n: &str) -> Self::Output {
				Self(self.0 << n.parse::<u32>().unwrap())
			}
		}
		impl Shr<&str> for $t {
			type Output = Self;
			fn shr(self, n: &str) -> Self::Output {
				Self(self.0 >> n.parse::<u32>().unwrap())
			}
		}
		impl Add for $t {
			type Output = Self;
			fn add(self, rhs: Self) -> Self::Output {
				Self(self.0.wrapping_add(rhs.0))
			}
		}
		impl Add<$u> for $t {
			type Output = Self;
			fn add(self, rhs: $u) -> Self::Output {
				Self(self.0.wrapping_add(rhs))
			}
		}
		impl Sub for $t {
			type Output = Self;
			fn sub(self, rhs: Self) -> Self::Output {
				Self(self.0.wrapping_sub(rhs.0))
			}
		}
		impl Sub<$u> for $t {
			type Output = Self;
			fn sub(self, rhs: $u) -> Self::Output {
				Self(self.0.wrapping_sub(rhs))
			}
		}
		impl Mul for $t {
			type Output = Self;
			fn mul(self, rhs: Self) -> Self::Output {
				Self(self.0.wrapping_mul(rhs.0))
			}
		}
		impl Mul<$u> for $t {
			type Output = Self;
			fn mul(self, rhs: $u) -> Self::Output {
				Self(self.0.wrapping_mul(rhs))
			}
		}
	}
}
macro_rules! create_simple_word {
	{$t:tt, $u:tt, $n:literal} => {
		#[derive(PartialEq, Eq, Clone, Copy)]
		pub struct $t($u);
		simple_word_convert_impl!{$t, $u}
		simple_word_convert_bytes_impl!{$t, $u, $n}
		simple_word_ops_impl!{$t, $u}
	}
}

create_simple_word!{Word16, u16, 2}
create_simple_word!{Word32, u32, 4}
create_simple_word!{Word64, u64, 8}
create_simple_word!{Word128, u128, 16}

#[derive(PartialEq, Eq, Clone, Copy)]
pub struct Word256(u128, u128);
impl From<&str> for Word256 {
	fn from(s: &str) -> Self {
		let s = s.to_ascii_lowercase();
		assert!(s.starts_with("0x"));
		let s = &s[2..];
		let n0 = u128::from_str_radix(&s[s.len()-32..s.len()], 16).unwrap();
		let n1 = u128::from_str_radix(&s[s.len()-64..s.len()-32], 16).unwrap();
		Self(n0, n1)
	}
}
impl Word256 {
	pub const fn new(s: &str) -> Self {
		const TABLE: [i8; 128] = [
			-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,
			-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,
			-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,
			 0, 1, 2, 3, 4, 5, 6, 7, 8, 9,-1,-1,-1,-1,-1,-1,
			-1,10,11,12,13,14,15,-1,-1,-1,-1,-1,-1,-1,-1,-1,
			-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,
			-1,10,11,12,13,14,15,-1,-1,-1,-1,-1,-1,-1,-1,-1,
			-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1
		];
		macro_rules! hex_parse {
			(@ $s:expr; $i:expr) => {
				TABLE[$s[$i] as usize] as u128
			};
			(@ $s:expr; $i:expr, $($is:expr),+) => {
				hex_parse!(@ $s; $($is),+) * 0x10u128 + TABLE[$s[$i] as usize] as u128
			};
			($s:expr, $i:literal) => {
				hex_parse!(@ $s;
					$i+31, $i+30,
					$i+29, $i+28, $i+27, $i+26, $i+25, $i+24, $i+23, $i+22, $i+21, $i+20,
					$i+19, $i+18, $i+17, $i+16, $i+15, $i+14, $i+13, $i+12, $i+11, $i+10,
					$i+9, $i+8, $i+7, $i+6, $i+5, $i+4, $i+3, $i+2, $i+1, $i
				)
			}
		}
		Self(
			hex_parse!(s.as_bytes(), 34),
			hex_parse!(s.as_bytes(), 2)
		)
	}
}
impl From<u128> for Word256 {
	fn from(n: u128) -> Self {
		Self(n, 0)
	}
}
impl ConvertBytes<32> for Word256 {
	fn from_be_bytes(bytes: [u8; 32]) -> Self {
		let mut v: [u8; 16] = [0; 16];
		v.copy_from_slice(&bytes[0..16]);
		let n1 = u128::from_be_bytes(v);
		v.copy_from_slice(&bytes[16..32]);
		let n0 = u128::from_be_bytes(v);
		Self(n0, n1)
	}
	fn from_le_bytes(bytes: [u8; 32]) -> Self {
		let mut v: [u8; 16] = [0; 16];
		v.copy_from_slice(&bytes[0..16]);
		let n0 = u128::from_le_bytes(v);
		v.copy_from_slice(&bytes[16..32]);
		let n1 = u128::from_le_bytes(v);
		Self(n0, n1)
	}
	fn to_be_bytes(self) -> [u8; 32] {
		let mut result: [u8; 32] = [0; 32];
		let (v1, v0) = result.split_at_mut(16);
		v0.copy_from_slice(&self.0.to_be_bytes());
		v1.copy_from_slice(&self.1.to_be_bytes());
		result
	}
	fn to_le_bytes(self) -> [u8; 32] {
		let mut result: [u8; 32] = [0; 32];
		let (v0, v1) = result.split_at_mut(16);
		v0.copy_from_slice(&self.0.to_le_bytes());
		v1.copy_from_slice(&self.1.to_le_bytes());
		result
	}
}
impl BitAnd for Word256 {
	type Output = Self;
	fn bitand(self, rhs: Self) -> Self::Output {
		Self(self.0 & rhs.0, self.1 & rhs.1)
	}
}
impl BitOr for Word256 {
	type Output = Self;
	fn bitor(self, rhs: Self) -> Self::Output {
		Self(self.0 | rhs.0, self.1 | rhs.1)
	}
}
impl BitXor for Word256 {
	type Output = Self;
	fn bitxor(self, rhs: Self) -> Self::Output {
		Self(self.0 ^ rhs.0, self.1 ^ rhs.1)
	}
}
impl Add for Word256 {
	type Output = Self;
	fn add(self, rhs: Self) -> Self::Output {
		let (n0, of) = self.0.overflowing_add(rhs.0);
		let n1 = self.1.wrapping_add(rhs.1).wrapping_add(of as u128);
		Self(n0, n1)
	}
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub struct Word512(u128, u128, u128, u128);
impl From<&str> for Word512 {
	fn from(s: &str) -> Self {
		let s = s.to_ascii_lowercase();
		assert!(s.starts_with("0x"));
		let s = &s[2..];
		let n0 = u128::from_str_radix(&s[s.len()-32..s.len()], 16).unwrap();
		let n1 = u128::from_str_radix(&s[s.len()-64..s.len()-32], 16).unwrap();
		let n2 = u128::from_str_radix(&s[s.len()-96..s.len()-64], 16).unwrap();
		let n3 = u128::from_str_radix(&s[s.len()-128..s.len()-96], 16).unwrap();
		Self(n0, n1, n2, n3)
	}
}
impl Word512 {
	pub const fn new(s: &str) -> Self {
		const TABLE: [i8; 128] = [
			-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,
			-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,
			-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,
			 0, 1, 2, 3, 4, 5, 6, 7, 8, 9,-1,-1,-1,-1,-1,-1,
			-1,10,11,12,13,14,15,-1,-1,-1,-1,-1,-1,-1,-1,-1,
			-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,
			-1,10,11,12,13,14,15,-1,-1,-1,-1,-1,-1,-1,-1,-1,
			-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1
		];
		macro_rules! hex_parse {
			(@ $s:expr; $i:expr) => {
				TABLE[$s[$i] as usize] as u128
			};
			(@ $s:expr; $i:expr, $($is:expr),+) => {
				hex_parse!(@ $s; $($is),+) * 0x10u128 + TABLE[$s[$i] as usize] as u128
			};
			($s:expr, $i:literal) => {
				hex_parse!(@ $s;
					$i+31, $i+30,
					$i+29, $i+28, $i+27, $i+26, $i+25, $i+24, $i+23, $i+22, $i+21, $i+20,
					$i+19, $i+18, $i+17, $i+16, $i+15, $i+14, $i+13, $i+12, $i+11, $i+10,
					$i+9, $i+8, $i+7, $i+6, $i+5, $i+4, $i+3, $i+2, $i+1, $i
				)
			}
		}
		Self(
			hex_parse!(s.as_bytes(), 98),
			hex_parse!(s.as_bytes(), 66),
			hex_parse!(s.as_bytes(), 34),
			hex_parse!(s.as_bytes(), 2)
		)
	}
}
impl From<u128> for Word512 {
	fn from(n: u128) -> Self {
		Self(n, 0, 0, 0)
	}
}
impl ConvertBytes<64> for Word512 {
	fn from_be_bytes(bytes: [u8; 64]) -> Self {
		let mut v: [u8; 16] = [0; 16];
		v.copy_from_slice(&bytes[0..16]);
		let n3 = u128::from_be_bytes(v);
		v.copy_from_slice(&bytes[16..32]);
		let n2 = u128::from_be_bytes(v);
		v.copy_from_slice(&bytes[32..48]);
		let n1 = u128::from_be_bytes(v);
		v.copy_from_slice(&bytes[48..64]);
		let n0 = u128::from_be_bytes(v);
		Self(n0, n1, n2, n3)
	}
	fn from_le_bytes(bytes: [u8; 64]) -> Self {
		let mut v: [u8; 16] = [0; 16];
		v.copy_from_slice(&bytes[0..16]);
		let n0 = u128::from_le_bytes(v);
		v.copy_from_slice(&bytes[16..32]);
		let n1 = u128::from_le_bytes(v);
		v.copy_from_slice(&bytes[32..48]);
		let n2 = u128::from_le_bytes(v);
		v.copy_from_slice(&bytes[48..64]);
		let n3 = u128::from_le_bytes(v);
		Self(n0, n1, n2, n3)
	}
	fn to_be_bytes(self) -> [u8; 64] {
		let mut result: [u8; 64] = [0; 64];
		let (v3, v210) = result.split_at_mut(16);
		let (v2, v10) = v210.split_at_mut(16);
		let (v1, v0) = v10.split_at_mut(16);
		v0.copy_from_slice(&self.0.to_be_bytes());
		v1.copy_from_slice(&self.1.to_be_bytes());
		v2.copy_from_slice(&self.2.to_be_bytes());
		v3.copy_from_slice(&self.3.to_be_bytes());
		result
	}
	fn to_le_bytes(self) -> [u8; 64] {
		let mut result: [u8; 64] = [0; 64];
		let (v0, v123) = result.split_at_mut(16);
		let (v1, v23) = v123.split_at_mut(16);
		let (v2, v3) = v23.split_at_mut(16);
		v0.copy_from_slice(&self.0.to_le_bytes());
		v1.copy_from_slice(&self.1.to_le_bytes());
		v2.copy_from_slice(&self.2.to_le_bytes());
		v3.copy_from_slice(&self.3.to_le_bytes());
		result
	}
}
impl BitAnd for Word512 {
	type Output = Self;
	fn bitand(self, rhs: Self) -> Self::Output {
		Self(self.0 & rhs.0, self.1 & rhs.1, self.2 & rhs.2, self.3 & rhs.3)
	}
}
impl BitOr for Word512 {
	type Output = Self;
	fn bitor(self, rhs: Self) -> Self::Output {
		Self(self.0 | rhs.0, self.1 | rhs.1, self.2 | rhs.2, self.3 | rhs.3)
	}
}
impl BitXor for Word512 {
	type Output = Self;
	fn bitxor(self, rhs: Self) -> Self::Output {
		Self(self.0 ^ rhs.0, self.1 ^ rhs.1, self.2 ^ rhs.2, self.3 ^ rhs.3)
	}
}
impl Add for Word512 {
	type Output = Self;
	fn add(self, rhs: Self) -> Self::Output {
		let (n0, of0) = self.0.overflowing_add(rhs.0);
		let (n1, of1) = self.1.overflowing_add(rhs.1);
		let (n1, ofc) = n1.overflowing_add(of0 as u128);
		let of1 = of1 || ofc;
		let (n2, of2) = self.2.overflowing_add(rhs.2);
		let (n2, ofc) = n2.overflowing_add(of1 as u128);
		let of2 = of2 || ofc;
		let n3 = self.3.wrapping_add(rhs.3).wrapping_add(of2 as u128);
		Self(n0, n1, n2, n3)
	}
}