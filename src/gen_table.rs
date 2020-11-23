pub const fn gen_tables_u8() -> ([usize; 256], [u8; 1025]) {
    let mut logtable: [usize; 256] = [0; 256];
    let mut alogtable: [u8; 1025] = [0; 1025];

    let genpoly: usize = 0x11D;

    logtable[0] = 512;
    alogtable[0] = 1;

    let mut i = 1;
    while i < 255 {
        let mut next = (alogtable[i - 1] as usize) * 2;
        if next >= 256 {
            next = next ^ genpoly;
        }

        alogtable[i] = next as u8;
        logtable[alogtable[i] as usize] = i;

        i += 1;
    }

    alogtable[255] = alogtable[0];
    logtable[alogtable[255] as usize] = 255;
    let mut i = 256;
    while i < 510 {
        alogtable[i] = alogtable[i % 255];

        i += 1;
    }

    alogtable[510] = 1;

    let mut i = 511;

    while i < 1020 {
        alogtable[i] = 0;

        i += 1;
    }

    (logtable, alogtable)
}

// const fn gen_tables2() -> ([u8; 256], [u8; 1025]) {
//     fn ffmul(mut a: u8, mut b: u8) -> u8 {
//         let r = 0;

//         while a != 0 {
//             if (a & 1) != 0 {
//                 r ^= b;
//             }
//             let t = b & 0x80;
//             b <<= 1;
//             if t != 0 {
//                 b ^= 0x1b;
//             }
//             a = (a & 0xff) >> 1;
//         }
//         r
//     }

//     let mut logtable: [u8; 256] = [0; 256];
//     let mut alogtable: [u8; 256] = [0; 256];

//     let genpoly: u8 = 0x03;

//     alogtable[0] = 0x01;

//     let mut i = 1;
//     while i < 255 {
//         alogtable[i] = ffmul(alogtable[i - 1], genpoly);
//         i += 1;
//     }

//     let mut i = 1;
//     while i < 255 {
//         logtable[alogtable[i] & 0xff] = i;
//     }

//     (logtable, alogtable)
// }
