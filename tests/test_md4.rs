#[macro_use] mod assert;
use hash::md4::md4;
#[test]
fn test_md4() {
	// RFC 1320
	assert_hash!(md4, b"", "31d6cfe0d16ae931b73c59d7e0c089c0");
	assert_hash!(md4, b"a", "bde52cb31de33e46245e05fbdbd6fb24");
	assert_hash!(md4, b"abc", "a448017aaf21d8525fc10ae87aa6729d");
	assert_hash!(md4, b"message digest", "d9130a8164549fe818874806e1c7014b");
	assert_hash!(md4, b"abcdefghijklmnopqrstuvwxyz", "d79e1c308aa5bbcdeea8ed63df412da9");
	assert_hash!(md4, b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789", "043f8582f241db351ce627e153e7f0e4");
	assert_hash!(md4, b"12345678901234567890123456789012345678901234567890123456789012345678901234567890", "e33b4ddc9c38f2199c3e7b164fcc0536");
	// Dobbertin, Hans. Cryptanalysis of MD4. Journal of Cryptology. 11(4):253–271.
	assert_collision!(md4, b"\x12^\x98\x13\x0b\x81\x8atZ\xf1\x1dM\x16\x15\x1d\x18\xac\tn-\xb9\xbdmK\xc8\xb0dd\x97\xc0\xa1\xfb\xe0{\xe1\xab\xb3\xd4\x1e\xed\xf5\xab A)\x10w '\x10w \xff\xfb\xff\xfd\xfb\xbf\xff\xff\xd2\xbetg", b"\x12^\x98\x13\x0b\x81\x8atZ\xf1\x1dM\x16\x15\x1d\x18\xac\tn-\xb9\xbdmK\xc8\xb0dd\x97\xc0\xa1\xfb\xe0{\xe1\xab\xb3\xd4\x1e\xed\xf5\xab A)\x10w (\x10w \xff\xfb\xff\xfd\xfb\xbf\xff\xff\xd2\xbetg");
	assert_collision!(md4, b"\x9bDt\x90&\xfc\x89\x10\xa2\x7f\xf3\x8b\xaf\rc\x1d$~$c\nCONTRACT\n\nAt the price of $176,495 Alf Blow", b"\x9bDt\x90&\xfc\x89\x10\xa2\x7f\xf3\x8b\xaf\rc\x1d$~$c\nCONTRACT\n\nAt the price of $276,495 Alf Blow");
}