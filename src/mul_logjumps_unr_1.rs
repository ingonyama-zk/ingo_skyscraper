// unrolled logjumps using i1/i2 with 3 temp registers car1, car2, car3
use crate::constants::*;


#[inline]
pub fn mul_logjumps_unr_1(a: [u64; 4], b: [u64; 4]) -> [u64; 4] {
    let mut res = [0u64; 4];
    let mut car1: u64;
    let mut car2: u64;
    let mut car3: u64;

    let a00 = (a[0] as u128).wrapping_mul(b[0] as u128);
    let a01 = (a[0] as u128).wrapping_mul(b[1] as u128);
    let a02 = (a[0] as u128).wrapping_mul(b[2] as u128);
    let a03 = (a[0] as u128).wrapping_mul(b[3] as u128);
    let a10 = (a[1] as u128).wrapping_mul(b[0] as u128);
    let a11 = (a[1] as u128).wrapping_mul(b[1] as u128);
    let a12 = (a[1] as u128).wrapping_mul(b[2] as u128);
    let a13 = (a[1] as u128).wrapping_mul(b[3] as u128);
    let a20 = (a[2] as u128).wrapping_mul(b[0] as u128);
    let a21 = (a[2] as u128).wrapping_mul(b[1] as u128);
    let a22 = (a[2] as u128).wrapping_mul(b[2] as u128);
    let a23 = (a[2] as u128).wrapping_mul(b[3] as u128);
    let a30 = (a[3] as u128).wrapping_mul(b[0] as u128);
    let a31 = (a[3] as u128).wrapping_mul(b[1] as u128);
    let a32 = (a[3] as u128).wrapping_mul(b[2] as u128);
    let a33 = (a[3] as u128).wrapping_mul(b[3] as u128);

    // step 0 (invr)
    (res[0], car2) = (a00 as u64, (a00 >> 64) as u64);
    let m0 = (res[0] as u128).wrapping_mul(U64_I1[0] as u128);
    let m1 = (res[0] as u128).wrapping_mul(U64_I1[1] as u128);
    let m2 = (res[0] as u128).wrapping_mul(U64_I1[2] as u128);
    let m3 = (res[0] as u128).wrapping_mul(U64_I1[3] as u128);
    // accumulate
    (res[1], car2) = wadd_1(a01, car2);
    (res[2], car2) = wadd_1(a02, car2);
    (res[3], car2) = wadd_1(a03, car2);
    // reduce
    (res[0], car1) = wadd_1(m0, res[1]);
    (res[1], car1) = wadd_2(m1, res[2], car1);
    (res[2], car1) = wadd_2(m2, res[3], car1);
    (res[3], car3) = wadd_2(m3, car2, car1);

    // step 1 (invr)
    (res[0], car2) = wadd_1(a10, res[0]);
    let m0 = (res[0] as u128).wrapping_mul(U64_I1[0] as u128);
    let m1 = (res[0] as u128).wrapping_mul(U64_I1[1] as u128);
    let m2 = (res[0] as u128).wrapping_mul(U64_I1[2] as u128);
    let m3 = (res[0] as u128).wrapping_mul(U64_I1[3] as u128);
    // accumulate
    (res[1], car2) = wadd_2(a11, res[1], car2);
    (res[2], car2) = wadd_2(a12, res[2], car2);
    (res[3], car2) = wadd_2(a13, res[3], car2);
    // reduce
    (res[0], car1) = wadd_1(m0, res[1]);
    (res[1], car1) = wadd_2(m1, res[2], car1);
    (res[2], car1) = wadd_2(m2, res[3], car1);
    (res[3], car3) = wadd_3(m3, car3, car2, car1);

    // step 2 (invr)
    (res[0], car2) = wadd_1(a20, res[0]);
    let m0 = (res[0] as u128).wrapping_mul(U64_I1[0] as u128);
    let m1 = (res[0] as u128).wrapping_mul(U64_I1[1] as u128);
    let m2 = (res[0] as u128).wrapping_mul(U64_I1[2] as u128);
    let m3 = (res[0] as u128).wrapping_mul(U64_I1[3] as u128);
    // accumulate
    (res[1], car2) = wadd_2(a21, res[1], car2);
    (res[2], car2) = wadd_2(a22, res[2], car2);
    (res[3], car2) = wadd_2(a23, res[3], car2);
    // reduce
    (res[0], car1) = wadd_1(m0, res[1]);
    (res[1], car1) = wadd_2(m1, res[2], car1);
    (res[2], car1) = wadd_2(m2, res[3], car1);
    (res[3], car3) = wadd_3(m3, car3, car2, car1);

    // step 3 (mu0)
    (res[0], car2) = wadd_1(a30, res[0]);
    let m = res[0].wrapping_mul(U64_MU0);
    let m0 = (m as u128).wrapping_mul(U64_P[0] as u128);
    let m1 = (m as u128).wrapping_mul(U64_P[1] as u128);
    let m2 = (m as u128).wrapping_mul(U64_P[2] as u128);
    let m3 = (m as u128).wrapping_mul(U64_P[3] as u128);
    // accumulate
    (res[1], car2) = wadd_2(a31, res[1], car2);
    (res[2], car2) = wadd_2(a32, res[2], car2);
    (res[3], car2) = wadd_2(a33, res[3], car2);
    // reduce
    car1 = ((m0 >> 64) as u64).wrapping_add((res[0] != 0) as u64);
    (res[0], car1) = wadd_2(m1, res[1], car1);
    (res[1], car1) = wadd_2(m2, res[2], car1);
    (res[2], car1) = wadd_2(m3, res[3], car1);
    res[3] = car1.wrapping_add(car2).wrapping_add(car3);

    res
}

#[inline]
fn wadd_1(w: u128, a: u64) -> (u64, u64) {
    let tmp = w.wrapping_add(a as u128);
    (tmp as u64, (tmp >> 64) as u64)
}

#[inline]
fn wadd_2(w: u128, a: u64, b: u64) -> (u64, u64) {
    let tmp = w.wrapping_add(a as u128).wrapping_add(b as u128);
    (tmp as u64, (tmp >> 64) as u64)
}

#[inline]
fn wadd_3(w: u128, a: u64, b: u64, c: u64) -> (u64, u64) {
    let tmp = w.wrapping_add(a as u128).wrapping_add(b as u128).wrapping_add(c as u128);
    (tmp as u64, (tmp >> 64) as u64)
}
