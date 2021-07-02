#[macro_use] mod assert;
use hash::stribog::{stribog_512, stribog_256};
#[test]
fn test_stribog_512() {
	assert_hash!(stribog_512, b"012345678901234567890123456789012345678901234567890123456789012", "1b54d01a4af5b9d5cc3d86d68d285462b19abc2475222f35c085122be4ba1ffa00ad30f8767b3a82384c6574f024c311e2a481332b08ef7f41797891c1646f48");
	assert_hash!(stribog_512, b"\xd1\xe5 \xe2\xe5\xf2\xf0\xe8, \xd1\xf2\xf0\xe8\xe1\xee\xe6\xe8 \xe2\xed\xf3\xf6\xe8, \xe2\xe5\xfe\xf2\xfa \xf1 \xec\xee\xf0\xff \xf1\xf2\xf0\xe5\xeb\xe0\xec\xe8 \xed\xe0 \xf5\xf0\xe0\xe1\xf0\xfb\xff \xef\xeb\xfa\xea\xfb \xc8\xe3\xee\xf0\xe5\xe2\xfb", "1e88e62226bfca6f9994f1f2d51569e0daf8475a3b0fe61a5300eee46d961376035fe83549ada2b8620fcd7c496ce5b33f0cb9dddc2b6460143b03dabac9fb28"); //Се ветри, Стрибожи внуци, веютъ с моря стрелами на храбрыя плъкы Игоревы
	assert_hash!(stribog_512, b"", "8e945da209aa869f0455928529bcae4679e9873ab707b55315f56ceb98bef0a7362f715528356ee83cda5f2aac4c6ad2ba3a715c1bcd81cb8e9f90bf4c1c1a8a");
	assert_hash!(stribog_512, &b"\0".repeat(64), "b0fd29ac1b0df441769ff3fdb8dc564df67721d6ac06fb28ceffb7bbaa7948c6c014ac999235b58cb26fb60fb112a145d7b4ade9ae566bf2611402c552d20db7");
}

#[test]
fn test_stribog_256() {
	assert_hash!(stribog_256, b"012345678901234567890123456789012345678901234567890123456789012", "9d151eefd8590b89daa6ba6cb74af9275dd051026bb149a452fd84e5e57b5500");
	assert_hash!(stribog_256, b"\xd1\xe5 \xe2\xe5\xf2\xf0\xe8, \xd1\xf2\xf0\xe8\xe1\xee\xe6\xe8 \xe2\xed\xf3\xf6\xe8, \xe2\xe5\xfe\xf2\xfa \xf1 \xec\xee\xf0\xff \xf1\xf2\xf0\xe5\xeb\xe0\xec\xe8 \xed\xe0 \xf5\xf0\xe0\xe1\xf0\xfb\xff \xef\xeb\xfa\xea\xfb \xc8\xe3\xee\xf0\xe5\xe2\xfb", "9dd2fe4e90409e5da87f53976d7405b0c0cac628fc669a741d50063c557e8f50");
	assert_hash!(stribog_256, b"", "3f539a213e97c802cc229d474c6aa32a825a360b2a933a949fd925208d9ce1bb");
	assert_hash!(stribog_256, &[0u8; 64], "df1fda9ce83191390537358031db2ecaa6aa54cd0eda241dc107105e13636b95");
}