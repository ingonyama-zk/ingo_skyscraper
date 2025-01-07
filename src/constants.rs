pub const U64_P:  [u64; 4] = [0x43e1f593f0000001, 0x2833e84879b97091, 0xb85045b68181585d, 0x30644e72e131a029];
pub const U64_2P: [u64; 4] = [0x87c3eb27e0000002, 0x5067d090f372e122, 0x70a08b6d0302b0ba, 0x60c89ce5c2634053];
pub const U64_3P: [u64; 4] = [0xcba5e0bbd0000003, 0x789bb8d96d2c51b3, 0x28f0d12384840917, 0x912ceb58a394e07d];
pub const U64_4P: [u64; 4] = [0x0f87d64fc0000004, 0xa0cfa121e6e5c245, 0xe14116da06056174, 0xc19139cb84c680a6];
pub const U64_5P: [u64; 4] = [0x5369cbe3b0000005, 0xc903896a609f32d6, 0x99915c908786b9d1, 0xf1f5883e65f820d0];

pub const U64_MU0: u64 = 0xc2e1f593efffffff;
pub const U128_INVR: [u128; 2] = [0x090ef5a9e111ec87dc5ba0056db1194e, 0x15ebf95182c5551cc8260de4aeb85d5d];

pub const RC: [[u64; 4]; 8] = [[0x903c4324270bd744, 0x873125f708a7d269, 0x081dd27906c83855, 0x276b1823ea6d7667],
                               [0x7ac8edbb4b378d71, 0xe29d79f3d99e2cb7, 0x751417914c1a5a18, 0x0cf02bd758a484a6],
                               [0xfa7adc6769e5bc36, 0x1c3f8e297cca387d, 0x0eb7730d63481db0, 0x25b0e03f18ede544],
                               [0x57847e652f03cfb7, 0x33440b9668873404, 0x955a32e849af80bc, 0x002882fcbe14ae70],
                               [0x979231396257d4d7, 0x29989c3e1b37d3c1, 0x12ef02b47f1277ba, 0x039ad8571e2b7a9c],
                               [0xb5b48465abbb7887, 0xa72a6bc5e6ba2d2b, 0x4cd48043712f7b29, 0x1142d5410fc1fc1a],
                               [0x7ab2c156059075d3, 0x17cb3594047999b2, 0x44f2c93598f289f7, 0x1d78439f69bc0bec],
                               [0x05d7a965138b8edb, 0x36ef35a3d55c48b1, 0x8ddfb8a1ac6f1628, 0x258588a508f4ff82]];

pub const _1P_MINUS_RC:[[u64; 4]; 8] = [[0xb3a5b26fc8f428bd, 0xa102c25171119e27, 0xb032733d7ab92007, 0x08f9364ef6c429c2], 
                                        [0xc91907d8a4c87290, 0x45966e54a01b43d9, 0x433c2e253566fe44, 0x2374229b888d1b83], 
                                        [0x4967192c861a43cb, 0x0bf45a1efcef3813, 0xa998d2a91e393aad, 0x0ab36e33c843bae5], 
                                        [0xec5d772ec0fc304a, 0xf4efdcb211323c8c, 0x22f612ce37d1d7a0, 0x303bcb76231cf1b9], 
                                        [0xac4fc45a8da82b2a, 0xfe9b4c0a5e819ccf, 0xa5614302026ee0a2, 0x2cc9761bc306258d], 
                                        [0x8e2d712e4444877a, 0x81097c8292ff4365, 0x6b7bc5731051dd33, 0x1f217931d16fa40f], 
                                        [0xc92f343dea6f8a2e, 0x1068b2b4753fd6de, 0x735d7c80e88ece66, 0x12ec0ad37775943d], 
                                        [0x3e0a4c2edc747126, 0xf144b2a4a45d27e0, 0x2a708d14d5124234, 0x0adec5cdd83ca0a7]];

