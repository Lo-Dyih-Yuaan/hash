#[macro_use] mod assert;
use hash::gost94::gost94;
#[test]
fn test_gost94() {
	assert_hash!(gost94, b"This is message, length=32 bytes", "b1c466d37519b82e8319819ff32595e047a28cb6f83eff1c6916a815a637fffa");
	assert_hash!(gost94, b"Suppose the original message has length = 50 bytes", "471aba57a60a770d3a76130635c1fbea4ef14de51f78b4ae57dd893b62f55208");
}
