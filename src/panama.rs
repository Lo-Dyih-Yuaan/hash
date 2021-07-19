use crate::word::*;
pub fn panama(message: &[u8]) -> [u8; 32] {
	//补位
	let padding = 32 - message.len()%32;
	let mut m: Vec<u8> = Vec::with_capacity(message.len() + padding);
	m.extend_from_slice(message);
	m.push(0x01u8);
	m.extend(&vec![0; padding-1]);
	//
	let m: &[Word32] = &Word32::slice_from_le_bytes(&m);
	//
	let mut a: [Word32; 17] = [Word32::new(0); 17];
	let mut b: [[Word32; 8]; 32] = [[Word32::new(0); 8]; 32];
	macro_rules! lambda {
		($b:expr, $q:expr) => {{
			let mut temp = [[Word32::new(0); 8]; 32];
			for i in 0..17 {
				temp[i] = match i {
					0 => {
						let mut temp = [Word32::new(0); 8];
						for i in 0..8 {
							temp[i] = $b[31][i] ^ $q[i];
						}
						temp
					},
					25 => {
						let mut temp = [Word32::new(0); 8];
						for i in 0..8 {
							temp[i] = $b[24][i] ^ $b[31][(i+2)%8];
						}
						temp
					},
					_ => $b[i-1]
				};
			}
			temp
		}};
	}
	macro_rules! theta {
		($a:expr) => {{
			let mut temp = [Word32::new(0); 17];
			for i in 0..17 {
				temp[i] = $a[i] ^ $a[(i+1)%17] ^ $a[(i+4)%17];
			}
			temp
		}};
	}
	macro_rules! gamma {
		($a:expr) => {{
			let mut temp = [Word32::new(0); 17];
			for i in 0..17 {
				temp[i] = $a[i] ^ ($a[(i+1)%17] | !$a[(i+2)%17]);
			}
			temp
		}};
	}
	macro_rules! pi {
		($a:expr) => {{
			let mut temp = [Word32::new(0); 17];
			for i in 0..17 {
				temp[i] = $a[(7*i)%17]  << ((i*(i+1)/2)%32) as u32;
			}
			temp
		}};
	}
	macro_rules! sigma {
		($a:expr, $b:expr, $l:expr) => {{
			let mut temp = [Word32::new(0); 17];
			temp[0] = $a[0] ^ 1;
			for i in 0..8 {
				temp[i+1] = $a[i+1] ^ $l[i];
				temp[i+9] = $a[i+9] ^ $b[16][i];
			}
			temp
		}};
	}
	macro_rules! push{
		{$a:expr, $b:expr, $p:expr} => {{
			let temp = sigma!(theta!(pi!(gamma!($a))), $b, $p);
			$b = lambda!($b, $p);
			$a = temp;
		}}
	}
	macro_rules! pull{
		{$a:expr, $b:expr} => {{
			let temp = sigma!(theta!(pi!(gamma!($a))), $b, $b[4]);
			$b = lambda!($b, $a[1..9]);
			$a = temp;
		}}
	}
	for m in m.chunks_exact(8) {
		push!{a, b, m}
	}
	for _ in 0..32 {
		pull!{a, b}
	}
	//输出
	let mut temp: [Word32; 8] = [Word32::new(0); 8];
	temp.copy_from_slice(&a[9..17]);
	Word32::to_le_bytes_array(temp)
}
