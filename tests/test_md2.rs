#[macro_use] mod assert;
use hash::md2::md2;
#[test]
fn test_md2() {
	// RFC 1319
	assert_hash!(md2, b"", "8350e5a3e24c153df2275c9f80692773");
	assert_hash!(md2, b"a", "32ec01ec4a6dac72c0ab96fb34c0b5d1");
	assert_hash!(md2, b"abc", "da853b0d3f88d99b30283a69e6ded6bb");
	assert_hash!(md2, b"message digest", "ab4f496bfb2a530b219ff33031fe06b0");
	assert_hash!(md2, b"abcdefghijklmnopqrstuvwxyz", "4e8ddff3650292ab5a4108c3aa47940b");
	assert_hash!(md2, b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789", "da33def2a42df13975352846c30338cd");
	assert_hash!(md2, b"12345678901234567890123456789012345678901234567890123456789012345678901234567890", "d5976f79d83d3a0dc9806c3c66f3efd8");
}
