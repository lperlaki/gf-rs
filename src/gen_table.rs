pub const fn gen_tables_u8(genpoly: usize) -> ([usize; 256], [u8; 1023]) {
    let mut logtable: [usize; 256] = [0; 256];
    let mut alogtable: [u8; 1023] = [0; 1023];

    logtable[0] = 511;
    alogtable[0] = 1;

    let mut i = 1;
    while i < 255 {
        let mut next = (alogtable[i - 1] as usize) * 2;
        if next >= 256 {
            next ^= genpoly;
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

    (logtable, alogtable)
}
