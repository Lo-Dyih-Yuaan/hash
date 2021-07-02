#[macro_use] mod assert;
use hash::sha1::sha1;
#[test]
fn test_sha1() {
	// RFC 3174
	assert_hash!(sha1, b"abc", "a9993e364706816aba3e25717850c26c9cd0d89d");
	assert_hash!(sha1, &b"abcdefghijklmnopq".windows(4).collect::<Vec<_>>().concat(), "84983e441c3bd26ebaae4aa1f95129e5e54670f1");
	assert_hash!(sha1, &b"a".repeat(1000000), "34aa973cd4c4daa4f61eeb2bdbad27316534016f");
	assert_hash!(sha1, &b"01234567".repeat(80), "dea356a2cddd90c7a7ecedc5ebb563934f460452");
	//
	assert_collision!(sha1, include_bytes!("shattered-1.pdf"), include_bytes!("shattered-2.pdf"));
	assert_collision!(sha1, include_bytes!("good.pdf"), include_bytes!("bad.pdf"));
}
