// unrolled ver1 acar cios with 2 temp registers car1, car2
use crate::constants::*;


#[inline]
pub fn sqr_cios_opt_unr_1(a: [u64; 4]) -> [u64; 4] {
    let p = U64_P;
    let mu0 = U64_MU0;
    let mut res = [0u64; 4];
    let mut car1: u64;
    let mut car2: u64;

    let a00 = (a[0] as u128).wrapping_mul(a[0] as u128);
    let a01 = (a[0] as u128).wrapping_mul(a[1] as u128);
    let a02 = (a[0] as u128).wrapping_mul(a[2] as u128);
    let a03 = (a[0] as u128).wrapping_mul(a[3] as u128);
    let a11 = (a[1] as u128).wrapping_mul(a[1] as u128);
    let a12 = (a[1] as u128).wrapping_mul(a[2] as u128);
    let a13 = (a[1] as u128).wrapping_mul(a[3] as u128);
    let a22 = (a[2] as u128).wrapping_mul(a[2] as u128);
    let a23 = (a[2] as u128).wrapping_mul(a[3] as u128);
    let a33 = (a[3] as u128).wrapping_mul(a[3] as u128);
    let a10 = a01;
    let a20 = a02;
    let a21 = a12;
    let a30 = a03;
    let a31 = a13;
    let a32 = a23;

    // step 0, 0
    let tmp = (a00 as u64).wrapping_add(res[0]);
    (res[0], car2) = (tmp, (a00 >> 64) as u64);
    let m = res[0].wrapping_mul(mu0);
    let m0 = (m as u128).wrapping_mul(p[0] as u128);
    let m1 = (m as u128).wrapping_mul(p[1] as u128);
    let m2 = (m as u128).wrapping_mul(p[2] as u128);
    let m3 = (m as u128).wrapping_mul(p[3] as u128);
    // carry_1 = ((m0  >> 64) as u64).wrapping_add(1); // doesn't work when digits are 0 or powers of 2 etc.
    let (_, tmp) = res[0].overflowing_add(m0 as u64);
    car1 = ((m0 >>64) as u64).wrapping_add(tmp as u64);
    // step 0, 1
    let tmp = a01.wrapping_add((res[1]).wrapping_add(car2) as u128);
    (res[1], car2) = (tmp as u64, (tmp >> 64) as u64);
    let tmp = m1.wrapping_add(res[1] as u128).wrapping_add(car1 as u128);
    (res[0], car1) = (tmp as u64, (tmp >> 64) as u64);
    // step 0, 2
    let tmp = a02.wrapping_add((res[2]).wrapping_add(car2) as u128);
    (res[2], car2) = (tmp as u64, (tmp >> 64) as u64);
    let tmp = m2.wrapping_add(res[2] as u128).wrapping_add(car1 as u128);
    (res[1], car1) = (tmp as u64, (tmp >> 64) as u64);
    // step 0, 3
    let tmp = a03.wrapping_add((res[3]).wrapping_add(car2) as u128);
    (res[3], car2) = (tmp as u64, (tmp >> 64) as u64);
    let tmp = m3.wrapping_add(res[3] as u128).wrapping_add(car1 as u128);
    (res[2], car1) = (tmp as u64, (tmp >> 64) as u64);
    // step 0, 4
    res[3] = car1.wrapping_add(car2);

    // step 1, 0
    let tmp = a10.wrapping_add(res[0] as u128);
    (res[0], car2) = (tmp as u64, (tmp >> 64) as u64);
    let m = res[0].wrapping_mul(mu0);
    let m0 = (m as u128).wrapping_mul(p[0] as u128);
    let m1 = (m as u128).wrapping_mul(p[1] as u128);
    let m2 = (m as u128).wrapping_mul(p[2] as u128);
    let m3 = (m as u128).wrapping_mul(p[3] as u128);
    // carry_1 = ((m0  >> 64) as u64).wrapping_add(1); // doesn't work when digits are 0 or powers of 2 etc.
    let (_, tmp) = res[0].overflowing_add(m0 as u64);
    car1 = ((m0  >> 64) as u64).wrapping_add(tmp as u64);
    // step 1, 1
    let tmp = a11.wrapping_add(res[1] as u128).wrapping_add(car2 as u128);
    (res[1], car2) = (tmp as u64, (tmp >> 64) as u64);
    let tmp = m1.wrapping_add(res[1] as u128).wrapping_add(car1 as u128);
    (res[0], car1) = (tmp as u64, (tmp >> 64) as u64);
    // step 1, 2
    let tmp = a12.wrapping_add(res[2] as u128).wrapping_add(car2 as u128);
    (res[2], car2) = (tmp as u64, (tmp >> 64) as u64);
    let tmp = m2.wrapping_add(res[2] as u128).wrapping_add(car1 as u128);
    (res[1], car1) = (tmp as u64, (tmp >> 64) as u64);
    // step 1, 3
    let tmp = a13.wrapping_add(res[3] as u128).wrapping_add(car2 as u128);
    (res[3], car2) = (tmp as u64, (tmp >> 64) as u64);
    let tmp = m3.wrapping_add(res[3] as u128).wrapping_add(car1 as u128);
    (res[2], car1) = (tmp as u64, (tmp >> 64) as u64);
    // step 1, 4
    res[3] = car1.wrapping_add(car2);

    // step 2, 0
    let tmp = a20.wrapping_add(res[0] as u128);
    (res[0], car2) = (tmp as u64, (tmp >> 64) as u64);
    let m = res[0].wrapping_mul(mu0);
    let m0 = (m as u128).wrapping_mul(p[0] as u128);
    let m1 = (m as u128).wrapping_mul(p[1] as u128);
    let m2 = (m as u128).wrapping_mul(p[2] as u128);
    let m3 = (m as u128).wrapping_mul(p[3] as u128);
    // carry_1 = ((m0  >> 64) as u64).wrapping_add(1); // doesn't work when digits are 0 or powers of 2 etc.
    let (_, tmp) = res[0].overflowing_add(m0 as u64);
    car1 = ((m0  >> 64) as u64).wrapping_add(tmp as u64);
    // step 2, 1
    let tmp = a21.wrapping_add(res[1] as u128).wrapping_add(car2 as u128);
    (res[1], car2) = (tmp as u64, (tmp >> 64) as u64);
    let tmp = m1.wrapping_add(res[1] as u128).wrapping_add(car1 as u128);
    (res[0], car1) = (tmp as u64, (tmp >> 64) as u64);
    // step 2, 2
    let tmp = a22.wrapping_add(res[2] as u128).wrapping_add(car2 as u128);
    (res[2], car2) = (tmp as u64, (tmp >> 64) as u64);
    let tmp = m2.wrapping_add(res[2] as u128).wrapping_add(car1 as u128);
    (res[1], car1) = (tmp as u64, (tmp >> 64) as u64);
    // step 2, 3
    let tmp = a23.wrapping_add(res[3] as u128).wrapping_add(car2 as u128);
    (res[3], car2) = (tmp as u64, (tmp >> 64) as u64);
    let tmp = m3.wrapping_add(res[3] as u128).wrapping_add(car1 as u128);
    (res[2], car1) = (tmp as u64, (tmp >> 64) as u64);
    // step 2, 4
    res[3] = car1.wrapping_add(car2);

    // step 3, 0
    let tmp = a30.wrapping_add(res[0] as u128);
    (res[0], car2) = (tmp as u64, (tmp >> 64) as u64);
    let m = res[0].wrapping_mul(mu0);
    let m0 = (m as u128).wrapping_mul(p[0] as u128);
    let m1 = (m as u128).wrapping_mul(p[1] as u128);
    let m2 = (m as u128).wrapping_mul(p[2] as u128);
    let m3 = (m as u128).wrapping_mul(p[3] as u128);
    // carry_1 = ((m0  >> 64) as u64).wrapping_add(1); // doesn't work when digits are 0 or powers of 2 etc.
    let (_, tmp) = res[0].overflowing_add(m0 as u64);
    car1 = ((m0  >> 64) as u64).wrapping_add(tmp as u64);    
    // step 3, 1
    let tmp = a31.wrapping_add(res[1] as u128).wrapping_add(car2 as u128);
    (res[1], car2) = (tmp as u64, (tmp >> 64) as u64);
    let tmp = m1.wrapping_add(res[1] as u128).wrapping_add(car1 as u128);
    (res[0], car1) = (tmp as u64, (tmp >> 64) as u64);
    // step 3, 2
    let tmp = a32.wrapping_add(res[2] as u128).wrapping_add(car2 as u128);
    (res[2], car2) = (tmp as u64, (tmp >> 64) as u64);
    let tmp = m2.wrapping_add(res[2] as u128).wrapping_add(car1 as u128);
    (res[1], car1) = (tmp as u64, (tmp >> 64) as u64);
    // step 3, 3
    // let tmp = a33.wrapping_add((res[3]).wrapping_add(carry_2) as u128); // only works for input < p
    let tmp = a33.wrapping_add(res[3] as u128).wrapping_add(car2 as u128);
    (res[3], car2) = (tmp as u64, (tmp >> 64) as u64);
    let tmp = m3.wrapping_add(res[3] as u128).wrapping_add(car1 as u128);
    (res[2], car1) = (tmp as u64, (tmp >> 64) as u64);
    // step 3, 3
    res[3] = car1.wrapping_add(car2);

    res
}
