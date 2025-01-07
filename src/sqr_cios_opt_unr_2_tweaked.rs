use crate::constants::*;


#[inline]
pub fn sqr_cios_opt_unr_2_tweaked(a: [u64; 4]) -> [u64; 4] {
    let p = U64_P;
    let mu0 = U64_MU0;

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

    // step 0, 0 - pre
    let res0 = a00 as u64;
    let car2 = (a00 >> 64) as u64;
    // step 0, 1 - pre
    // let tmp = a01.wrapping_add(car2 as u128); // moved this to step 1, 0
    let tmp = car2 as u128;
    let res1 = tmp as u64;
    let car2 = (tmp >> 64) as u64;
    // step 0, 2 - pre
    let tmp = a02.wrapping_add(car2 as u128);
    let res2 = tmp as u64;
    let car2 = (tmp >> 64) as u64;
    // step 0, 3 - pre
    let tmp = a03.wrapping_add(car2 as u128);
    let res3 = tmp as u64;
    let car2 = (tmp >> 64) as u64;
    // step 0, 0 - post
    let m = res0.wrapping_mul(mu0);
    let m0 = (m as u128).wrapping_mul(p[0] as u128);
    let m1 = (m as u128).wrapping_mul(p[1] as u128);
    let m2 = (m as u128).wrapping_mul(p[2] as u128);
    let m3 = (m as u128).wrapping_mul(p[3] as u128);
    // let car1 = ((m0  >> 64) as u64).wrapping_add(1); // doesn't work when digits are 0 or powers of 2 etc.
    let (_, tmp) = res0.overflowing_add(m0 as u64);
    let car1 = ((m0 >>64) as u64).wrapping_add(tmp as u64);
    // step 0, 1 - post
    let tmp = m1.wrapping_add(res1 as u128).wrapping_add(car1 as u128);
    let res0 = tmp as u64;
    let car1 = (tmp >> 64) as u64;
    // step 0, 2 - post
    let tmp = m2.wrapping_add(res2 as u128).wrapping_add(car1 as u128);
    let res1 = tmp as u64;
    let car1 = (tmp >> 64) as u64;
    // step 0, 3 - post
    let tmp = m3.wrapping_add(res3 as u128).wrapping_add(car1 as u128);
    let res2 = tmp as u64;
    let car1 = (tmp >> 64) as u64;
    // step 0, 4
    let res3 = car1.wrapping_add(car2);

    // step 1, 0 - pre
    let tmp = (a01 << 1).wrapping_add(res0 as u128);
    let res0 = tmp as u64;
    let car2 = (tmp >> 64) as u64;
    // step 1, 1 - pre
    let tmp = a11.wrapping_add(res1 as u128).wrapping_add(car2 as u128 | (a01 >> 127) << 64);
    let res1 = tmp as u64;
    let car2 = (tmp >> 64) as u64;
    // step 1, 2 - pre
    // let tmp = a12.wrapping_add(res2 as u128).wrapping_add(car2 as u128); // moved this to step 2, 1
    let tmp = (res2 as u128).wrapping_add(car2 as u128);
    let res2 = tmp as u64;
    let car2 = (tmp >> 64) as u64;
    // step 1, 3 - pre
    let tmp = a13.wrapping_add(res3 as u128).wrapping_add(car2 as u128);
    let res3 = tmp as u64;
    let car2 = (tmp >> 64) as u64;
    // step 1, 0 - post
    let m = res0.wrapping_mul(mu0);
    let m0 = (m as u128).wrapping_mul(p[0] as u128);
    let m1 = (m as u128).wrapping_mul(p[1] as u128);
    let m2 = (m as u128).wrapping_mul(p[2] as u128);
    let m3 = (m as u128).wrapping_mul(p[3] as u128);
    // let car1 = ((m0  >> 64) as u64).wrapping_add(1); // doesn't work when digits are 0 or powers of 2 etc.
    let (_, tmp) = res0.overflowing_add(m0 as u64);
    let car1 = ((m0 >>64) as u64).wrapping_add(tmp as u64);
    // step 1, 1 - post
    let tmp = m1.wrapping_add(res1 as u128).wrapping_add(car1 as u128);
    let res0 = tmp as u64;
    let car1 = (tmp >> 64) as u64;
    // step 1, 2 - post
    let tmp = m2.wrapping_add(res2 as u128).wrapping_add(car1 as u128);
    let res1 = tmp as u64;
    let car1 = (tmp >> 64) as u64;
    // step 1, 3 - post
    let tmp = m3.wrapping_add(res3 as u128).wrapping_add(car1 as u128);
    let res2 = tmp as u64;
    let car1 = (tmp >> 64) as u64;
    // step 1, 4
    let res3 = car1.wrapping_add(car2);

    // step 2, 0 - pre
    let tmp = a02.wrapping_add(res0 as u128);
    let res0 = tmp as u64;
    let car2 = (tmp >> 64) as u64;
    // step 2, 1 - pre
    let tmp = (a12 << 1).wrapping_add(res1 as u128).wrapping_add(car2 as u128);
    let res1 = tmp as u64;
    let car2 = (tmp >> 64) as u64;
    // step 2, 2 - pre
    let tmp = a22.wrapping_add(res2 as u128).wrapping_add(car2 as u128 | (a12 >> 127) << 64);
    let res2 = tmp as u64;
    let car2 = (tmp >> 64) as u64;
    // step 2, 3 - pre
    let tmp = a23.wrapping_add(res3 as u128).wrapping_add(car2 as u128);
    let res3 = tmp as u64;
    let car2 = (tmp >> 64) as u64;
    // step 2, 0 - post
    let m = res0.wrapping_mul(mu0);
    let m0 = (m as u128).wrapping_mul(p[0] as u128);
    let m1 = (m as u128).wrapping_mul(p[1] as u128);
    let m2 = (m as u128).wrapping_mul(p[2] as u128);
    let m3 = (m as u128).wrapping_mul(p[3] as u128);
    // let car1 = ((m0  >> 64) as u64).wrapping_add(1); // doesn't work when digits are 0 or powers of 2 etc.
    let (_, tmp) = res0.overflowing_add(m0 as u64);
    let car1 = ((m0 >>64) as u64).wrapping_add(tmp as u64);
    // step 2, 1 - post
    let tmp = m1.wrapping_add(res1 as u128).wrapping_add(car1 as u128);
    let res0 = tmp as u64;
    let car1 = (tmp >> 64) as u64;
    // step 2, 2 - post
    let tmp = m2.wrapping_add(res2 as u128).wrapping_add(car1 as u128);
    let res1 = tmp as u64;
    let car1 = (tmp >> 64) as u64;
    // step 2, 3 - post
    let tmp = m3.wrapping_add(res3 as u128).wrapping_add(car1 as u128);
    let res2 = tmp as u64;
    let car1 = (tmp >> 64) as u64;
    // step 2, 4
    let res3 = car1.wrapping_add(car2);

    // step 3, 0 - pre
    let tmp = a03.wrapping_add(res0 as u128);
    let res0 = tmp as u64;
    let car2 = (tmp >> 64) as u64;
    // step 3, 1 - pre
    let tmp = a13.wrapping_add(res1 as u128).wrapping_add(car2 as u128);
    let res1 = tmp as u64;
    let car2 = (tmp >> 64) as u64;
    // step 3, 2 - pre
    let tmp = a23.wrapping_add(res2 as u128).wrapping_add(car2 as u128);
    let res2 = tmp as u64;
    let car2 = (tmp >> 64) as u64;
    // step 3, 3 - pre
    // let tmp = a33.wrapping_add(res3.wrapping_add(car2) as u128); // only works for input < p
    let tmp = a33.wrapping_add(res3 as u128).wrapping_add(car2 as u128);
    let res3 = tmp as u64;
    let car2 = (tmp >> 64) as u64;
    // step 3, 0 - post
    let m = res0.wrapping_mul(mu0);
    let m0 = (m as u128).wrapping_mul(p[0] as u128);
    let m1 = (m as u128).wrapping_mul(p[1] as u128);
    let m2 = (m as u128).wrapping_mul(p[2] as u128);
    let m3 = (m as u128).wrapping_mul(p[3] as u128);
    // let car1 = ((m0  >> 64) as u64).wrapping_add(1); // doesn't work when digits are 0 or powers of 2 etc.
    let (_, tmp) = res0.overflowing_add(m0 as u64);
    let car1 = ((m0 >>64) as u64).wrapping_add(tmp as u64);
    // step 3, 1 - post
    let tmp = m1.wrapping_add(res1 as u128).wrapping_add(car1 as u128);
    let res0 = tmp as u64;
    let car1 = (tmp >> 64) as u64;
    // step 3, 2 - post
    let tmp = m2.wrapping_add(res2 as u128).wrapping_add(car1 as u128);
    let res1 = tmp as u64;
    let car1 = (tmp >> 64) as u64;
    // step 3, 3 - post
    let tmp = m3.wrapping_add(res3 as u128).wrapping_add(car1 as u128);
    let res2 = tmp as u64;
    let car1 = (tmp >> 64) as u64;
    // step 3, 3
    let res3 = car1.wrapping_add(car2);

    [res0, res1, res2, res3]
}
