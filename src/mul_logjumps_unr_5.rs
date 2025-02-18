// this version uses i1/i2 with daisy-chained carries and 64b additions
use crate::constants::*;


#[inline]
pub fn mul_logjumps_unr_5(a: [u64; 4], b: [u64; 4]) -> [u64; 4] {
    // ab mults
    let (c00hi, c00lo) = mult(a[0], b[0]);
    let (c01hi, c01lo) = mult(a[0], b[1]);
    let (c02hi, c02lo) = mult(a[0], b[2]);
    let (c03hi, c03lo) = mult(a[0], b[3]);
    let (c10hi, c10lo) = mult(a[1], b[0]);
    let (c11hi, c11lo) = mult(a[1], b[1]);
    let (c12hi, c12lo) = mult(a[1], b[2]);
    let (c13hi, c13lo) = mult(a[1], b[3]);
    let (c20hi, c20lo) = mult(a[2], b[0]);
    let (c21hi, c21lo) = mult(a[2], b[1]);
    let (c22hi, c22lo) = mult(a[2], b[2]);
    let (c23hi, c23lo) = mult(a[2], b[3]);
    let (c30hi, c30lo) = mult(a[3], b[0]);
    let (c31hi, c31lo) = mult(a[3], b[1]);
    let (c32hi, c32lo) = mult(a[3], b[2]);
    let (c33hi, c33lo) = mult(a[3], b[3]);

    let mut c: bool;
    let mut r00 = c00lo;
    let mut r01 = 0u64;
    let mut r10 = 0u64;
    let mut r11 = 0u64;
    let mut r20 = 0u64;
    let mut r21 = 0u64;
    let mut r30 = 0u64;
    let mut r31 = 0u64;

    let (ir000hi, ir000lo) = mult(r00, U64_I2[0]);
    let (ir001hi, ir001lo) = mult(r00, U64_I2[1]);
    let (ir002hi, ir002lo) = mult(r00, U64_I2[2]);
    let (ir003hi, ir003lo) = mult(r00, U64_I2[3]);

    let (r01, c) = r01.carrying_add(c00hi, false); // carry is rare
    let (r01, c) = r01.carrying_add(c01lo, false);
    let (r10, c) = r10.carrying_add(c11lo, c); // carry is rare
    let (r01, c) = r01.carrying_add(c10lo, false);
    let (r10, c) = r10.carrying_add(c01hi, c);
    let (r11, c) = r11.carrying_add(c11hi, c); // carry is rare

    let (ir010hi, ir010lo) = mult(r01, U64_I2[0]);
    let (ir011hi, ir011lo) = mult(r01, U64_I2[1]);
    let (ir012hi, ir012lo) = mult(r01, U64_I2[2]);
    let (ir013hi, ir013lo) = mult(r01, U64_I2[3]);

    let (r10, c) = r10.carrying_add(c10hi  , false);
    let (r11, c) = r11.carrying_add(c12lo  , c);
    let (r20, c) = r20.carrying_add(c12hi  , c);
    let (r21, c) = r21.carrying_add(c13hi  , c); // carry is rare
    let (r10, c) = r10.carrying_add(c02lo  , false);
    let (r11, c) = r11.carrying_add(c21lo  , c);
    let (r20, c) = r20.carrying_add(c21hi  , c);
    let (r21, c) = r21.carrying_add(c31hi  , c);
    let (r30, c) = r30.carrying_add(c23hi  , c);
    let (r10, c) = r10.carrying_add(c20lo  , false);
    let (r11, c) = r11.carrying_add(c02hi  , c);
    let (r20, c) = r20.carrying_add(c13lo  , c);
    let (r21, c) = r21.carrying_add(c23lo  , c);
    let (r30, c) = r30.carrying_add(c32hi  , c);
    let (r31, c) = r31.carrying_add(c33hi  , c);
    let (r10, c) = r10.carrying_add(ir000lo, false);
    let (r11, c) = r11.carrying_add(c20hi  , c);
    let (r20, c) = r20.carrying_add(c31lo  , c);
    let (r21, c) = r21.carrying_add(c32lo  , c);
    let (r30, c) = r30.carrying_add(c33lo  , c);
    let (r31, c) = r31.carrying_add(ir013hi, c);

    let (ir100hi, ir100lo) = mult(r10, U64_I1[0]);
    let (ir101hi, ir101lo) = mult(r10, U64_I1[1]);
    let (ir102hi, ir102lo) = mult(r10, U64_I1[2]);
    let (ir103hi, ir103lo) = mult(r10, U64_I1[3]);

    let (r11, c) = r11.carrying_add(c03lo  , false);
    let (r20, c) = r20.carrying_add(c03hi  , c);
    let (r21, c) = r21.carrying_add(c22hi  , c);
    let (r30, c) = r30.carrying_add(ir003hi, c);
    let (r31, c) = r31.carrying_add(ir103hi, c);
    let (r11, c) = r11.carrying_add(c30lo  , false);
    let (r20, c) = r20.carrying_add(c30hi  , c);
    let (r21, c) = r21.carrying_add(ir002hi, c);
    let (r30, c) = r30.carrying_add(ir012hi, c);
    let (r31, _) = r31.carrying_add(0u64   , c);
    let (r11, c) = r11.carrying_add(ir000hi, false);
    let (r20, c) = r20.carrying_add(c22lo  , c);
    let (r21, c) = r21.carrying_add(ir003lo, c);
    let (r30, c) = r30.carrying_add(ir013lo, c);
    let (r31, _) = r31.carrying_add(0u64   , c);
    let (r11, c) = r11.carrying_add(ir001lo, false);
    let (r20, c) = r20.carrying_add(ir002lo, c);
    let (r21, c) = r21.carrying_add(ir012lo, c);
    let (r30, c) = r30.carrying_add(ir102hi, c);
    let (r31, _) = r31.carrying_add(0u64   , c);
    let (r11, c) = r11.carrying_add(ir010lo, false);
    let (r20, c) = r20.carrying_add(ir001hi, c);
    let (r21, c) = r21.carrying_add(ir011hi, c);
    let (r30, c) = r30.carrying_add(ir103lo, c);
    let (r31, _) = r31.carrying_add(0u64   , c);
    let (r11, c) = r11.carrying_add(ir100lo, false);
    let (r20, c) = r20.carrying_add(ir010hi, c);
    let (r21, c) = r21.carrying_add(ir102lo, c);
    let (r30, _) = r30.carrying_add(0u64   , c);

    let m = U64_MU0.wrapping_mul(r11);
    let (m0hi, m0lo) = mult(m, U64_P[0]);
    let (m1hi, m1lo) = mult(m, U64_P[1]);
    let (m2hi, m2lo) = mult(m, U64_P[2]);
    let (m3hi, m3lo) = mult(m, U64_P[3]);

    let (r11, c) = r11.carrying_add(m0lo   , false);
    let (r20, c) = r20.carrying_add(ir011lo, c);
    let (r21, c) = r21.carrying_add(ir101hi, c);
    let (r30, c) = r30.carrying_add(m2hi   , c);
    let (r31, c) = r31.carrying_add(m3hi   , c);
    let (r20, c) = r20.carrying_add(ir100hi, false);
    let (r21, c) = r21.carrying_add(m2lo   , c);
    let (r30, c) = r30.carrying_add(m3lo   , c);
    let (r31, _) = r31.carrying_add(0u64   , c);
    let (r20, c) = r20.carrying_add(ir101lo, false);
    let (r21, c) = r21.carrying_add(m1hi   , c);
    let (r30, _) = r30.carrying_add(0u64   , c);
    let (r20, c) = r20.carrying_add(m0hi   , false);
    let (r21, _) = r21.carrying_add(0u64   , c);
    let (r20, c) = r20.carrying_add(m1lo   , false);
    let (r21, _) = r21.carrying_add(0u64   , c);

    // return
    [r20, r21, r30, r31]
}

#[inline]
const fn mult(lhs: u64, rhs: u64) -> (u64, u64) {
    let res = (lhs as u128).wrapping_mul(rhs as u128);
    ((res >> 64) as u64, res as u64)
}