pub const _2P_MINUS_RC:[[u64; 4]; 8] = [[0xf787a803b8f428be, 0xc936aa99eacb0eb8, 0x6882b8f3fc3a7864, 0x395d84c1d7f5c9ec], 
                                        [0x0cfafd6c94c87291, 0x6dca569d19d4b46b, 0xfb8c73dbb6e856a1, 0x53d8710e69bebbac], 
                                        [0x8d490ec0761a43cc, 0x3428426776a8a8a4, 0x61e9185f9fba930a, 0x3b17bca6a9755b0f], 
                                        [0x303f6cc2b0fc304b, 0x1d23c4fa8aebad1e, 0xdb465884b9532ffe, 0x60a019e9044e91e2], 
                                        [0xf031b9ee7da82b2b, 0x26cf3452d83b0d60, 0x5db188b883f03900, 0x5d2dc48ea437c5b7], 
                                        [0xd20f66c23444877b, 0xa93d64cb0cb8b3f6, 0x23cc0b2991d33590, 0x4f85c7a4b2a14439], 
                                        [0x0d1129d1da6f8a2f, 0x389c9afceef94770, 0x2badc2376a1026c3, 0x4350594658a73467], 
                                        [0x81ec41c2cc747127, 0x19789aed1e169871, 0xe2c0d2cb56939a92, 0x3b431440b96e40d0]];


pub const _3P_MINUS_RC: [[u64; 4]; 8] = [[0x3b699d97a8f428bf, 0xf16a92e264847f4a, 0x20d2feaa7dbbd0c1, 0x69c1d334b9276a16], 
                                         [0x50dcf30084c87292, 0x95fe3ee5938e24fc, 0xb3dcb9923869aefe, 0x843cbf814af05bd6], 
                                         [0xd12b0454661a43cd, 0x5c5c2aaff0621935, 0x1a395e16213beb67, 0x6b7c0b198aa6fb39], 
                                         [0x74216256a0fc304c, 0x4557ad4304a51daf, 0x93969e3b3ad4885b, 0x9104685be580320c], 
                                         [0x3413af826da82b2c, 0x4f031c9b51f47df2, 0x1601ce6f0571915d, 0x8d921301856965e1], 
                                         [0x15f15c562444877c, 0xd1714d1386722488, 0xdc1c50e013548ded, 0x7fea161793d2e462], 
                                         [0x50f31f65ca6f8a30, 0x60d0834568b2b801, 0xe3fe07edeb917f20, 0x73b4a7b939d8d490], 
                                         [0xc5ce3756bc747128, 0x41ac833597d00902, 0x9b111881d814f2ef, 0x6ba762b39a9fe0fa]];

pub const _4P_MINUS_RC:[[u64; 4]; 8] = [[0x7f4b932b98f428c0, 0x199e7b2ade3defdb, 0xd9234460ff3d291f, 0x9a2621a79a590a3f], 
                                        [0x94bee89474c87293, 0xbe32272e0d47958d, 0x6c2cff48b9eb075b, 0xb4a10df42c21fc00], 
                                        [0x150cf9e8561a43ce, 0x849012f86a1b89c7, 0xd289a3cca2bd43c4, 0x9be0598c6bd89b62], 
                                        [0xb80357ea90fc304d, 0x6d8b958b7e5e8e40, 0x4be6e3f1bc55e0b8, 0xc168b6cec6b1d236], 
                                        [0x77f5a5165da82b2d, 0x773704e3cbadee83, 0xce52142586f2e9ba, 0xbdf66174669b060a], 
                                        [0x59d351ea1444877d, 0xf9a5355c002b9519, 0x946c969694d5e64a, 0xb04e648a7504848c], 
                                        [0x94d514f9ba6f8a31, 0x89046b8de26c2892, 0x9c4e4da46d12d77d, 0xa418f62c1b0a74ba], 
                                        [0x09b02ceaac747129, 0x69e06b7e11897994, 0x53615e3859964b4c, 0x9c0bb1267bd18124]];
