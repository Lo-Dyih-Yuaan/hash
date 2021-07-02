#[macro_use] mod assert;
use hash::sha0::sha0;
#[test]
fn test_sha0() {
	// FIPS PUB 180
	assert_hash!(sha0, b"abc", "0164b8a914cd2a5e74c4f7ff082c4d97f1edf880");
	assert_hash!(sha0, b"abcdbcdecdefdefgefghfghighijhijkijkljklmklmnlmnomnopnopq", "d2516ee1acfa5baf33dfc1c471e438449ef134c8");
	assert_hash!(sha0, &b"a".repeat(1000000), "3232affa48628a26653b5aaa44541fd90d690603");
}
