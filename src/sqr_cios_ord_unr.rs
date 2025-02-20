// special ordering version
use crate::constants::*;


#[inline]
pub fn sqr_cios_ord_unr(a: [u64; 4]) -> [u64; 4] {
    let p = U64_P;
    let mu0 = U64_MU0;
    let invR = U128_INVR;
    let mut t = [0u128; 3];
    // dbg!(a);

    // pre steps
    let a00div2 = (a[0] as u128).wrapping_mul(a[0] as u128) >> 1;
    let a0lsb = a[0] & 1 != 0;
    let a01 = (a[0] as u128).wrapping_mul(a[1] as u128);
    let a02 = (a[0] as u128).wrapping_mul(a[2] as u128);
    let a03 = (a[0] as u128).wrapping_mul(a[3] as u128);
    let (a03_hi, a03_lo) = ((a03 >> 64) as u64, a03 as u64);
    let a11div2 = (a[1] as u128).wrapping_mul(a[1] as u128) >> 1;
    let a1lsb = (a[1] & 1) << 63;
    let a13 = (a[1] as u128).wrapping_mul(a[3] as u128);
    let (a13_hi, a13_lo) = ((a13 >> 64) as u64, a13 as u64);
    let a12 = (a[1] as u128).wrapping_mul(a[2] as u128);
    let (a12_hi, a12_lo) = ((a12 >> 64) as u64, a12 as u64);
    let a23 = (a[2] as u128).wrapping_mul(a[3] as u128);
    let (a23_hi, a23_lo) = ((a23 >> 64) as u64, a23 as u64);
    let a22 = (a[2] as u128).wrapping_mul(a[2] as u128);
    let a33 = (a[3] as u128).wrapping_mul(a[3] as u128);
    
    // step 0, 0
    let (hi, lo) = ((a00div2 >> 64) as u64, a00div2 as u64);
    let m = lo.wrapping_mul(mu0); // lsb mult
    let m0 = ((m as u128).wrapping_mul(p[0] as u128) >> 64) as u64; // msb mult
    let (c_, _) = m0.carrying_add(hi, true);
    // step 0, 1
    let c00: bool;
    (t[0], c00) = (m as u128).wrapping_mul(p[1] as u128).wrapping_add(c_ as u128).overflowing_add(a01);
    // step 0, 3
    (t[2], _) = (m as u128).wrapping_mul(p[3] as u128).carrying_add(a03_lo as u128, c00);
    // step 0, 2
    let c1: bool;
    (t[1], c1) = (m as u128).wrapping_mul(p[2] as u128).overflowing_add(a02);
    // dbg!(t);

    // step 1, 0
    let tmp = t[0].wrapping_add(a1lsb as u128);
    let (hi, lo) = ((tmp >> 64) as u64, tmp as u64);
    let m = lo.wrapping_mul(mu0); // lsb mult
    let m0 = ((m as u128).wrapping_mul(p[0] as u128) >> 64) as u64; // msb mult
    let (c_, _) = (m0 as u128).carrying_add(hi as u128, true); // carry add
    // step 1, 1
    let c01: bool;
    (t[0], c01) = (m as u128).wrapping_mul(p[1] as u128).wrapping_add(c_).overflowing_add(t[1]);
    // step 1, 2
    t[1] = (m as u128).wrapping_mul(p[2] as u128).wrapping_add(t[2]);
    // step 1, 3
    t[2] = (m as u128).wrapping_mul(p[3] as u128);//.wrapping_add(c01 as u128); // carry add TODO
    // dbg!(t);

    // step 2, 0
    let tmp: u128;
    let c0: bool;
    (tmp, c0) = t[0].overflowing_add(a11div2);
    let (hi, lo) = ((tmp >> 64) | ((c0 as u128) << 64), tmp as u64);
    let m = lo.wrapping_mul(mu0); // lsb mult
    let m0 = ((m as u128).wrapping_mul(p[0] as u128) >> 64) as u64; // msb mult
    let (c_, _) = (m0 as u128).carrying_add(hi, true);
    // step 2, 1
    let c02: bool;
    (t[0], c02) = (m as u128).wrapping_mul(p[1] as u128).wrapping_add(a12_lo as u128).wrapping_add(c_).overflowing_add(t[1]);
    // step 2, 2
    (t[1], _) = (m as u128).wrapping_mul(p[2] as u128).carrying_add(t[2], c1);
    // step 2, 3
    t[2] = (m as u128).wrapping_mul(p[3] as u128);//.wrapping_add(c02 as u128); // carry add TODO
    // dbg!(t);

    // step 3, 0
    let (hi, lo) = ((t[0] >> 64) as u64, t[0] as u64);
    let m = lo.wrapping_mul(mu0); // lsb mult
    let m0 = ((m as u128).wrapping_mul(p[0] as u128) >> 64) as u64; // msb mult
    let (c_, _) = (m0 as u128).carrying_add(hi as u128, true); // carry add
    // step 3, 1
    let c03: bool;
    (t[0], c03) = (m as u128).wrapping_mul(p[1] as u128).wrapping_add(c_).wrapping_add(a13_lo as u128).wrapping_add(a12_hi as u128).wrapping_add(a03_hi as u128).carrying_add( t[1], c01);
    // step 3, 2
    (t[1], _) = (m as u128).wrapping_mul(p[2] as u128).wrapping_add(a23_lo as u128).wrapping_add(a13_hi as u128).carrying_add(t[2], c02);
    // step 3, 3
    (t[2], _) = (m as u128).wrapping_mul(p[3] as u128).carrying_add(a23_hi as u128, c03);
    // dbg!(t);

    // final addition
    let mut res = if a0lsb {invR} else {[0u128; 2]};
    let c0: bool;
    let c1: bool;
    let c2: bool;
    (res[0], c0) = res[0].overflowing_add(a22);
    (res[0], c1) = res[0].overflowing_add(t[0] << 1);
    (res[0], c2) = res[0].overflowing_add(t[1] << 65);
    let c4 = (c0 as u8) + (c1 as u8) + (c2 as u8);
    res[1] = res[1].wrapping_add(a33);
    res[1] = res[1].wrapping_add((t[2] << 1) | (t[0] >> 127));
    res[1] = res[1].wrapping_add(t[1] >> 63);
    res[1] = res[1].wrapping_add(c4 as u128);
    // dbg!([res[0] as u64, (res[0] >> 64) as u64, res[1] as u64, (res[1] >> 64) as u64]);
    [res[0] as u64, (res[0] >> 64) as u64, res[1] as u64, (res[1] >> 64) as u64]
}
