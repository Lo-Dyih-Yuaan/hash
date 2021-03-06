#[macro_use] mod assert;
use hash::whirlpool::*;
#[test]
fn test_whirlpool() {
	assert_hash!(whirlpool, b"", "19FA61D75522A4669B44E39C1D2E1726C530232130D407F89AFEE0964997F7A73E83BE698B288FEBCF88E3E03C4F0757EA8964E59B63D93708B138CC42A66EB3");
	assert_hash!(whirlpool, b"a", "8ACA2602792AEC6F11A67206531FB7D7F0DFF59413145E6973C45001D0087B42D11BC645413AEFF63A42391A39145A591A92200D560195E53B478584FDAE231A");
	assert_hash!(whirlpool, b"abc", "4E2448A4C6F486BB16B6562C73B4020BF3043E3A731BCE721AE1B303D97E6D4C7181EEBDB6C57E277D0E34957114CBD6C797FC9D95D8B582D225292076D4EEF5");
	assert_hash!(whirlpool, b"message digest", "378C84A4126E2DC6E56DCC7458377AAC838D00032230F53CE1F5700C0FFB4D3B8421557659EF55C106B4B52AC5A4AAA692ED920052838F3362E86DBD37A8903E");
	assert_hash!(whirlpool, b"abcdefghijklmnopqrstuvwxyz", "F1D754662636FFE92C82EBB9212A484A8D38631EAD4238F5442EE13B8054E41B08BF2A9251C30B6A0B8AAE86177AB4A6F68F673E7207865D5D9819A3DBA4EB3B");
	assert_hash!(whirlpool, b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789", "DC37E008CF9EE69BF11F00ED9ABA26901DD7C28CDEC066CC6AF42E40F82F3A1E08EBA26629129D8FB7CB57211B9281A65517CC879D7B962142C65F5A7AF01467");
	assert_hash!(whirlpool, &b"1234567890".repeat(8), "466EF18BABB0154D25B9D38A6414F5C08784372BCCB204D6549C4AFADB6014294D5BD8DF2A6C44E538CD047B2681A51A2C60481E88C5A20B2C2A80CF3A9A083B");
	assert_hash!(whirlpool, b"abcdbcdecdefdefgefghfghighijhijk", "2A987EA40F917061F5D6F0A0E4644F488A7A5A52DEEE656207C562F988E95C6916BDC8031BC5BE1B7B947639FE050B56939BAAA0ADFF9AE6745B7B181C3BE3FD");
	assert_hash!(whirlpool, &[b'a'; 1000000], "0C99005BEB57EFF50A7CF005560DDF5D29057FD86B20BFD62DECA0F1CCEA4AF51FC15490EDDC47AF32BB2B66C34FF9AD8C6008AD677F77126953B226E4ED8B01");
}
#[test]
fn test_whirlpool_0() {
	assert_hash!(whirlpool_0, b"", "B3E1AB6EAF640A34F784593F2074416ACCD3B8E62C620175FCA0997B1BA2347339AA0D79E754C308209EA36811DFA40C1C32F1A2B9004725D987D3635165D3C8");
	assert_hash!(whirlpool_0, b"a", "F4B620445AE62431DBD6DBCEC64D2A3031CD2F48DF5E755F30B3D069929ED4B4EDA0AE65441BC86746021FB7F2167F84D67566EFABA003F0ABB67A42A2CE5B13");
	assert_hash!(whirlpool_0, b"abc", "54EE18B0BBD4DD38A211699F2829793156E5842DF502A2A25995C6C541F28CC050FF57D4AF772DEE7CEDCC4C34C3B8EC06446C6657F2F36C2C06464399879B86");
	assert_hash!(whirlpool_0, b"message digest", "29E158BA336CE7F930115178A6C86019F0F413ADB283D8F0798AF06CA0A06D6D6F295A333B1C24BDA2F429AC918A3748AEF90F7A2C8BFB084D5F979CF4E7B2B5");
	assert_hash!(whirlpool_0, b"abcdefghijklmnopqrstuvwxyz", "5AC9757E1407432DAF348A972B8AD4A65C1123CF1F9B779C1AE7EE2D540F30B3CEFA8F98DCA5FBB42084C5C2F161A7B40EB6B4A1FC7F9AAAB92A4BB6002EDC5E");
	assert_hash!(whirlpool_0, b"abcdbcdecdefdefgefghfghighijhijkijkljklmklmnlmnomnopnopq", "D570EEA0004210EAF957DF144AB5241BDD4E2191789D4BA6A848CBCB0DAD70439A603D3BB810560F8DFE4C6CF5E2B8968547BC729A9950C9DA6EA02CAE6881D7");
	assert_hash!(whirlpool_0, b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789", "CAE4175F09753DE84974CFA968621092FE41EE9DE913919C2B452E6CB424056721D640E563F628F29DD3BD0030837AE4AC14AA17308505A92E5F7A92F112BE75");
	assert_hash!(whirlpool_0, &b"1234567890".repeat(8), "E5965B4565B041A0D459610E5E48E944C4830CD16FEBA02D9D263E7DA8DE6A6B88966709BF28A5328D928312E7A172DA4CFF72FE6DE02277DAE4B1DBA49689A2");
	assert_hash!(whirlpool_0, &[b'a'; 1000000], "BB6CBA9730D6C029C0C15FB7A2AA3597CF9442DAD96A676C5EE9A1D55F1D64D5E0D1ED0E71250ED960A1BD2E065642CFFF1C976E061BAB70D6C54D284EAAEFB9");
}
#[test]
fn test_whirlpool_t() {
	assert_hash!(whirlpool_t, b"", "470F0409ABAA446E49667D4EBE12A14387CEDBD10DD17B8243CAD550A089DC0FEEA7AA40F6C2AAAB71C6EBD076E43C7CFCA0AD32567897DCB5969861049A0F5A");
	assert_hash!(whirlpool_t, b"a", "B290E0E7931025ED37043AD568F0036B40E6BFF8F7455868780F47EF7B5D693E62448029A9351CD85AC29CB0725E4CFEB996A92F2B8DA8768483AC58EC0E492C");
	assert_hash!(whirlpool_t, b"abc", "8AFC0527DCC0A19623860EF2369D0E25DE8EBE2ABAA40F598AFAF6B07C002ED73E4FC0FC220FD4F54F74B5D6B07AA57764C3DBDCC2CDD919D89FA8155A34B841");
	assert_hash!(whirlpool_t, b"message digest", "817EADF8EFCA5AFBC11F71D0814E03A8D569C90F748C8603597A7A0DE3C8D55F528199010218249517B58B14BEE523515608754B53A3CCA35C0865BA5E361431");
	assert_hash!(whirlpool_t, b"abcdefghijklmnopqrstuvwxyz", "4AFC2B07BDDC8417635FCB43E695E16F45E116C226DD84339EB95C2CCB39E7ACBE1AF8F7B1F3BD380077E71929498BC968200371F9299015434D1DF109A0AA1D");
	assert_hash!(whirlpool_t, b"abcdbcdecdefdefgefghfghighijhijkijkljklmklmnlmnomnopnopq", "D6111BC1E1301197C24B02F307E6DFD9349EFED2716540AC1E2BD1A6D627F6D336643F3DBE8E80DE772A2B890AA0088265B099FB4DC90B5FB3845995980E54F0");
	assert_hash!(whirlpool_t, b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789", "0F960EC9AB7D0C7E355A423D1EF4911A39797C836A71414276AFEB8FA475DBA0C348547143162F3212EDF1FB8D8C652A11A579A399C2DBD837FE8608F5096131");
	assert_hash!(whirlpool_t, &b"1234567890".repeat(8), "6AE43784C69D01C273BBA40F8411495167909E0C1ACC241473D44E27BC8641E646535D38FCE20604941988C387C201CFF199C8FA2AFBEDD036D66202892A7EEE");
	assert_hash!(whirlpool_t, &[b'a'; 1000000], "0EE18BA7CA7EE091DACE6285661EEDF819A8FA17620F72AEFFE5AA62C462138B626AA09072A10FCBCFE7F7FF22DB2F4D6D1F0771856C4A7924F9B0E4044D9112");
}
