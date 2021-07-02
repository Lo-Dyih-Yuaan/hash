#[macro_use] mod assert;
use hash::sm3::sm3;
#[test]
fn test_sm3() {
	// GM/T 0004-2012
	assert_hash!(sm3, b"abc", "66c7f0f462eeedd9d1f2d46bdc10e4e24167c4875cf2f7a2297da02b8f4ba8e0");
	assert_hash!(sm3, &b"abcd".repeat(16), "debe9ff92275b8a138604889c18e5a4d6fdb70e5387e5765293dcba39c0c5732");
}
