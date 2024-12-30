#![feature(bigint_helper_methods)]
const U64_P: [u64; 4] = [4891460686036598785, 2896914383306846353, 13281191951274694749, 3486998266802970665];
const U64_MU0: u64 = 14042775128853446655;
const U128_INVR: [u128; 2] = [12040726767651885946275568079130925390, 29139034451183002340352571843769032029];


#[inline]
pub fn sqr_cios_opt_unr_2(a: [u64; 4]) -> [u64; 4] {
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
    let m0 = ((m as u128).wrapping_mul(p[0] as u128) >> 64) as u64;
    let m1 = (m as u128).wrapping_mul(p[1] as u128);
    let m2 = (m as u128).wrapping_mul(p[2] as u128);
    let m3 = (m as u128).wrapping_mul(p[3] as u128);
    let car1 = m0.wrapping_add(1);
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
    let m0 = ((m as u128).wrapping_mul(p[0] as u128) >> 64) as u64;
    let m1 = (m as u128).wrapping_mul(p[1] as u128);
    let m2 = (m as u128).wrapping_mul(p[2] as u128);
    let m3 = (m as u128).wrapping_mul(p[3] as u128);
    let car1 = m0.wrapping_add(1);
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
    let m0 = ((m as u128).wrapping_mul(p[0] as u128) >> 64) as u64;
    let m1 = (m as u128).wrapping_mul(p[1] as u128);
    let m2 = (m as u128).wrapping_mul(p[2] as u128);
    let m3 = (m as u128).wrapping_mul(p[3] as u128);
    let car1 = m0.wrapping_add(1);
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
    let tmp = (res3).wrapping_add(car2);
    let tmp = a33.wrapping_add(tmp as u128);
    let res3 = tmp as u64;
    let car2 = (tmp >> 64) as u64;
    // step 3, 0 - post
    let m = res0.wrapping_mul(mu0);
    let m0 = ((m as u128).wrapping_mul(p[0] as u128) >> 64) as u64;
    let m1 = (m as u128).wrapping_mul(p[1] as u128);
    let m2 = (m as u128).wrapping_mul(p[2] as u128);
    let m3 = (m as u128).wrapping_mul(p[3] as u128);
    let car1 = m0.wrapping_add(1);
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


#[inline]
pub fn sqr_cios_opt_unr_1(a: [u64; 4]) -> [u64; 4] {
    let p = U64_P;
    let mu0 = U64_MU0;
    let mut res = [0u64; 4];
    let mut carry_1: u64;
    let mut carry_2: u64;

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
    (res[0], carry_2) = (tmp, (a00 >> 64) as u64);
    let m = res[0].wrapping_mul(mu0);
    let m0 = ((m as u128).wrapping_mul(p[0] as u128) >> 64) as u64;
    let m1 = (m as u128).wrapping_mul(p[1] as u128);
    let m2 = (m as u128).wrapping_mul(p[2] as u128);
    let m3 = (m as u128).wrapping_mul(p[3] as u128);
    carry_1 = m0.wrapping_add(1);
    // step 0, 1
    let tmp = a01.wrapping_add((res[1]).wrapping_add(carry_2) as u128);
    (res[1], carry_2) = (tmp as u64, (tmp >> 64) as u64);
    let tmp = m1.wrapping_add(res[1] as u128).wrapping_add(carry_1 as u128);
    (res[0], carry_1) = (tmp as u64, (tmp >> 64) as u64);
    // step 0, 2
    let tmp = a02.wrapping_add((res[2]).wrapping_add(carry_2) as u128);
    (res[2], carry_2) = (tmp as u64, (tmp >> 64) as u64);
    let tmp = m2.wrapping_add(res[2] as u128).wrapping_add(carry_1 as u128);
    (res[1], carry_1) = (tmp as u64, (tmp >> 64) as u64);
    // step 0, 3
    let tmp = a03.wrapping_add((res[3]).wrapping_add(carry_2) as u128);
    (res[3], carry_2) = (tmp as u64, (tmp >> 64) as u64);
    let tmp = m3.wrapping_add(res[3] as u128).wrapping_add(carry_1 as u128);
    (res[2], carry_1) = (tmp as u64, (tmp >> 64) as u64);
    // step 0, 4
    res[3] = carry_1.wrapping_add(carry_2);

    // step 1, 0
    let tmp = a10.wrapping_add(res[0] as u128);
    (res[0], carry_2) = (tmp as u64, (tmp >> 64) as u64);
    let m = res[0].wrapping_mul(mu0);
    let m0 = ((m as u128).wrapping_mul(p[0] as u128) >> 64) as u64;
    let m1 = (m as u128).wrapping_mul(p[1] as u128);
    let m2 = (m as u128).wrapping_mul(p[2] as u128);
    let m3 = (m as u128).wrapping_mul(p[3] as u128);
    carry_1 = m0.wrapping_add(1);
    // step 1, 1
    let tmp = (res[1] as u128).wrapping_add(carry_2 as u128).wrapping_add(a11);
    (res[1], carry_2) = (tmp as u64, (tmp >> 64) as u64);
    let tmp = m1.wrapping_add(res[1] as u128).wrapping_add(carry_1 as u128);
    (res[0], carry_1) = (tmp as u64, (tmp >> 64) as u64);
    // step 1, 2
    let tmp = a12.wrapping_add((res[2] as u128).wrapping_add(carry_2 as u128));
    (res[2], carry_2) = (tmp as u64, (tmp >> 64) as u64);
    let tmp = m2.wrapping_add(res[2] as u128).wrapping_add(carry_1 as u128);
    (res[1], carry_1) = (tmp as u64, (tmp >> 64) as u64);
    // step 1, 3
    let tmp = a13.wrapping_add((res[3] as u128).wrapping_add(carry_2 as u128));
    (res[3], carry_2) = (tmp as u64, (tmp >> 64) as u64);
    let tmp = m3.wrapping_add(res[3] as u128).wrapping_add(carry_1 as u128);
    (res[2], carry_1) = (tmp as u64, (tmp >> 64) as u64);
    // step 1, 4
    res[3] = carry_1.wrapping_add(carry_2);

    // step 2, 0
    let tmp = a20.wrapping_add(res[0] as u128);
    (res[0], carry_2) = (tmp as u64, (tmp >> 64) as u64);
    let m = res[0].wrapping_mul(mu0);
    let m0 = ((m as u128).wrapping_mul(p[0] as u128) >> 64) as u64;
    let m1 = (m as u128).wrapping_mul(p[1] as u128);
    let m2 = (m as u128).wrapping_mul(p[2] as u128);
    let m3 = (m as u128).wrapping_mul(p[3] as u128);
    carry_1 = m0.wrapping_add(1);
    // step 2, 1
    let tmp = a21.wrapping_add((res[1] as u128).wrapping_add(carry_2 as u128));
    (res[1], carry_2) = (tmp as u64, (tmp >> 64) as u64);
    let tmp = m1.wrapping_add(res[1] as u128).wrapping_add(carry_1 as u128);
    (res[0], carry_1) = (tmp as u64, (tmp >> 64) as u64);
    // step 2, 2
    let tmp = a22.wrapping_add((res[2] as u128).wrapping_add(carry_2 as u128));
    (res[2], carry_2) = (tmp as u64, (tmp >> 64) as u64);
    let tmp = m2.wrapping_add(res[2] as u128).wrapping_add(carry_1 as u128);
    (res[1], carry_1) = (tmp as u64, (tmp >> 64) as u64);
    // step 2, 3
    let tmp = a23.wrapping_add((res[3] as u128).wrapping_add(carry_2 as u128));
    (res[3], carry_2) = (tmp as u64, (tmp >> 64) as u64);
    let tmp = m3.wrapping_add(res[3] as u128).wrapping_add(carry_1 as u128);
    (res[2], carry_1) = (tmp as u64, (tmp >> 64) as u64);
    // step 2, 4
    res[3] = carry_1.wrapping_add(carry_2);

    // step 3, 0
    let tmp = a30.wrapping_add(res[0] as u128);
    (res[0], carry_2) = (tmp as u64, (tmp >> 64) as u64);
    let m = res[0].wrapping_mul(mu0);
    let m0 = ((m as u128).wrapping_mul(p[0] as u128) >> 64) as u64;
    let m1 = (m as u128).wrapping_mul(p[1] as u128);
    let m2 = (m as u128).wrapping_mul(p[2] as u128);
    let m3 = (m as u128).wrapping_mul(p[3] as u128);
    carry_1 = m0.wrapping_add(1);
    // step 3, 1
    let tmp = a31.wrapping_add((res[1] as u128).wrapping_add(carry_2 as u128));
    (res[1], carry_2) = (tmp as u64, (tmp >> 64) as u64);
    let tmp = m1.wrapping_add(res[1] as u128).wrapping_add(carry_1 as u128);
    (res[0], carry_1) = (tmp as u64, (tmp >> 64) as u64);
    // step 3, 2
    let tmp = a32.wrapping_add((res[2] as u128).wrapping_add(carry_2 as u128));
    (res[2], carry_2) = (tmp as u64, (tmp >> 64) as u64);
    let tmp = m2.wrapping_add(res[2] as u128).wrapping_add(carry_1 as u128);
    (res[1], carry_1) = (tmp as u64, (tmp >> 64) as u64);
    // step 3, 3
    let tmp = a33.wrapping_add((res[3]).wrapping_add(carry_2) as u128);
    (res[3], carry_2) = (tmp as u64, (tmp >> 64) as u64);
    let tmp = m3.wrapping_add(res[3] as u128).wrapping_add(carry_1 as u128);
    (res[2], carry_1) = (tmp as u64, (tmp >> 64) as u64);
    // step 3, 3
    res[3] = carry_1.wrapping_add(carry_2);

    res
}


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


#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test() {
        assert_eq!(sqr_cios_opt_unr_2([7043351708507219244, 12973988534611502911, 2286018893667355461, 3390003277958482388]), [939962812878190028, 8706293248576803058, 16205989205945120409, 3883740201024575067]); 
        for _ in 0..1000000 {
            let input: [u64; 4] = [
                rand::random::<u64>(),
                rand::random::<u64>(),
                rand::random::<u64>(),
                rand::random::<u64>(),
            ];
            println!("{:?}", input);
            assert_eq!(sqr_cios_opt_unr_2(input), sqr_cios_opt_unr_1(input));    
        }

        // let input: [u64; 4] = [20074282081074489, 415449875894500508, 9499724717238044174, 12059022386397940636];
        // assert_eq!(sqr_cios_opt_asm(input), sqr_cios_opt_unr(input)); 
    }
}