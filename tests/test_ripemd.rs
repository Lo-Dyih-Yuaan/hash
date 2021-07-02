#[macro_use] mod assert;
use hash::ripemd::{ripemd, ripemd_160, ripemd_128};
#[test]
fn test_ripemd() {
	// RFC 1320
	assert_hash!(ripemd, b"", "9f73aa9b372a9dacfb86a6108852e2d9");
	assert_hash!(ripemd, b"a", "486f74f790bc95ef7963cd2382b4bbc9");
	assert_hash!(ripemd, b"abc", "3f14bad4c2f9b0ea805e5485d3d6882d");
	assert_hash!(ripemd, b"message digest", "5f5c7ebe1abbb3c7036482942d5f9d49");
	assert_hash!(ripemd, b"abcdefghijklmnopqrstuvwxyz", "ff6e1547494251a1cca6f005a6eaa2b4");
	assert_hash!(ripemd, b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789", "ff418a5aed3763d8f2ddf88a29e62486");
	assert_hash!(ripemd, b"12345678901234567890123456789012345678901234567890123456789012345678901234567890", "dfd6b45f60fe79bbbde87c6bfc6580a5");
	// Wang, Xiaoyun; Feng, Dengguo; Lai, Xuejia; Yu, Hongbo. Collisions for Hash Functions MD4, MD5, HAVAL-128 and RIPEMD.
	assert_collision!(ripemd, b"\x8e\xaf\x9fWy\xf5\xec\t\xbajJW\x115Ax\xa4\x10\xb4\xa2\x9fl/\xad, V\x0b\x11yuM\xe7\xaa\xde\x0b\xf2\x91\xbcx}m\xbcG\xb1\xd1\xbd\x9a\x15 ]\xa4\xff\x04q\x81\xa8XG&\xa5N\x06a", b"\x8e\xaf\x9fWy\xf5\xec\t\xbajJW\x115Qx\xa4\x10\xb4\xa2\x9fl/\xad, V\x0b\x11yuM\xe7\xaa\xde\x0b\xf2\x91\xbcx}m\xc0\xc7\xb1\xd1\xbd\x9a\x15 ]\xa4\xff\x04q\x81\xa8XG&\xa5N\x06\xe1");
	assert_collision!(ripemd, b"\x8e\xaf\x9fWy\xf5\xec\t\xbajJW\x115Ax\xa4\x10\xb4\xa2\x9fl/\xad, V\x0b\x11yuM\xe7\xaa\xde\x0b\xf2\x91\xbcx}m\xbcG\xb1\xd1\xbd\x9a\x15 ]\xa4\xff\x04\xa5\xa0\xa8X\x8d\xb1\xb6f\x0c\xe7", b"\x8e\xaf\x9fWy\xf5\xec\t\xbajJW\x115Qx\xa4\x10\xb4\xa2\x9fl/\xad, V\x0b\x11yuM\xe7\xaa\xde\x0b\xf2\x91\xbcx}m\xc0\xc7\xb1\xd1\xbd\x9a\x15 ]\xa4\xff\x04\xa5\xa0\xa8X\x8d\xb1\xb6f\x0cg");
}

#[test]
fn test_ripemd_160() {
	assert_hash!(ripemd_160, b"", "9c1185a5c5e9fc54612808977ee8f548b2258d31");
	assert_hash!(ripemd_160, b"a", "0bdc9d2d256b3ee9daae347be6f4dc835a467ffe");
	assert_hash!(ripemd_160, b"abc", "8eb208f7e05d987a9b044a8e98c6b087f15a0bfc");
	assert_hash!(ripemd_160, b"message digest", "5d0689ef49d2fae572b881b123a85ffa21595f36");
	assert_hash!(ripemd_160, b"abcdefghijklmnopqrstuvwxyz", "f71c27109c692c1b56bbdceb5b9d2865b3708dbc");
	assert_hash!(ripemd_160, b"abcdbcdecdefdefgefghfghighijhijkijkljklmklmnlmnomnopnopq", "12a053384a9c0c88e405a06c27dcf49ada62eb2b");
	assert_hash!(ripemd_160, b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789", "b0e20b6e3116640286ed3a87a5713079b21f5189");
	assert_hash!(ripemd_160, &b"1234567890".repeat(8), "9b752e45573d4b39f4dbd3323cab82bf63326bfb");
	assert_hash!(ripemd_160, &b"a".repeat(1000000), "52783243c1697bdbe16d37f97f68f08325dc1528");
}

#[test]
fn test_ripemd_128() {
	assert_hash!(ripemd_128, b"", "cdf26213a150dc3ecb610f18f6b38b46");
	assert_hash!(ripemd_128, b"a", "86be7afa339d0fc7cfc785e72f578d33");
	assert_hash!(ripemd_128, b"abc", "c14a12199c66e4ba84636b0f69144c77");
	assert_hash!(ripemd_128, b"message digest", "9e327b3d6e523062afc1132d7df9d1b8");
	assert_hash!(ripemd_128, b"abcdefghijklmnopqrstuvwxyz", "fd2aa607f71dc8f510714922b371834e");
	assert_hash!(ripemd_128, b"abcdbcdecdefdefgefghfghighijhijkijkljklmklmnlmnomnopnopq", "a1aa0689d0fafa2ddc22e88b49133a06");
	assert_hash!(ripemd_128, b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789", "d1e959eb179c911faea4624c60c5c702");
	assert_hash!(ripemd_128, &b"1234567890".repeat(8), "3f45ef194732c2dbb2c4a2c769795fa3");
	assert_hash!(ripemd_128, &b"a".repeat(1000000), "4a7f5723f954eba1216c9d8f6320431f");
}