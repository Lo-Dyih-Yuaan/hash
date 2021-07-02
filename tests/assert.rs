#[allow(unused_macros)]
macro_rules! assert_hash {
	($h:expr, $m:expr, $d:expr) => {
		assert_eq!($h($m).iter().fold(String::new(), |acc, x| format!("{}{:02x}", acc, x)), $d.to_lowercase())
	}
}
#[allow(unused_macros)]
macro_rules! assert_collision {
	($h:expr, $m1:expr, $m2:expr) => {
		assert_ne!($m1, $m2);
		assert_eq!($h($m1), $h($m2));
	}
}