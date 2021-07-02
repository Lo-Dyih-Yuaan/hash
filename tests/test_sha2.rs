#[macro_use] mod assert;
use hash::sha2::*;

#[test]
fn test_sha_224() {
	assert_hash!(sha_224, b"abc", "23097D223405D8228642A477BDA255B32AADBCE4BDA0B3F7E36C9DA7");
	assert_hash!(sha_224, b"abcdbcdecdefdefgefghfghighijhijkijkljklmklmnlmnomnopnopq", "75388B16512776CC5DBA5DA1FD890150B0C6455CB4F58B1952522525");
}

#[test]
fn test_sha_256() {
	assert_hash!(sha_256, b"abc", "BA7816BF8F01CFEA414140DE5DAE2223B00361A396177A9CB410FF61F20015AD");
	assert_hash!(sha_256, b"abcdbcdecdefdefgefghfghighijhijkijkljklmklmnlmnomnopnopq", "248D6A61D20638B8E5C026930C3E6039A33CE45964FF2167F6ECEDD419DB06C1");
}

#[test]
fn test_sha_384() {
	assert_hash!(sha_384, b"abc", "CB00753F45A35E8BB5A03D699AC65007272C32AB0EDED1631A8B605A43FF5BED8086072BA1E7CC2358BAECA134C825A7");
	assert_hash!(sha_384, b"abcdefghbcdefghicdefghijdefghijkefghijklfghijklmghijklmnhijklmnoijklmnopjklmnopqklmnopqrlmnopqrsmnopqrstnopqrstu", "09330C33F71147E83D192FC782CD1B4753111B173B3B05D22FA08086E3B0F712FCC7C71A557E2DB966C3E9FA91746039");
}

#[test]
fn test_sha_512() {
	assert_hash!(sha_512, b"abc", "DDAF35A193617ABACC417349AE20413112E6FA4E89A97EA20A9EEEE64B55D39A2192992A274FC1A836BA3C23A3FEEBBD454D4423643CE80E2A9AC94FA54CA49F");
	assert_hash!(sha_512, b"abcdefghbcdefghicdefghijdefghijkefghijklfghijklmghijklmnhijklmnoijklmnopjklmnopqklmnopqrlmnopqrsmnopqrstnopqrstu", "8E959B75DAE313DA8CF4F72814FC143F8F7779C6EB9F7FA17299AEADB6889018501D289E4900F7E4331B99DEC4B5433AC7D329EEB6DD26545E96E55B874BE909");
}

#[test]
fn test_sha_512_224() {
	assert_hash!(sha_512_t::<28>, b"abc", "4634270F707B6A54DAAE7530460842E20E37ED265CEEE9A43E8924AA");
	assert_hash!(sha_512_t::<28>, b"abcdefghbcdefghicdefghijdefghijkefghijklfghijklmghijklmnhijklmnoijklmnopjklmnopqklmnopqrlmnopqrsmnopqrstnopqrstu", "23FEC5BB94D60B23308192640B0C453335D664734FE40E7268674AF9");
}

#[test]
fn test_sha_512_256() {
	assert_hash!(sha_512_t::<32>, b"abc", "53048E2681941EF99B2E29B76B4C7DABE4C2D0C634FC6D46E0E2F13107E7AF23");
	assert_hash!(sha_512_t::<32>, b"abcdefghbcdefghicdefghijdefghijkefghijklfghijklmghijklmnhijklmnoijklmnopjklmnopqklmnopqrlmnopqrsmnopqrstnopqrstu", "3928E184FB8690F840DA3988121D31BE65CB9D3EF83EE6146FEAC861E19B563A");
}