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
    let tmp = a01.wrapping_add(car2 as u128);
    (res[1], car2) = (tmp as u64, (tmp >> 64) as u64);
    let tmp = a02.wrapping_add(car2 as u128);
    (res[2], car2) = (tmp as u64, (tmp >> 64) as u64);
    let tmp = a03.wrapping_add(car2 as u128);
    (res[3], car2) = (tmp as u64, (tmp >> 64) as u64);
    // reduce
    let tmp = m0.wrapping_add(res[1] as u128);
    (res[0], car1) = (tmp as u64, (tmp >> 64) as u64);
    let tmp = m1.wrapping_add(res[2] as u128).wrapping_add(car1 as u128);
    (res[1], car1) = (tmp as u64, (tmp >> 64) as u64);
    let tmp = m2.wrapping_add(res[3] as u128).wrapping_add(car1 as u128);
    (res[2], car1) = (tmp as u64, (tmp >> 64) as u64);
    let tmp = m3.wrapping_add(car2 as u128).wrapping_add(car1 as u128);
    (res[3], car3) = (tmp as u64, (tmp >> 64) as u64);

    // step 1 (invr)
    let tmp = a10.wrapping_add(res[0] as u128);
    (res[0], car2) = (tmp as u64, (tmp >> 64) as u64);
    let m0 = (res[0] as u128).wrapping_mul(U64_I1[0] as u128);
    let m1 = (res[0] as u128).wrapping_mul(U64_I1[1] as u128);
    let m2 = (res[0] as u128).wrapping_mul(U64_I1[2] as u128);
    let m3 = (res[0] as u128).wrapping_mul(U64_I1[3] as u128);
    // accumulate
    let tmp = a11.wrapping_add(res[1] as u128).wrapping_add(car2 as u128);
    (res[1], car2) = (tmp as u64, (tmp >> 64) as u64);
    let tmp = a12.wrapping_add(res[2] as u128).wrapping_add(car2 as u128);
    (res[2], car2) = (tmp as u64, (tmp >> 64) as u64);
    let tmp = a13.wrapping_add(res[3] as u128).wrapping_add(car2 as u128);
    (res[3], car2) = (tmp as u64, (tmp >> 64) as u64);
    // reduce
    let tmp = m0.wrapping_add(res[1] as u128);
    (res[0], car1) = (tmp as u64, (tmp >> 64) as u64);
    let tmp = m1.wrapping_add(res[2] as u128).wrapping_add(car1 as u128);
    (res[1], car1) = (tmp as u64, (tmp >> 64) as u64);
    let tmp = m2.wrapping_add(res[3] as u128).wrapping_add(car1 as u128);
    (res[2], car1) = (tmp as u64, (tmp >> 64) as u64);
    let tmp = m3.wrapping_add(car2 as u128).wrapping_add(car1 as u128).wrapping_add(car3 as u128);
    (res[3], car3) = (tmp as u64, (tmp >> 64) as u64);

    // step 2 (invr)
    let tmp = a20.wrapping_add(res[0] as u128);
    (res[0], car2) = (tmp as u64, (tmp >> 64) as u64);
    let m0 = (res[0] as u128).wrapping_mul(U64_I1[0] as u128);
    let m1 = (res[0] as u128).wrapping_mul(U64_I1[1] as u128);
    let m2 = (res[0] as u128).wrapping_mul(U64_I1[2] as u128);
    let m3 = (res[0] as u128).wrapping_mul(U64_I1[3] as u128);
    // accumulate
    let tmp = a21.wrapping_add(res[1] as u128).wrapping_add(car2 as u128);
    (res[1], car2) = (tmp as u64, (tmp >> 64) as u64);
    let tmp = a22.wrapping_add(res[2] as u128).wrapping_add(car2 as u128);
    (res[2], car2) = (tmp as u64, (tmp >> 64) as u64);
    let tmp = a23.wrapping_add(res[3] as u128).wrapping_add(car2 as u128);
    (res[3], car2) = (tmp as u64, (tmp >> 64) as u64);
    // reduce
    let tmp = m0.wrapping_add(res[1] as u128);
    (res[0], car1) = (tmp as u64, (tmp >> 64) as u64);
    let tmp = m1.wrapping_add(res[2] as u128).wrapping_add(car1 as u128);
    (res[1], car1) = (tmp as u64, (tmp >> 64) as u64);
    let tmp = m2.wrapping_add(res[3] as u128).wrapping_add(car1 as u128);
    (res[2], car1) = (tmp as u64, (tmp >> 64) as u64);
    let tmp = m3.wrapping_add(car2 as u128).wrapping_add(car1 as u128).wrapping_add(car3 as u128);
    (res[3], car3) = (tmp as u64, (tmp >> 64) as u64);

    // step 3 (mu0)
    let tmp = a30.wrapping_add(res[0] as u128);
    (res[0], car2) = (tmp as u64, (tmp >> 64) as u64);
    let m = res[0].wrapping_mul(U64_MU0);
    let m0 = (m as u128).wrapping_mul(U64_P[0] as u128);
    let m1 = (m as u128).wrapping_mul(U64_P[1] as u128);
    let m2 = (m as u128).wrapping_mul(U64_P[2] as u128);
    let m3 = (m as u128).wrapping_mul(U64_P[3] as u128);
    // accumulate
    let tmp = a31.wrapping_add(res[1] as u128).wrapping_add(car2 as u128);
    (res[1], car2) = (tmp as u64, (tmp >> 64) as u64);
    let tmp = a32.wrapping_add(res[2] as u128).wrapping_add(car2 as u128);
    (res[2], car2) = (tmp as u64, (tmp >> 64) as u64);
    let tmp = a33.wrapping_add(res[3] as u128).wrapping_add(car2 as u128);
    (res[3], car2) = (tmp as u64, (tmp >> 64) as u64);
    // reduce
    let tmp = res[0] != 0;
    car1 = ((m0 >> 64) as u64).wrapping_add(tmp as u64);
    let tmp = m1.wrapping_add(res[1] as u128).wrapping_add(car1 as u128);
    (res[0], car1) = (tmp as u64, (tmp >> 64) as u64);
    let tmp = m2.wrapping_add(res[2] as u128).wrapping_add(car1 as u128);
    (res[1], car1) = (tmp as u64, (tmp >> 64) as u64);
    let tmp = m3.wrapping_add(res[3] as u128).wrapping_add(car1 as u128);
    (res[2], car1) = (tmp as u64, (tmp >> 64) as u64);
    res[3] = car1.wrapping_add(car2).wrapping_add(car3);

    res
}
